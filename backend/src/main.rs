use actix_web::{
    get,
    web::{self, Query},
    App, HttpResponse, HttpServer, Responder,
};

mod config;
mod interfaces;

use interfaces::*;

#[get("/key_hints")]
async fn get_key_hints(
    Query(key_hints_request): Query<KeyHintsRequest>,
) -> Result<impl Responder, UserError> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&KeyHintsResponse { key_hints: vec![] }).unwrap()))
}

#[get("/retrieved_info")]
async fn get_retrieved_info(
    Query(retrieve_info_request): Query<RetrievedInfoRequest>,
) -> Result<impl Responder, UserError> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&RetrievedInfoResponse { news_infos: vec![] }).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::retrieve_config();

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(get_key_hints)
                .service(get_retrieved_info),
        )
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
