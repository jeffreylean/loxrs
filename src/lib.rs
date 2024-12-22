pub struct ErrorMessage {
    line: i64,
    location: String,
    message: String, 
}
pub enum Exception {
    RuntimeError(ErrorMessage)
}

impl Exception {
    pub fn runtime_error<T>(line: i64, location: String, message: String) -> Result<T,Exception> {
        Err(Exception::RuntimeError(ErrorMessage{line, location, message}))
    }
}

impl ErrorMessage {
    pub fn report(&self) {
        print!("[line {}] Error{}: {}", self.line, self.location, self.message);
    }
}
