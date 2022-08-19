use super::resolved::*;
use ts_cursor::file::*;

pub enum Language {
    PHP,
    JavaScript,
}

pub struct Repository<'a> {
    files: Vec<File>,
    language: Language,
    resolved: Vec<Resolved<'a>>,
}

impl<'a> Repository<'a> {
    pub fn from_files(files: Vec<File>, language: Language) -> Self {
        Self {
            files,
            language,
            resolved: vec![],
        }
    }

    pub fn resolve(&mut self) {}
}
