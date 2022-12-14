use ts_cursor::cursor::*;
use ts_cursor::traverser::*;

pub enum ResKind {
    Root,
    Function,
    Class,
}

pub struct Resolved<'a> {
    pub cursor: Cursor<'a>,
    pub kind: ResKind,
}

impl<'a> Resolved<'a> {
    pub fn new(cursor: Cursor<'a>, kind: ResKind) -> Self {
        Self { cursor, kind }
    }

    /// returns vec of resolved parameter names
    /// empty if not function variant
    pub fn parameters(&self) -> Vec<Cursor<'a>> {
        match &self.kind {
            ResKind::Function => {
                let mut cursor = self.cursor.clone();
                if cursor.goto_field("parameters") {
                    Traversal::from_cursor(&cursor, STKind::Abstract)
                        .filter_map(|mot| match mot {
                            Order::Enter(cur) => Some(cur),
                            _ => None,
                        })
                        .filter(|cur| cur.kind() == "variable_name")
                        .collect()
                } else {
                    vec![]
                }
            }
            // return empty if not function
            _ => vec![],
        }
    }

    /// returns vec of resolved property names
    /// empty if not class variant
    pub fn properties(&self) -> Vec<Cursor<'a>> {
        match &self.kind {
            ResKind::Class => {
                let mut cursor = self.cursor.clone();
                if cursor.goto_field("body") {
                    Traversal::from_cursor(&cursor, STKind::Abstract)
                        .filter_map(|mot| match mot {
                            Order::Enter(cur) => Some(cur),
                            _ => None,
                        })
                        .filter(|cur| cur.kind() == "property_declaration")
                        .collect()
                } else {
                    vec![]
                }
            }
            // return empty if not class
            _ => vec![],
        }
    }
}
