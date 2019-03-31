pub mod model;
pub mod schema;

use rocket::request::{Form, FromForm, FormItems, FromParam};
use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::Response;
use self::model::Reaction;
use user::auth::ApiKey;

use db;

#[options( "/")]
fn options_handler<'a>() -> Response<'a> {
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "Any")
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[get("/list/<id>")]
fn read(key: ApiKey, id: i32, connection: db::Connection) -> JsonValue {
    let res = Reaction::count_reaction_type_to_post(id, &connection);
    json!(
        {
            "code": 0,
            "data": res
        }
    )
}

#[get("/list/<id>", rank = 2)]
fn read_noauth(id: i32) -> JsonValue {
    json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    )
}

#[post("/", data = "<Reaction>")]
fn create(key: ApiKey, Reaction: Json<Reaction>, connection: db::Connection) -> JsonValue {
    let insert = Reaction { reaction_id: None, author_id: Some(key.0.parse::<i32>().unwrap()), ..Reaction.into_inner() };
    if(Reaction::create(insert, &connection)) {
        json!({
            "code": 0,
            "data": "Reaction creato correttamente"
        })
    } else {
        json!({
            "code": 1,
            "data": "Errore nella creazione del Reaction"
        })
    }
}

#[post("/", data = "<Reaction>", rank = 2)]
fn create_noauth(Reaction: Json<Reaction>) -> JsonValue {
    json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    )
}

#[options( "/<id>")]
fn options_handler_delete<'a>(id: i32) -> Response<'a> {
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "Any")
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, DELETE")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[delete("/<post_id>")]
fn delete(key: ApiKey, post_id: i32, connection: db::Connection) -> JsonValue {
    if (Reaction::delete(post_id, key.0.parse::<i32>().unwrap(), &connection)){
        json!({
            "code": 0,
            "data": "Reaction cancellato correttamente"
        })
    } else {
        json!({
            "code": 1,
            "data": "Errore nel cancellare Il Reaction"
        })
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/reactions", routes![options_handler, read, read_noauth, create, create_noauth, options_handler_delete, delete])
}