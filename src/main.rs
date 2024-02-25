use rocket::response::status;
#[macro_use] extern crate rocket;
use rocket::http::{Status, ContentType};

#[get("/")]
fn json() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON, "{ \"hi\": \"world\" }"))
}

#[get("/<id>")]
fn new(id: usize) -> status::Accepted<String> {
    status::Accepted(format!("id: '{}'", id))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![json,new])
}
