#![feature(proc_macro_hygiene, decl_macro)] // language features needed by Rocket

// Import the rocket macros
#[macro_use]
extern crate rocket;

// Create route / that returns "Hello, world!"
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[cfg(test)] // Only compile this when unit testing is requested
mod tests {
  use super::*; // Modules are their own scope, so you 
                // need to explictly use the stuff in
                // the parent module.
                
  use rocket::http::Status;
  use rocket::local::*;
  
  #[test]
  fn test_index() {
    // create the rocket instance to test
    let rkt = rocket::ignite().mount("/", routes![index]);
    
    // create a HTTP client bound to this rocket instance
    let client = Client::new(rkt).expect("valid rocket");
    
    // get a HTTP response
    let mut response = client.get("/").dispatch();
    
    // Ensure it returns HTTP 200
    assert_eq!(response.status(), Status::Ok);
    
    // Ensure the body is what we expect it to be
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
  }
}
