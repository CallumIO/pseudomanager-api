use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize}
use serde_json::{Result, Value};

#[post("/pseudonym")]
pub async fn handle_request(payload: web::Json<serde_json::Value>)) -> impl Responder {
    // payload is the JSON body object

    // Response
    // HttpResponse::Ok().json(Pseudonym {});
    HttpResponse::Ok();
}

// object to be sent to the client
// TODO: Add correct properties

#[derive(Serialize)]
pub struct Pseudonym {
    firstname: String,
    lastname: String,
    middlename: String,
    age: u8,
    birthdate: String,
    address: Address,
}
