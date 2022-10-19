use rocket::*;

#[get("/")]
fn index() -> &'static str {
  "Hello World"
}

#[get("/<name>")]
fn greet(name: &str) -> String {
  format!("Hello, {name}!")
}

#[catch(default)]
fn fail() -> &'static str {
  "FAIL"
}

#[launch]
fn app() -> Rocket<Build> {
  build()
    .mount("/", routes![index])
    .mount("/hello", routes![greet])
    .mount("/hi", routes![greet])
    .register("/", catchers![fail])
}
