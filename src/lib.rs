use stack_graphs::storage::{SQLiteReader, SQLiteWriter};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use string_builder::Builder;
use tree_sitter::{Node, Parser, Query, QueryCursor, TreeCursor};
use tree_sitter_stack_graphs::cli::index::Indexer;
use tree_sitter_stack_graphs::cli::load::LanguageConfigurationsLoaderArgs;
use tree_sitter_stack_graphs::cli::query::{Querier, QueryResult};
use tree_sitter_stack_graphs::cli::util::reporter::{ConsoleReporter, Level};
use tree_sitter_stack_graphs::cli::util::SourcePosition;
use tree_sitter_stack_graphs::{CancellationFlag, NoCancellation};

/// The tree-sitter references query source for Python language.
pub const PYTHON_REFERENCES_QUERY_SOURCE: &str = include_str!("python-references.scm");

/// The tree-sitter references query source for JavaScript language.
pub const JAVASCRIPT_REFERENCES_QUERY_SOURCE: &str = include_str!("javascript-references.scm");

/// The tree-sitter references query source for TypeScript language.
pub const TYPESCRIPT_REFERENCES_QUERY_SOURCE: &str = include_str!("typescript-references.scm");

#[derive(Debug, Clone)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
}

pub enum TextMode {
    Overview,
    Complete,
}

#[derive(Debug)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

pub struct Span {
    pub start: Point,
    pub end: Point,
}

pub struct Definition {
    /// Programming language.
    pub language: Language,
    /// File path
    pub path: String,
    /// Span
    pub span: Span,
}

pub struct Capture {
    pub name: String,
    pub text: String,
    pub start: Point,
    pub end: Point,
}

#[derive(Clone)]
pub struct Reference {
    /// File path
    pub path: String,
    /// Position line (0-based)
    pub line: usize,
    /// Position column (0-based grapheme)
    pub column: usize,
    /// The text string
    pub text: String,
}

pub struct ParseResult {
    pub capture: Capture,
    pub definitions: Vec<Definition>,
}

pub struct Navigator {
    language: Language,
    db_path: PathBuf,
    verbose: bool,
    hide_error_details: bool,
}

impl Navigator {
    pub fn new(language: Language, db_path: String) -> Self {
        Self {
            db_path: PathBuf::from(db_path),
            language: language,
            verbose: false,
            hide_error_details: false,
        }
    }

    pub fn index(&self, source_paths: Vec<String>, force: bool) -> anyhow::Result<()> {
        // Only handle Python, JavaScript and TypeScript for now.
        let lc = match self.language {
            Language::Python => {
                tree_sitter_stack_graphs_python::try_language_configuration(&NoCancellation)
                    .unwrap()
            }
            Language::JavaScript => {
                tree_sitter_stack_graphs_javascript::try_language_configuration(&NoCancellation)
                    .unwrap()
            }
            Language::TypeScript => {
                tree_sitter_stack_graphs_typescript::try_language_configuration_typescript(
                    &NoCancellation,
                )
                .unwrap()
            }
            _ => panic!("Unsupport language: {:?}", self.language),
        };

        let configurations = vec![lc];
        let load_args = LanguageConfigurationsLoaderArgs::new();
        let mut loader = load_args.get(configurations)?;

        let mut db = SQLiteWriter::open(&self.db_path)?;
        let reporter = self.get_reporter();
        let mut indexer = Indexer::new(&mut db, &mut loader, &reporter);
        indexer.force = force;
        indexer.set_collect_stats(false);

        let source_paths = source_paths
            .into_iter()
            .map(|p| PathBuf::from(p).canonicalize())
            .collect::<std::result::Result<Vec<_>, _>>()?;
        indexer.index_all(source_paths, Option::<PathBuf>::None, &NoCancellation)?;
        Ok(())
    }

    pub fn clean(&self, delete: bool) -> anyhow::Result<()> {
        let mut db = SQLiteWriter::open(&self.db_path)?;
        if delete {
            if !self.db_path.exists() {
                return Ok(());
            }
            std::fs::remove_file(&self.db_path)?;
        } else {
            db.clean_all()?;
        }
        Ok(())
    }

