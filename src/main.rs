use std::{
    env,
    fs::{self, DirEntry},
    path::Path,
    path::PathBuf,
};

fn main() -> Result<(), anyhow::Error> {
    // 現在のパスを取得
    let path: PathBuf = env::current_dir()?;

    // 出力例の通り、Current Pathと空行を出力
    println!("Current Path: {}\n", path.display());

    // ルートとなるディレクトリ名を出力
    let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
    println!("{}", dir_name);

    // .gitignore を読み込んで除外リストを作成する
    let ignores = load_ignores(&path);

    // ツリーの描画を再帰的に開始
    print_tree(&path, "", &ignores)?;

    Ok(())
}

/// `.gitignore` を読み込んで簡易的な除外リストを生成する
fn load_ignores(path: &Path) -> Vec<String> {
    // .git はデフォルトで中身を展開しない
    let mut ignores = vec![".git".to_string()];

    let gitignore_path = path.join(".gitignore");
    if let Ok(content) = fs::read_to_string(gitignore_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            // 空行やコメント行はスキップ
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // 簡易的に先頭・末尾のスラッシュを取り除く (例: "/target" -> "target")
            let clean_name = trimmed.trim_matches('/');
            ignores.push(clean_name.to_string());
        }
    }
    ignores
}

/// 再帰的にディレクトリを探索し、ツリー構造で出力する
fn print_tree(dir_path: &Path, prefix: &str, ignores: &[String]) -> Result<(), anyhow::Error> {
    // フォルダ内の要素を取得
    let mut entries: Vec<DirEntry> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();

    // 出力順を安定させるため、ファイル名でアルファベット順にソート
    entries.sort_by_key(|e| e.file_name());

    let count = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        // 現在の要素が、フォルダ内で最後の要素かどうか
        let is_last = i == count - 1;

        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy().into_owned();

        // 枝の記号を決定
        let branch = if is_last { "└─ " } else { "├─ " };

        // ファイル/フォルダの出力
        println!("{}{}{}", prefix, branch, name_str);

        let file_type = entry.file_type()?;
        // フォルダの場合は再帰的に中身を探索する
        if file_type.is_dir() {
            // 除外リスト (ignores) に含まれている場合は、中身を探索しない
            if !ignores.contains(&name_str) {
                // 子要素へ渡す接頭辞(プレフィックス)の決定
                // 最後なら空白を渡し、途中の枝なら縦線(│)を渡して階層の線を下へ繋ぐ
                let next_prefix = if is_last {
                    format!("{}  ", prefix)
                } else {
                    format!("{}│  ", prefix)
                };

                // 再帰呼び出し
                print_tree(&entry.path(), &next_prefix, ignores)?;
            }
        }
    }

    Ok(())
}
