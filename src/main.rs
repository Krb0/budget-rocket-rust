mod database;
mod routes;
use routes::auth::auth_router;
#[macro_use]
extern crate rocket;
use database::setup::set_up_db;
#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    println!("Database setup complete! ");

    rocket::build().mount("/auth", auth_router())
}
