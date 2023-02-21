use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum TypeRequest {
    GET,
    POST,
}
#[derive(Debug)]
pub enum Location {
    Place(String, Box<Location>),
    Nil,
}

#[derive(Debug)]
pub struct Request {
    pub type_request: TypeRequest,
    pub location_raw: String,
    pub location: Box<Location>,
    pub params: HashMap<String, String>,
}