/// Utility methods extending [serde_json::Error]
pub trait ErrorExt {
    /// **Zero**-based byte offset at which the error was detected.
    /// Returns `None` if this reaches or exceeds `text.len()`.
    fn offset_within(&self, text: &str) -> Option<usize>;
}

impl ErrorExt for serde_json::Error {
    fn offset_within(&self, text: &str) -> Option<usize> {
        let mut remaining = text;
        for _ in 1..self.line() {
            match remaining.find('\n') {
                None => return None,
                Some(n) => remaining = &remaining[(n+1)..],
            };
        }

        let n = (text.len() - remaining.len()).checked_add(self.column().saturating_sub(1))?;
        return if n < text.len() { Some(n) } else { None };
    }
}
