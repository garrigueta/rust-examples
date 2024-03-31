#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Index!"
}

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/world", routes![world])
}