use anyhow::Result;
use gar_crawl::*;
use std::collections::HashMap;
use ts_cursor::file::*;

pub async fn crawl_svn_php(url: &str) -> Result<Vec<File>> {
    let raw_files = crawl(
        url,
        100,
        vec![".php".into(), ".phtml".into(), ".html".into()],
    )
    .await?;
    let ts_lang = tree_sitter_php::language();

    let files = raw_files
        .iter()
        .flat_map(|(name, text)| File::from_string(name, text.to_owned(), ts_lang))
        .collect();

    Ok(files)
}

async fn crawl(url: &str, depth: usize, filetypes: Vec<String>) -> Result<HashMap<String, String>> {
    let mut files = HashMap::new();

    Crawler::builder()
        .add_default_propagators()
        .whitelist(url)
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36".into())
        .on_page(|args| {
            filetypes.iter()
                .filter(|&expr| args.page.url.to_string().ends_with(expr))
                .take(1)
                .for_each(|_| {
                    files.insert(args.page.url.to_string(), args.page.text.clone());
                });
        })
        .depth(depth)
        .build()?
        .crawl(url)
        .await?;

    Ok(files)
}
