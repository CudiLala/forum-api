use actix_web::{
  web::{self, ServiceConfig},
  HttpResponse,
};
use serde_json::json;

mod api;

pub fn app(cfg: &mut ServiceConfig) {
  cfg
    .service(web::scope("/auth").configure(api::auth::view))
    .default_service(web::to(|| async {
      HttpResponse::NotFound().json(json!({
        "success": false,
        "message": "Route not found. Please check path or method used"
      }))
    }));
}