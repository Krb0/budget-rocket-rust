mod routes;
use routes::auth::auth_router;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {

    rocket::build().mount("/auth", auth_router())

}