    pub fn resolve(&mut self, reference: Reference) -> Vec<Definition> {
        let mut db = SQLiteReader::open(&self.db_path).unwrap();
        let reporter = self.get_reporter();
        let mut querier = Querier::new(&mut db, &reporter);

        let cancellation_flag = NoCancellation;
        let reference = SourcePosition {
            path: PathBuf::from(reference.path),
            line: reference.line,
            column: reference.column,
        };
        let results = querier.definitions(reference, &cancellation_flag).unwrap();
        let numbered = results.len() > 1;
        let _indent = if numbered { 6 } else { 0 };
        //if numbered {
        //    println!("found {} references at position", results.len());
        //}

        let mut all_definitions: Vec<Definition> = Vec::new();
        for (
            _idx,
            QueryResult {
                source: _,
                targets: definitions,
            },
        ) in results.into_iter().enumerate()
        {
            //if numbered {
            //    println!("{:4}: queried reference", idx);
            //} else {
            //    println!("queried reference");
            //}
            //match definitions.len() {
            //    0 => println!("{}has no definitions", " ".repeat(indent)),
            //    1 => println!("{}has definition", " ".repeat(indent)),
            //    n => println!("{}has {} definitions", " ".repeat(indent), n),
            //}
            for definition in definitions.into_iter() {
                all_definitions.push(Definition {
                    language: self.language.clone(),
                    path: definition.path.display().to_string(),
                    span: Span {
                        start: Point {
                            line: definition.span.start.line,
                            column: definition.span.start.column.grapheme_offset,
                        },
                        end: Point {
                            line: definition.span.end.line,
                            column: definition.span.end.column.grapheme_offset,
                        },
                    },
                });
                //print!(
                //    "{}",
                //    Excerpt::from_source(
                //        &definition.path,
                //        file_reader.get(&definition.path).unwrap_or_default(),
                //        definition.span.start.line,
                //        first_line_column_range(&definition),
                //        indent
                //    )
                //);
            }
        }
        all_definitions
    }

    fn get_reporter(&self) -> ConsoleReporter {
        return ConsoleReporter {
            skipped_level: if self.verbose {
                Level::Summary
            } else {
                Level::None
            },
            succeeded_level: if self.verbose {
                Level::Summary
            } else {
                Level::None
            },
            failed_level: if self.hide_error_details {
                Level::Summary
            } else {
                Level::Details
            },
            canceled_level: if self.hide_error_details {
                Level::Summary
            } else {
                Level::Details
            },
        };
    }
}

pub struct Snippet {
    language: Language,
    path: String,
    line_start: usize,
    line_end: usize,
}

impl Snippet {
    pub fn new(language: Language, path: String, line_start: usize, line_end: usize) -> Self {
        Self {
            language: language,
            path: path,
            line_start: line_start,
            line_end: line_end,
        }
    }

    pub fn references(&self, query_path: String) -> Vec<Reference> {
        let file_path = PathBuf::from(&self.path);
        let query_source = if query_path.is_empty() {
            match self.language {
                Language::Python => PYTHON_REFERENCES_QUERY_SOURCE.to_string(),
                Language::JavaScript => JAVASCRIPT_REFERENCES_QUERY_SOURCE.to_string(),
                Language::TypeScript => TYPESCRIPT_REFERENCES_QUERY_SOURCE.to_string(),
                _ => panic!("Unsupport language: {:?}", self.language),
            }
        } else {
            let query_path = PathBuf::from(query_path);
            fs::read_to_string(query_path).expect("Should have been able to read the query file")
        };

        let source_code = fs::read(&file_path).expect("Should have been able to read the file");

        //println!("[SOURCE]\n\n{}\n", String::from_utf8_lossy(&source_code));
        //println!("[QUERY]\n\n{}\n", query_source);

        let mut parser = Parser::new();
        let language = match self.language {
            Language::Python => tree_sitter_python::language(),
            Language::JavaScript => tree_sitter_javascript::language(),
            Language::TypeScript => tree_sitter_typescript::language_typescript(),
            _ => panic!("Unsupport language: {:?}", self.language),
        };
        parser
            .set_language(language)
            .expect("Error loading Python parser");

        let tree = parser.parse(source_code.clone(), None).unwrap();
        let root_node = tree.root_node();

        let mut cursor = QueryCursor::new();
        let query = Query::new(language, &query_source).unwrap();
        let captures = cursor.captures(&query, root_node, source_code.as_slice());

        let mut references: Vec<Reference> = Vec::new();
        for (mat, capture_index) in captures {
            let capture = mat.captures[capture_index];
            let capture_name = &query.capture_names()[capture.index as usize];
            let pos_start = capture.node.start_position();
            let pos_end = capture.node.end_position();

            if pos_start.row >= self.line_start && pos_end.row <= self.line_end {
                //println!("[CAPTURE]\nname: {capture_name}, start: {}, end: {}, text: {:?}, capture: {:?}", pos_start, pos_end, capture.node.utf8_text(&source_code).unwrap_or(""), capture.node.to_sexp());
                let reference = Reference {
                    path: file_path.display().to_string(),
                    line: pos_start.row,
                    column: pos_start.column,
                    text: capture
                        .node
                        .utf8_text(&source_code)
                        .unwrap_or("")
                        .to_string(),
                };
                references.push(reference);
            }
        }
        references
    }

