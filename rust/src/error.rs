#[derive(Debug)]
pub struct BindError {
    message: String
}

#[allow(dead_code)]
impl BindError {

    pub fn from_raw_str(message: &str) -> Self {
        BindError { message: String::from(message) }
    }

    pub fn from_string(message: String) -> Self {
        BindError { message }
    }

    pub fn get_message(self) -> String {
        self.message
    }
}
