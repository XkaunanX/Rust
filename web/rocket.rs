#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "¡Hola desde Rocket!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
