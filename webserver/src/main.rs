#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> &'static str {
    "Index!"
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! {
        title: "About",
        content: "This is the about page! <br> This page uses rocket and handlebars template engine"
    })
}

#[get("/profile")]
fn profile() -> &'static str {
    "Profile"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, about, profile])
    .attach(Template::fairing())
}