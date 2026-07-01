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

#[allow(dead_code)]
pub fn unexpected_eof(line: usize, column: usize, expected: &str) -> String {
    format!(
        "AgentML parse error at line {}, column {}:\n  Unexpected end of input. Expected {}.\n",
        line, column, expected
    )
}

#[allow(dead_code)]
pub fn unterminated_string(line: usize, column: usize) -> String {
    format!(
        "AgentML parse error at line {}, column {}:\n  Unterminated string literal.\nSuggestion: Add closing `\"` at the end of the string.\n",
        line, column
    )
}

#[allow(dead_code)]
pub fn unknown_field(line: usize, column: usize, field: &str) -> String {
    format!(
        "AgentML parse warning at line {}, column {}:\n  Unknown field `{}`. This section will be ignored.\n",
        line, column, field
    )
}
