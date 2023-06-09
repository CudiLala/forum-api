use actix_web::web::{self, ServiceConfig};
mod controllers;
mod models;

pub fn view(cfg: &mut ServiceConfig) {
  cfg.route("", web::get().to(controllers::fetch_post));
  cfg.route("/save", web::post().to(controllers::save_post));
  cfg.route("/unsave", web::post().to(controllers::unsave_post));
  cfg.route("/comments", web::get().to(controllers::fetch_comments));
  cfg.route("/comments", web::post().to(controllers::create_comment));
}
