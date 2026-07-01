#[allow(dead_code)]
pub struct Diagnostic {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

#[allow(dead_code)]
impl Diagnostic {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Diagnostic {
            message: message.into(),
            line,
            column,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[allow(dead_code)]
pub fn format_diagnostic(d: &Diagnostic) -> String {
    let mut msg = format!(
        "AgentML parse error at line {}, column {}:\n  {}\n",
        d.line, d.column, d.message
    );
    if let Some(suggestion) = &d.suggestion {
        msg.push_str(&format!("Suggestion: {}\n", suggestion));
    }
    msg
}
