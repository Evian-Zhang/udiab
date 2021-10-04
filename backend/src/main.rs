use actix_web::{
    get,
    web::{self, Query},
    App, HttpResponse, HttpServer, Responder,
};
use search_base::ProjectDocument;
use tantivy::{query::QueryParser, IndexReader};

mod config;
mod interfaces;

use interfaces::*;

struct AppState {
    reader: IndexReader,
    project_document: ProjectDocument,
}

#[get("/key_hints")]
async fn get_key_hints(
    app_state: web::Data<AppState>,
    Query(key_hints_request): Query<KeyHintsRequest>,
) -> Result<impl Responder, UserError> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&KeyHintsResponse { key_hints: vec![] }).unwrap()))
}

#[get("/retrieved_info")]
async fn get_retrieved_info(
    app_state: web::Data<AppState>,
    Query(retrieve_info_request): Query<RetrievedInfoRequest>,
) -> Result<impl Responder, UserError> {
    let searcher = app_state.reader.searcher();

    let query_parser = QueryParser::for_index(
        searcher.index(),
        retrieve_info_request
            .advanced_search_options
            .search_field
            .tantivy_fields(app_state.project_document),
    );
    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&RetrievedInfoResponse {
            article_infos: vec![],
        })
        .unwrap(),
    ))
}

#[get("/top_info")]
async fn get_top_info(app_state: web::Data<AppState>) -> Result<impl Responder, UserError> {
    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&TopArticleInfoResponse {
            top_article_infos: vec![],
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
                .app_data(web::Data::new(AppState {
                    reader: reader.clone(),
                    project_document: project_document.clone(),
                }))
                .service(get_key_hints)
                .service(get_retrieved_info),
        )
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
