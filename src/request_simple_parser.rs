
use regex::{Captures, Match, Regex};
use crate::request::{Location, Request, TypeRequest};


pub fn parse_request(request_str: String) -> Option<Request> {
    let reg_exp_full_req = Regex::new(r"([A-Za-z]+)\s(.+)\s(.+)");

    let captures = reg_exp_full_req.unwrap().captures_iter(&*request_str).next().unwrap();
    let type_request= get_type_request(&captures)?;

    Some(Request {
        type_request,
        location_raw: get_location().0,
        location: get_location().1,
        params: Default::default(),
    })
}

fn get_type_request(captures : &Captures) -> Option<TypeRequest>{
     Some(match captures.get(1) {
        None => return None,
        Some(x) => match x.as_str() {
            "POST" => { TypeRequest::POST }
            _ => TypeRequest::GET
        }
    })
}

fn get_location() -> (String, Box<Location>){
    //todo
    let location_raw = String::from("/");
    (location_raw, Box::new(Location::Place(String::from("/"), Box::new(Location::Nil))))
}