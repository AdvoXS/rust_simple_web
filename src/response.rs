#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
}

impl Response {
    pub fn get_status_code_body(&self) -> String {
        format!("HTTP/1.1 {} OK", self.status_code)
    }

    pub fn get_body(&self) -> &String {
        &self.body
    }

    pub fn get_length(&self) -> usize {
        let s = &self.body;
        s.len()
    }
}