    pub fn contains(&self, d: Definition) -> bool {
        d.path == self.path
            && d.span.start.line >= self.line_start
            && d.span.end.line <= self.line_end
    }
}

impl Definition {
    pub fn text(&self, mode: TextMode) -> String {
        let file_path = PathBuf::from(&self.path);
        let source_code = fs::read(&file_path).expect("Should have been able to read the file");

        let mut parser = Parser::new();
        let language = match self.language {
            Language::Python => tree_sitter_python::language(),
            Language::JavaScript => tree_sitter_javascript::language(),
            Language::TypeScript => tree_sitter_typescript::language_typescript(),
            _ => panic!("Unsupport language: {:?}", self.language),
        };
        parser
            .set_language(language)
            .expect(format!("Error loading {:?} parser", self.language).as_str());

        let tree = parser.parse(source_code.clone(), None).unwrap();
        let root_node = tree.root_node();
        let mut cursor = tree.walk();

        let module_node_kind = match self.language {
            Language::Python => "module",
            Language::JavaScript => "program",
            Language::TypeScript => "program",
            _ => panic!("Unsupport language: {:?}", self.language),
        };

        let mut reached_root = false;
        while !reached_root {
            let node = cursor.node();
            let start_pos = node.start_position();
            //println!("kind: {:?}, start.line: {:?}, start.column: {:?}", node.kind(), start_pos.row, start_pos.column);
            if start_pos.row == self.span.start.line && node.kind() != module_node_kind {
                match mode {
                    TextMode::Complete => {
                        return node.utf8_text(&source_code).unwrap_or("").to_string();
                    }
                    TextMode::Overview => {
                        let mut lines: Vec<String> = String::from_utf8(source_code)
                            .unwrap()
                            .lines()
                            .map(|s| s.to_string())
                            .collect();
                        // Clear lines before this node.
                        for i in 0..node.start_position().row {
                            lines[i] = "".to_string();
                        }
                        // Clear lines after this node.
                        for i in node.end_position().row + 1..lines.len() {
                            lines[i] = "".to_string();
                        }
                        match self.language {
                            Language::Python => self.python_collect_node_lines(&mut lines, node),
                            Language::JavaScript => {
                                self.javascript_collect_node_lines(&mut lines, node)
                            }
                            Language::TypeScript => {
                                // TODO: define a separate typescript_collect_node_lines if there're differences.
                                //self.typescript_collect_node_lines(&mut lines, node)
                                self.javascript_collect_node_lines(&mut lines, node)
                            }
                            _ => panic!("Unsupport language: {:?}", self.language),
                        }
                        return lines
                            .into_iter()
                            .filter(|l| !l.is_empty())
                            .collect::<Vec<_>>()
                            .join("\n");
                    }
                }
            }

            if cursor.goto_first_child() {
                continue;
            }

            if cursor.goto_next_sibling() {
                continue;
            }

            let mut retracing = true;
            while retracing {
                if !cursor.goto_parent() {
                    retracing = false;
                    reached_root = true;
                }

                if cursor.goto_next_sibling() {
                    retracing = false;
                }
            }
        }

        String::from("")
    }

