/// Represents a single FFmpeg execution step
#[derive(Debug, Clone)]
pub struct Step {
    pub program: String,
    pub args: Vec<String>,
}

impl Step {
    pub fn new(program: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            program: program.into(),
            args,
        }
    }
}

