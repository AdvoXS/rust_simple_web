
use crate::controller;
use crate::controller::Controller;
use crate::controller_404::Controller404;
use crate::home_controller::HomeController;
use crate::request::{Request, TypeRequest};
use crate::response::Response;

pub fn handle_request(request : Request) -> Option<Response> {
        get_controller(request).handle()
}

fn get_controller(request : Request) -> Box<dyn Controller> {
    if request.type_request == TypeRequest::GET && request.location_raw == "/" {
        Box::new(HomeController{})
    }
    else { Box::new(Controller404{}) }
}