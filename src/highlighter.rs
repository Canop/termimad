use std::fmt;
use std::sync::Arc;

/// A wrapper around a syntax-highlighting function.
///
/// The inner function takes a code string and an optional language token,
/// and returns one highlighted (ANSI-escaped) string per input line.
pub struct CodeHighlighter(pub Arc<dyn Fn(&str, Option<&str>) -> Vec<String> + Send + Sync>);

impl CodeHighlighter {
    pub fn highlight(&self, code: &str, lang: Option<&str>) -> Vec<String> {
        (self.0)(code, lang)
    }
}

impl Clone for CodeHighlighter {
    fn clone(&self) -> Self {
        CodeHighlighter(Arc::clone(&self.0))
    }
}

impl fmt::Debug for CodeHighlighter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeHighlighter(..)")
    }
}

impl PartialEq for CodeHighlighter {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
