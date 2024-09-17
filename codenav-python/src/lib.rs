use codenav;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyTuple};

#[pyclass]
#[derive(Clone)]
struct Point {
    #[pyo3(get)]
    line: usize,
    #[pyo3(get)]
    column: usize,
}

impl From<codenav::Point> for Point {
    fn from(p: codenav::Point) -> Self {
        Self {
            line: p.line,
            column: p.column,
        }
    }
}

impl ToPyObject for Point {
    fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
        self.clone().into_py(py)
    }
}

#[pyclass]
#[derive(Clone)]
struct Span {
    #[pyo3(get)]
    start: Point,
    #[pyo3(get)]
    end: Point,
}

impl From<codenav::Span> for Span {
    fn from(s: codenav::Span) -> Self {
        Self {
            start: Point::from(s.start),
            end: Point::from(s.end),
        }
    }
}

impl ToPyObject for Span {
    fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
        self.clone().into_py(py)
    }
}

#[pyclass]
#[derive(Clone, Copy)]
enum TextMode {
    Overview = 0,
    Complete = 1,
}

#[pyclass]
#[derive(Clone)]
struct Definition {
    #[pyo3(get)]
    path: String,
    #[pyo3(get)]
    span: Span,
}

#[pymethods]
impl Definition {
    #[pyo3(signature = (mode=TextMode::Complete, /))]
    fn text<'py>(&self, py: Python<'py>, mode: TextMode) -> PyResult<String> {
        let d = codenav::Definition {
            path: self.path.clone(),
            span: codenav::Span {
                start: codenav::Point {
                    line: self.span.start.line,
                    column: self.span.start.column,
                },
                end: codenav::Point {
                    line: self.span.end.line,
                    column: self.span.end.column,
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
            path: d.path,
            span: Span::from(d.span),
        }
    }
}

impl ToPyObject for Definition {
    fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
        self.clone().into_py(py)
    }
}

#[pyclass]
#[derive(Clone)]
struct Capture {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    start: Point,
    #[pyo3(get)]
    end: Point,
}

impl From<codenav::Capture> for Capture {
    fn from(c: codenav::Capture) -> Self {
        Self {
            name: c.name,
            text: c.text,
            start: Point::from(c.start),
            end: Point::from(c.end),
        }
    }
}

impl ToPyObject for Capture {
    fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
        self.clone().into_py(py)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Reference {
    /// File path
    #[pyo3(get)]
    pub path: String,
    /// Position line (0-based)
    #[pyo3(get)]
    pub line: usize,
    /// Position column (0-based grapheme)
    #[pyo3(get)]
    pub column: usize,
    /// The text string
    #[pyo3(get)]
    pub text: String,
}

#[pymethods]
impl Reference {
    #[new]
    fn new(path: String, line: usize, column: usize, text: String) -> Self {
        Self {
            path: path,
            line: line,
            column: column,
            text: text,
        }
    }
}

impl From<codenav::Reference> for Reference {
    fn from(r: codenav::Reference) -> Self {
        Self {
            path: r.path,
            line: r.line,
            column: r.column,
            text: r.text,
        }
    }
}

impl ToPyObject for Reference {
    fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
        self.clone().into_py(py)
    }
}

#[pyclass]
#[derive(Clone)]
struct ParseResult {
    #[pyo3(get)]
    capture: Capture,
    #[pyo3(get)]
    definitions: Vec<Definition>,
}

impl From<codenav::ParseResult> for ParseResult {
    fn from(r: codenav::ParseResult) -> Self {
        Self {
            capture: Capture::from(r.capture),
            definitions: r
                .definitions
                .into_iter()
                .map(Definition::from)
                .collect::<Vec<_>>(),
        }
    }
}

impl ToPyObject for ParseResult {
    fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
        self.clone().into_py(py)
    }
}

#[pyclass]
struct Navigator {
    nav: codenav::Navigator,
}

#[pymethods]
impl Navigator {
    // Example:
    //
    // ```python
    // import codenav_python as codenav
    // nav = codenav.Navigator("./test.sqlite")
    // ```
    #[new]
    fn new(db_path: String) -> Self {
        Self {
            nav: codenav::Navigator::new(db_path),
        }
    }

    // Args:
    // source_paths: Source file or directory paths to index.
    // force: Index files even if they are already present in the database.
    //
    // Example:
    //
    // ```python
    // nav.index(["/Users/russellluo/Projects/work/opencsg/projects/crmaestro/codegpt/apps"])
    // ```
    #[pyo3(signature = (source_paths, force=false, /))]
    fn index<'py>(&self, py: Python<'py>, source_paths: Vec<String>, force: bool) -> PyResult<()> {
        self.nav.index(source_paths, force);
        Ok(())
    }

    // Example:
    //
    // ```python
    // nav.clean()
    // ```
    #[pyo3(signature = (delete=false, /))]
    fn clean<'py>(&self, py: Python<'py>, delete: bool) -> PyResult<()> {
        self.nav.clean(delete);
        Ok(())
    }

    // Example:
    //
    // ```python
    // reference = codenav.Reference('/Users/russellluo/Projects/work/opencsg/projects/crmaestro/codegpt/apps/codereview/tasks.py', 274, 1)
    // nav.resolve(reference)
    // ```
    fn resolve<'py>(&mut self, py: Python<'py>, reference: Reference) -> PyResult<Py<PyTuple>> {
        let definitions = self.nav.resolve(codenav::Reference {
            path: reference.path,
            line: reference.line,
            column: reference.column,
            text: reference.text,
        });
        let py_definitions = definitions
            .into_iter()
            .map(Definition::from)
            .collect::<Vec<_>>();
        let tuple = PyTuple::new_bound(py, py_definitions).unbind();
        Ok(tuple)
    }
}

#[pyclass]
struct Snippet {
    s: codenav::Snippet,
}

#[pymethods]
impl Snippet {
    // Example:
    //
    // ```python
    // import codenav_python as codenav
    // s = codenav.Snippet("test.py", 0, 11)
    // ```
    #[new]
    fn new(path: String, line_start: usize, line_end: usize) -> Self {
        Self {
            s: codenav::Snippet::new(path, line_start, line_end),
        }
    }

    // Example:
    //
    // ```python
    // s.references()
    // ```
    #[pyo3(signature = (query_path="".to_string(), /))]
    fn references<'py>(&self, py: Python<'py>, query_path: String) -> PyResult<Py<PyTuple>> {
        let references = self.s.references(query_path);
        let py_references = references
            .into_iter()
            .map(Reference::from)
            .collect::<Vec<_>>();
        let tuple = PyTuple::new_bound(py, py_references).unbind();
        Ok(tuple)
    }

    // Example:
    //
    // ```python
    // s.contains(Definition(...))
    // ```
    fn contains<'py>(&self, py: Python<'py>, d: Definition) -> PyResult<bool> {
        let contained = self.s.contains(codenav::Definition {
            path: d.path,
            span: codenav::Span {
                start: codenav::Point {
                    line: d.span.start.line,
                    column: d.span.start.column,
                },
                end: codenav::Point {
                    line: d.span.end.line,
                    column: d.span.end.column,
                },
            },
        });
        Ok(contained)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "codenav")]
fn codenav_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Span>()?;
    m.add_class::<TextMode>()?;
    m.add_class::<Definition>()?;
    m.add_class::<Capture>()?;
    m.add_class::<Reference>()?;
    m.add_class::<ParseResult>()?;
    m.add_class::<Navigator>()?;
    m.add_class::<Snippet>()?;
    Ok(())
}
