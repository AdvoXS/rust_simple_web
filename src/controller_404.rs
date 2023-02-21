use crate::controller::Controller;
use crate::response::Response;
use std::{
    fs,
    io::{prelude::*, BufReader},
};
use std::fs::File;
use std::path::Path;
use crate::controller;

pub struct Controller404 {}

impl Controller for Controller404 {
    fn handle(&self) -> Option<Response> {
        let mut file = File::open(Path::new("not_found.html")).expect("panic");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap(); // todo для GET сделать поиск необходимого html
        Some(Response {
            status_code: 404,
            body: String::from(contents),
        })
    }
}