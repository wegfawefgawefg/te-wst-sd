#[macro_use]
extern crate rocket;

use crate::requests::get_available_submissions;
use crate::requests::get_big_file_streaming_socket;
use crate::requests::new_submission;

mod common;
mod db;
mod requests;
mod sanitation;
mod submissions;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            new_submission,
            get_available_submissions,
            get_big_file_streaming_socket
        ],
    )
}
