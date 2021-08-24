use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn get_name(name: String) -> String {
    format!("Hello {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_name])
}