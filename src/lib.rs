pub mod crawler;
pub mod repository;
pub mod resolved;

#[cfg(test)]
mod tests {
    use super::repository::*;
    use crate::crawler::crawl_svn_php;
    use ts_cursor::file::File;

    #[tokio::test]
    async fn downloader() {
        let files = crawl_svn_php("http://plugins.svn.wordpress.org/qiwi-button/trunk/")
            .await
            .unwrap();

        println!("{:?}", files.len());

        let repo = Repository::from_files(&files, Language::PHP);

        assert_eq!(files.len(), 3);
        assert_eq!(repo.resolved().len(), 8);
    }

    #[test]
    fn multifile() {
        let file1 = File::new("./test_php/test_graph.php", tree_sitter_php::language()).unwrap();
        let file2 =
            File::new("./test_php/test_multifile.php", tree_sitter_php::language()).unwrap();
        let files = vec![file1, file2];
        let repo = Repository::from_files(&files, Language::PHP);
        let mut resolved_str = repo
            .resolved()
            .iter()
            .map(|r| r.0.to_owned())
            .collect::<Vec<String>>();
        resolved_str.sort();
        assert_eq!(
            format!("{:?}", resolved_str),
            "[\"ROOT\", \"test\", \"test1\", \"test2\"]"
        );
    }

    #[test]
    fn params() {
        let file1 = File::new("./test_php/test_graph.php", tree_sitter_php::language()).unwrap();
        let file2 =
            File::new("./test_php/test_multifile.php", tree_sitter_php::language()).unwrap();
        let files = vec![file1, file2];
        let repo = Repository::from_files(&files, Language::PHP);

        let resolved = repo.resolved();
        let fun = &resolved["test2"];
        let params = fun.parameters();

        // assert that the params of test2 are p1 and p2
        assert_eq!(
            format!(
                "{:?}",
                params
                    .iter()
                    .map(|p| p.name(true).unwrap())
                    .collect::<Vec<String>>()
            ),
            "[\"p1\", \"p2\"]"
        );
    }
}
