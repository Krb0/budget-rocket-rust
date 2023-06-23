use rocket::{get, routes, Route};
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

#[get("/login")]
fn auth_index() -> Json<Message> {
    let message = Message { message: "Login OK!" };
    Json(message)
}

pub fn auth_router() -> Vec<Route>{
    routes![auth_index]
}
