// src/main.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::response::content::RawHtml;
use std::env;

mod schema;
mod models;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to the database")
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
        <div>
            <p>Hello, world!</p>
            <p>Ao clicar, você fará um INSERT e também verá os itens contidos no BD atualmente:</p>
        </div>
        <a href="example">Testar BD</a>
    "#)
}
#[get("/example")]
fn get_example() -> String {
    use schema::example_table::dsl::*;

    let mut connection = establish_connection();

    insert_example_data(&mut connection);
    
    let results = example_table
        .limit(5)
        .load::<models::Example>(&mut connection)
        .expect("Error loading example_table");

    let mut response = String::new();
    response.push_str("Ao entrar nessa página, um INSERT automatico é efetuado.\n<F5> para fazer mais INSERTS no BD");
    for example in results {
        response.push_str(&format!("ID: {}, Name: {}, Age: {}\n", example.id, example.name, example.age));
    }

    response
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_example])
}

fn insert_example_data (connection: &mut PgConnection) {
    use schema::example_table::dsl::*;

    diesel::insert_into(example_table)
        .values((
            name.eq("example".to_string()),
            age.eq(0)
        ))
        .execute(connection)
        .expect("Error inserting example data into the database");
}
