use rocket::response::content::RawJson;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawJson<&'static str> {
    RawJson("{'message': 'Hello, world!'}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
