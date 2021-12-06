use actix_web::{
    get,
    web::{self, Query},
    App, HttpResponse, HttpServer, Responder,
};
use std::time::Instant;

mod config;
mod interfaces;
mod model;

use interfaces::*;
use model::UdiabModel;

#[get("/key_hints")]
async fn get_key_hints(
    udiab_model: web::Data<UdiabModel>,
    Query(key_hints_request): Query<KeyHintsRequest>,
) -> Result<impl Responder, UserError> {
    let start = Instant::now();
    let key_hints = udiab_model.get_key_hints(key_hints_request.key)?;
    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&KeyHintsResponse {
            key_hints,
            duration: start.elapsed().as_millis(),
        })
        .unwrap(),
    ))
}

#[get("/retrieved_info")]
async fn get_retrieved_info(
    udiab_model: web::Data<UdiabModel>,
    Query(retrieve_info_request): Query<RetrievedInfoRequest>,
) -> Result<impl Responder, UserError> {
    let start = Instant::now();
    let RetrievedInfoRequest {
        key,
        advanced_search_options,
        offset,
        page_size,
    } = retrieve_info_request;
    let article_infos =
        udiab_model.get_retrieved_info(key, advanced_search_options, offset, page_size)?;
    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&RetrievedInfoResponse {
            article_infos,
            duration: start.elapsed().as_millis(),
        })
        .unwrap(),
    ))
}

#[get("/top_info")]
async fn get_top_info(udiab_model: web::Data<UdiabModel>) -> Result<impl Responder, UserError> {
    let start = Instant::now();
    let top_article_infos = udiab_model.get_top_info()?;
    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&TopArticleInfoResponse {
            top_article_infos,
            duration: start.elapsed().as_millis(),
        })
        .unwrap(),
    ))
}

#[get("/more_like_this")]
async fn get_more_like_this(
    udiab_model: web::Data<UdiabModel>,
    Query(more_like_this_request): Query<MoreLikeThisRequest>,
) -> Result<impl Responder, UserError> {
    let start = Instant::now();
    let more_like_this_article_infos = udiab_model.get_more_like_this(
        more_like_this_request.address,
        more_like_this_request.offset,
        more_like_this_request.page_size,
    )?;
    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&MoreLikeThisResponse {
            more_like_this_article_infos,
            duration: start.elapsed().as_millis(),
        })
        .unwrap(),
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::retrieve_config();
    let (index, project_document) = search_base::index(&config.index_store_directory).unwrap();
    let reader = index.reader().unwrap();

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(web::Data::new(UdiabModel {
                    reader: reader.clone(),
                    project_document: project_document.clone(),
                }))
                .service(get_key_hints)
                .service(get_retrieved_info)
                .service(get_top_info)
                .service(get_more_like_this),
        )
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
