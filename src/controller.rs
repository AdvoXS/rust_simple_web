use crate::response::Response;
//todo понять как использовать для home_controller
pub trait Controller {
    fn handle(&self) -> Option<Response>;
    fn send_response(&self) {

    }
}
