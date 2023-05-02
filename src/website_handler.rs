use super::http::{Response, Request, StatusCode, Method};
use super::server::Handler;

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path  }
  }
}

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
          "/" => Response::new(StatusCode::Ok, Some("<h1>irasshaimase</h1>".to_string())),
          "/hello" => Response::new(StatusCode::Ok, Some("<h1>Kon'nichiwa</h1>".to_string())),
          _ => Response::new(StatusCode::NotFound, None)
      },
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}