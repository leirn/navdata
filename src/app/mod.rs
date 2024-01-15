pub mod db;
pub mod navdata;

use log::info;

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    navdata::airport::register_routes(cfg);

    info!("Routes loaded");
}