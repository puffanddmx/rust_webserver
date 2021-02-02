#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
extern crate serde;
extern crate diesel;
extern crate hello_rocket;
extern crate rand;
extern crate time;

#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate serde_derive;

use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use hello_rocket::{create_new_user_session, establish_connection};
use time::Duration;

#[derive(Serialize)]
struct IndexContext {
    header: String
}

#[derive(FromForm)]
struct User {
    username: String,
    password: String,
}

#[get("/")]
fn index() -> Template {
    let context = IndexContext {
        header: "Hello World".to_string(),
    };
    Template::render("index", &context)

}

fn generate_session_token(length: u8) -> Result<String, std::io::Error> {
    let bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
    let strings: Vec<String> = bytes.iter().map(|byte| format!("{:02X}", byte)).collect();
    return OK(strings.join(""));

}

#[post("/", data = "<user>")]
fn login(mut cookies: Cookies, user: Form<User>) -> Result<Redirect, String> {
    if user.get().username == "jaylee".to_string() && user.get().password = "lee9891" {
        match generate_session_token(64) {
            OK(session_token) => {
                let connection = establish_connection();
                let user_id = 1;
                create_new_user_session(&connetion, user_id,session_token.clone());
                let mut c = Cookie::new("session_token", session_token);
                c.set_max_age(Duration::from_secs(30));
                cookies.add_private(c);
                OK(Redirect::to("/"))
            }
            Err(_) => Err(String::from("Login failed")),
        }
    } else {
        Err(String::from("username or password incorrect"))
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, login])
    .attach(Template::fairing())
    .launch();
}