    pub fn python_collect_node_lines(&self, lines: &mut Vec<String>, node: Node) {
        //println!("kind: {:?}, node: {:?}", node.kind(), node);
        match node.kind() {
            "class_definition" => {
                //println!("this is a class definition");

                for i in 0..node.child_count() {
                    let n = node.child(i).unwrap();
                    //println!("kind: {:?}, child node: {:?}", n.kind(), n);

                    for j in 0..n.child_count() {
                        let nn = n.child(j).unwrap();
                        //println!("kind: {:?}, child child node: {:?}", nn.kind(), nn);
                        match nn.kind() {
                            "function_definition" => {
                                self.python_collect_node_lines(lines, nn);
                            }
                            "decorated_definition" => {
                                self.python_collect_node_lines(lines, nn);
                            }
                            _ => {}
                        }
                    }
                }
            }

            "function_definition" => {
                //println!("this is a function definition");

                for i in 0..node.child_count() {
                    let n = node.child(i).unwrap();
                    if n.kind() == "block" {
                        // Clear lines belonging to the function block.
                        let start = n.start_position();
                        let end = n.end_position();
                        for i in start.row..end.row {
                            lines[i] = "".to_string();
                        }
                        lines[end.row] = "...".to_string();
                    }
                }
            }

            "decorated_definition" => {
                //println!("this is a decorated function definition");
                for i in 0..node.child_count() {
                    let n = node.child(i).unwrap();
                    if n.kind() == "function_definition" {
                        self.python_collect_node_lines(lines, n);
                    }
                }
            }

            _ => {}
        }
    }

    pub fn javascript_collect_node_lines(&self, lines: &mut Vec<String>, node: Node) {
        match node.kind() {
            "export_statement" => {
                //println!("this is a export statement");

                for i in 0..node.child_count() {
                    let n = node.child(i).unwrap();
                    //println!("kind: {:?}, child node: {:?}", n.kind(), n);

                    match n.kind() {
                        "class_declaration" | "function_declaration" => {
                            self.javascript_collect_node_lines(lines, n);
                        }
                        _ => {}
                    }
                }
            }

            "class_declaration" => {
                //println!("this is a class declaration");

                for i in 0..node.child_count() {
                    let n = node.child(i).unwrap();
                    //println!("kind: {:?}, child node: {:?}", n.kind(), n);

                    for j in 0..n.child_count() {
                        let nn = n.child(j).unwrap();
                        //println!("kind: {:?}, child child node: {:?}", nn.kind(), nn);
                        match nn.kind() {
                            "method_definition" => {
                                self.javascript_collect_node_lines(lines, nn);
                            }
                            _ => {}
                        }
                    }
                }
            }

            "function_declaration" | "method_definition" => {
                //println!("this is a function declaration");

                for i in 0..node.child_count() {
                    let n = node.child(i).unwrap();
                    if n.kind() == "statement_block" {
                        // Clear lines belonging to the function block.
                        let start = n.start_position();
                        let end = n.end_position();
                        for i in start.row + 1..end.row {
                            // Only clear lines between the left brace and the right brace.
                            lines[i] = "".to_string();
                        }
                        if end.row - start.row > 1 {
                            // If the function block is not empty, replace the body with an ellipsis.
                            lines[end.row - 1] = "...".to_string();
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn python_snippet_references() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir).join("examples").join("python");

        let mut snippet = Snippet::new(
            Language::Python,
            examples_dir.join("chef.py").display().to_string(),
            2,
            2,
        );
        let references = snippet.references(String::from(""));
        let refs: Vec<_> = references
            .into_iter()
            .map(|r| {
                format!(
                    "{}:{}:{}=>{}",
                    Path::new(&r.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap(),
                    r.line,
                    r.column,
                    r.text,
                )
            })
            .collect();

        assert_eq!(refs, vec!["chef.py:2:0=>broil"]);
    }

    #[test]
    fn python_navigator_resolve() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir).join("examples").join("python");

        let mut nav = Navigator::new(Language::Python, String::from("./test.sqlite"));
        nav.index(vec![examples_dir.display().to_string()], true);

        let reference = Reference {
            path: examples_dir.join("chef.py").display().to_string(),
            line: 2,
            column: 0,
            text: String::from("broil"),
        };

        let definitions = nav.resolve(reference);
        let defs: Vec<_> = definitions
            .into_iter()
            .map(|d| {
                format!(
                    "{}:{}:{}",
                    Path::new(&d.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap(),
                    d.span.start.line,
                    d.span.start.column
                )
            })
            .collect();

        nav.clean(true);

        assert_eq!(defs, vec!["chef.py:0:20", "kitchen.py:2:4"]);
    }

    #[test]
    fn python_definition_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir).join("examples").join("python");

        let definition = Definition {
            language: Language::Python,
            path: examples_dir.join("stove.py").display().to_string(),
            span: Span {
                start: Point { line: 3, column: 4 },
                end: Point { line: 3, column: 8 },
            },
        };
        assert_eq!(
            definition.text(TextMode::Complete),
            "def broil():\n    print('broil')"
        );
        assert_eq!(definition.text(TextMode::Overview), "def broil():\n...");
    }

    #[test]
    fn javascript_snippet_references() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir)
            .join("examples")
            .join("javascript");

        let mut snippet = Snippet::new(
            Language::JavaScript,
            examples_dir.join("chef.js").display().to_string(),
            2,
            2,
        );
        let references = snippet.references(String::from(""));
        let refs: Vec<_> = references
            .into_iter()
            .map(|r| {
                format!(
                    "{}:{}:{}=>{}",
                    Path::new(&r.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap(),
                    r.line,
                    r.column,
                    r.text,
                )
            })
            .collect();

        assert_eq!(refs, vec!["chef.js:2:0=>broil"]);
    }

    #[test]
    fn javascript_navigator_resolve() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir)
            .join("examples")
            .join("javascript");

