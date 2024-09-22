use codenav;
use napi_derive::napi;

#[napi]
#[derive(Debug)]
pub enum Language {
    Python = 0,
    JavaScript = 1,
    TypeScript = 2,
}

impl From<codenav::Language> for Language {
    fn from(language: codenav::Language) -> Self {
        match language {
            codenav::Language::Python => Language::Python,
            codenav::Language::JavaScript => Language::JavaScript,
            codenav::Language::TypeScript => Language::TypeScript,
            _ => panic!("Unsupport language: {:?}", language),
        }
    }
}

impl Language {
    // TODO: find a more idiomatic way?
    fn to(&self) -> codenav::Language {
        match self {
            Language::Python => codenav::Language::Python,
            Language::JavaScript => codenav::Language::JavaScript,
            Language::TypeScript => codenav::Language::TypeScript,
            _ => panic!("Unsupport language: {:?}", self),
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct Point {
    pub line: u32,
    pub column: u32,
}

impl From<codenav::Point> for Point {
    fn from(p: codenav::Point) -> Self {
        Self {
            line: p.line as u32,
            column: p.column as u32,
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct Span {
    pub start: Point,
    pub end: Point,
}

impl From<codenav::Span> for Span {
    fn from(s: codenav::Span) -> Self {
        Self {
            start: Point::from(s.start),
            end: Point::from(s.end),
        }
    }
}

#[napi]
pub enum TextMode {
    Overview,
    Complete,
}

#[napi]
#[derive(Clone)]
pub struct Definition {
    pub language: Language,
    pub path: String,
    pub span: Span,
}

#[napi]
impl Definition {
    #[napi]
    pub fn text(&self, mode: TextMode) -> napi::Result<String> {
        let d = codenav::Definition {
            language: self.language.to(),
            path: self.path.clone(),
            span: codenav::Span {
                start: codenav::Point {
                    line: self.span.start.line as usize,
                    column: self.span.start.column as usize,
                },
                end: codenav::Point {
                    line: self.span.end.line as usize,
                    column: self.span.end.column as usize,
                },
            },
        };
        let m = match mode {
            TextMode::Overview => codenav::TextMode::Overview,
            TextMode::Complete => codenav::TextMode::Complete,
        };
        Ok(d.text(m))
    }

}

impl From<codenav::Definition> for Definition {
    fn from(d: codenav::Definition) -> Self {
        Self {
            language: Language::from(d.language),
            path: d.path,
            span: Span::from(d.span),
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct Reference {
    /// File path
    pub path: String,
    /// Position line (0-based)
    pub line: u32,
    /// Position column (0-based grapheme)
    pub column: u32,
    /// The text string
    pub text: String,
}

impl From<codenav::Reference> for Reference {
    fn from(r: codenav::Reference) -> Self {
        Self {
            path: r.path,
            line: r.line as u32,
            column: r.column as u32,
            text: r.text,
        }
    }
}

#[napi]
pub struct Navigator {
    nav: codenav::Navigator,
}

#[napi]
impl Navigator {
    // Example:
    //
    // ```javascript
    // import * as codenav from '@codenav/codenav'
    // let nav = new Navigator(codenav.Language.Python, './test.sqlite');
    // ```
    #[napi(constructor)]
    pub fn new(language: Language, db_path: String) -> Self {
        Self {
            nav: codenav::Navigator::new(language.to(), db_path),
        }
    }

    #[napi]
    pub fn index(&self, source_paths: Vec<String>, force: bool) -> napi::Result<()> {
        self.nav.index(source_paths, force);
        Ok(())
    }

    #[napi]
    pub fn clean(&self, delete: bool) -> napi::Result<()> {
        self.nav.clean(delete);
        Ok(())
    }

    #[napi]
    pub fn resolve(&mut self, reference: Reference) -> napi::Result<Vec<Definition>> {
        let definitions = self.nav.resolve(codenav::Reference {
            path: reference.path,
            line: reference.line as usize,
            column: reference.column as usize,
            text: reference.text,
        });
        let py_definitions = definitions
            .into_iter()
            .map(Definition::from)
            .collect::<Vec<_>>();
        Ok(py_definitions)
    }
}


#[napi]
pub struct Snippet {
    s: codenav::Snippet,
}

#[napi]
impl Snippet {
    // Example:
    //
    // ```javascript
    // import { Sinppet } from './index.js'
    // let s = new Snippet("test.py", 0, 11);
    // ```
    #[napi(constructor)]
    pub fn new(language: Language, path: String, line_start: u32, line_end: u32) -> Self {
        Self {
            s: codenav::Snippet::new(language.to(), path, line_start as usize, line_end as usize),
        }
    }

    #[napi]
    pub fn references(&self, query_path: String) -> napi::Result<Vec<Reference>> {
        let references = self.s.references(query_path);
        let py_references = references
            .into_iter()
            .map(Reference::from)
            .collect::<Vec<_>>();
        Ok(py_references)
    }

    #[napi]
    pub fn contains(&self, d: &Definition) -> napi::Result<bool> {
        let contained = self.s.contains(codenav::Definition {
            language: d.language.to(),
            path: d.path.clone(),
            span: codenav::Span {
                start: codenav::Point {
                    line: d.span.start.line as usize,
                    column: d.span.start.column as usize,
                },
                end: codenav::Point {
                    line: d.span.end.line as usize,
                    column: d.span.end.column as usize,
                },
            },
        });
        Ok(contained)
    }
}