use rocket_dyn_templates::{context, Template};
use rocket::request::{FromRequest, Outcome};
use std::convert::Infallible;
use std::fs;

mod utilities;

use crate::utilities::execute_command;

#[macro_use]
extern crate rocket;

use rocket::http::HeaderMap;
use rocket::Request;

struct RequestHeaders<'h>(&'h HeaderMap<'h>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let request_headers = request.headers();
        Outcome::Success(RequestHeaders(request_headers))
    }
}

#[get("/")]
fn index(headers: RequestHeaders) -> Template {
    println!("{}", headers.0.get_one("Referer").unwrap_or("Uh Oh!"));
    Template::render(
        "index",
        context! {
            lolz: "hello"
        },
    )
}

#[get("/<species>")]
fn animal(species: &str) -> String {
    let noise = match species {
        "cow" => "MOO",
        "cat" => "MIAOW",
        _ => "Hi",
    };
    format!("{} says {}!", species, noise)
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "404",
        context! {
            url: "?",
        },
    )
}

#[catch(500)]
fn error() -> Template {
    Template::render("500", context! {})
}

#[launch]
fn rocket() -> _ {
    execute_command("tailwindcss -i templates/style.css -o dist/output.css --minify");
    let css = fs::read_to_string("./dist/output.css").unwrap();
    fs::write(
        "templates/style.html.tera",
        format!("<style>{}</style>", css),
    )
    .unwrap();
    rocket::build()
        .mount("/", routes![index])
        .mount("/animal", routes![animal])
        .register("/", catchers![not_found])
        .register("/", catchers![error])
        .attach(Template::fairing())
}
