
#[macro_use] extern crate rocket;

#[get("/t")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/d", routes![index])
}
