pub mod model;
pub mod schema;

use rocket::request::{Form, FromForm, FormItems, FromParam};
use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::Response;
use self::model::Post;
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

#[post("/", data = "<Post>")]
fn create(key: ApiKey, Post: Json<Post>, connection: db::Connection) -> JsonValue {
    let insert = Post { post_id: None, author_id: Some(key.0.parse::<i32>().unwrap()), ..Post.into_inner() };
    if(Post::create(insert, &connection)) {
        json!({
            "code": 0,
            "data": "Post creato correttamente"
        })
    } else {
        json!({
            "code": 1,
            "data": "Errore nella creazione del Post"
        })
    }
}

#[post("/", data = "<Post>", rank = 2)]
fn create_noauth(Post: Json<Post>) -> JsonValue {
    json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    )
}

#[derive(FromForm)]
pub struct Parameters {
    limit: i64,
    offset: i64,
    parent_id: i32,
    sortBy: Option<String>,
    sortDesc: Option<bool>,
    content: Option<String>,
}

#[get("/list?<parameters..>")]
fn read(key: ApiKey, parameters: Option<Form<Parameters>>, connection: db::Connection) -> Result<JsonValue, Status> {
    match parameters {
        Some(p) => {
            let count: i64 = Post::count(p.parent_id.clone(), p.content.clone(),  &connection);
            Post::read(p.limit.clone(), p.offset.clone(), p.parent_id.clone(), p.sortBy.clone(), p.sortDesc.clone(), p.content.clone(), &connection)
                .map(|item| {
                    return json!({
                        "code": 0,
                        "data": item,
                        "pagination": {
                            "totalRows": count
                        }
                    })
                })
                .map_err(|_| Status::NotFound)
        },
        None => {
            Ok(json!({
                "code": 1
            }))
        }
    }
}

#[get("/list?<parameters..>", rank = 2)]
fn read_noauth(parameters: Option<Form<Parameters>>) -> JsonValue {
    json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    )
}

#[put("/<id>", data = "<Post>")]
fn update(key: ApiKey, id: i32, Post: Json<Post>, connection: db::Connection) -> JsonValue {
    let update = Post { post_id: Some(id), ..Post.into_inner() };
    if(Post::update(id, key.0.parse::<i32>().unwrap(), update, &connection)) {
        json!({
            "code": 0,
            "data": "Post aggiornato correttamente"
        })
    } else {
        json!({
            "code": 1,
            "data": "Errore nell'aggiornare Il Post"
        })
    }
}

#[put("/<id>", data = "<Post>", rank = 2)]
fn update_noauth(id: i32, Post: Json<Post>) -> JsonValue {
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

#[delete("/<id>")]
fn delete(key: ApiKey, id: i32, connection: db::Connection) -> JsonValue {
    if (Post::delete(id, key.0.parse::<i32>().unwrap(), &connection)){
        json!({
            "code": 0,
            "data": "Post cancellato correttamente"
        })
    } else {
        json!({
            "code": 1,
            "data": "Errore nel cancellare Il Post"
        })
    }
}

#[delete("/<id>", rank = 2)]
fn delete_noauth(id: i32) -> JsonValue {
    json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    )
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/post", routes![options_handler, read, read_noauth, create, create_noauth, update, update_noauth, options_handler_delete, delete, delete_noauth])
}