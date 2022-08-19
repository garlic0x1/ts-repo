use std::collections::HashMap;

use super::resolved::*;
use ts_cursor::{cursor::*, file::*, traverser::*};

#[derive(Copy, Clone)]
pub enum Language {
    PHP,
    JavaScript,
}

pub struct Repository<'a> {
    files: Vec<File>,
    language: Language,
    resolved: HashMap<String, Resolved<'a>>,
}

impl<'a> Repository<'a> {
    pub fn from_files(files: &'a Vec<File>, language: Language) -> Self {
        let mut s = Self {
            files: files.to_owned(),
            language,
            resolved: HashMap::new(),
        };

        files
            .iter()
            .map(|f| f.cursor(STKind::Abstract))
            .for_each(|cur| {
                s.resolve(cur);
            });

        s
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn add_file(&mut self, file: &'a File) {
        self.resolve(file.cursor(STKind::Abstract));
        self.files.push(file.to_owned());
    }

    pub fn resolved(&self) -> &HashMap<String, Resolved<'a>> {
        &self.resolved
    }

    pub fn language(&self) -> Language {
        self.language
    }

    fn resolve(&mut self, cursor: Cursor<'a>) {
        Traversal::from_cursor(&cursor, STKind::Abstract)
            .filter_map(|mot| match mot {
                Order::Enter(cur) => Some(cur),
                _ => None,
            })
            .for_each(|cur| match cur.kind() {
                "function_definition" | "method_declaration" => {
                    if let Some(n) = cur.name(true) {
                        let res = Resolved::new(cur, ResKind::Function);
                        self.resolved.insert(n.to_owned(), res);
                    }
                }
                "program" => {
                    let res = Resolved::new(cur, ResKind::Root);
                    self.resolved.insert("ROOT".to_owned(), res);
                }
                _ => (),
            });
    }
}