        let mut nav = Navigator::new(Language::JavaScript, String::from("./test.sqlite"));
        nav.index(vec![examples_dir.display().to_string()], true);

        let reference = Reference {
            path: examples_dir.join("chef.js").display().to_string(),
            line: 2,
            column: 0,
            text: String::from("broil"),
        };

        let definitions = nav.resolve(reference);
        let defs: Vec<_> = definitions
            .into_iter()
            .map(|d| {
                format!(
                    "{}:{}:{}",
                    Path::new(&d.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap(),
                    d.span.start.line,
                    d.span.start.column
                )
            })
            .collect();

        nav.clean(true);

        assert_eq!(defs, vec!["chef.js:0:9", "kitchen.js:2:16"]);
    }

    #[test]
    fn javascript_definition_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir)
            .join("examples")
            .join("javascript");

        let definition = Definition {
            language: Language::JavaScript,
            path: examples_dir.join("stove.js").display().to_string(),
            span: Span {
                start: Point {
                    line: 4,
                    column: 16,
                },
                end: Point {
                    line: 4,
                    column: 20,
                },
            },
        };
        assert_eq!(
            definition.text(TextMode::Complete),
            "export function broil() {\n  console.log('broil');\n}",
        );
        assert_eq!(
            definition.text(TextMode::Overview),
            "export function broil() {\n...\n}",
        );
    }

    #[test]
    fn typescript_snippet_references() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir)
            .join("examples")
            .join("typescript");

        let mut snippet = Snippet::new(
            Language::TypeScript,
            examples_dir.join("chef.ts").display().to_string(),
            2,
            2,
        );
        let references = snippet.references(String::from(""));
        let refs: Vec<_> = references
            .into_iter()
            .map(|r| {
                format!(
                    "{}:{}:{}=>{}",
                    Path::new(&r.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap(),
                    r.line,
                    r.column,
                    r.text,
                )
            })
            .collect();

        assert_eq!(refs, vec!["chef.ts:2:0=>broil"]);
    }

    #[test]
    fn typescript_navigator_resolve() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir)
            .join("examples")
            .join("typescript");

        let mut nav = Navigator::new(Language::TypeScript, String::from("./test.sqlite"));
        nav.index(vec![examples_dir.display().to_string()], true);

        let reference = Reference {
            path: examples_dir.join("chef.ts").display().to_string(),
            line: 2,
            column: 0,
            text: String::from("broil"),
        };

        let definitions = nav.resolve(reference);
        let defs: Vec<_> = definitions
            .into_iter()
            .map(|d| {
                format!(
                    "{}:{}:{}",
                    Path::new(&d.path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap(),
                    d.span.start.line,
                    d.span.start.column
                )
            })
            .collect();

        nav.clean(true);

        assert_eq!(defs, vec!["chef.ts:0:9", "kitchen.ts:2:16"]);
    }

    #[test]
    fn typescript_definition_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let examples_dir = PathBuf::from(manifest_dir)
            .join("examples")
            .join("typescript");

        let definition = Definition {
            language: Language::TypeScript,
            path: examples_dir.join("stove.ts").display().to_string(),
            span: Span {
                start: Point {
                    line: 4,
                    column: 16,
                },
                end: Point {
                    line: 4,
                    column: 20,
                },
            },
        };
        assert_eq!(
            definition.text(TextMode::Complete),
            "export function broil() {\n  console.log('broil');\n}",
        );
        assert_eq!(
            definition.text(TextMode::Overview),
            "export function broil() {\n...\n}",
        );
    }
}
