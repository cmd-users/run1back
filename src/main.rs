#[macro_use] extern crate rocket; 

use rocket::*;
use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
        <div>
            Hello, world!"
        </div>
    "#)
}

#[get("/")]
fn test() -> RawHtml<&'static str> {
    RawHtml(r#"
        <div style="background-color: red">
            test
        </div>
    "#)
} 

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/test", routes![test])
}

