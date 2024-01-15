use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use serde_json::json;
use sqlite::State;

use crate::app::db::AppState;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(navaid);
    cfg.service(navaid_by_icao_code);

    info!("navaids routes loaded");
}

#[derive(Deserialize)]
struct FormData {
    page: Option<u32>,
    search: Option<String>,
}

#[get("/navaid")]
async fn navaid(param: web::Query<FormData>, app_state: web::Data<AppState>) -> impl Responder {
    let con = app_state.sqlite_connection.lock().unwrap();

    let search = param.search.clone();

    let mut statement = match search {
        Some(search) => {
            let query = "SELECT * FROM navaids WHERE icao_code LIKE '%' || ? || '%' OR name LIKE '%' || ? || '%' OR associated_airport LIKE '%' || ? || '%' LIMIT 100";
            let mut s = con.prepare(query).unwrap();
            s.bind((1, search.as_str())).unwrap();
            s.bind((2, search.as_str())).unwrap();
            s.bind((3, search.as_str())).unwrap();
            s
        }
        None => {
            let query = "SELECT * FROM navaids LIMIT 100";
            con.prepare(query).unwrap()
        }
    };

    let mut data = json!([]);

    while let Ok(State::Row) = statement.next() {
        let mut navaid = json!({});

        for column_name in statement.column_names() {
            navaid.as_object_mut().unwrap().insert(
                column_name.clone(),
                json!(statement.read::<String, _>(column_name.as_str()).unwrap()),
            );
        }
        data.as_array_mut().unwrap().push(navaid);
    }

    HttpResponse::Ok().json(json!({"status": "success", "navaid" : data}))
}

#[get("/navaid/{icao}")]
async fn navaid_by_icao_code(
    icao: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let query = "SELECT * FROM navaids WHERE icao_code=?";

    let mut data = json!([]);

    let con = app_state.sqlite_connection.lock().unwrap();
    let mut statement = con.prepare(query).unwrap();
    statement.bind((1, icao.as_str())).unwrap();

    while let Ok(State::Row) = statement.next() {
        let mut navaid_data = json!({});

        for column_name in statement.column_names() {
            navaid_data.as_object_mut().unwrap().insert(
                column_name.clone(),
                json!(statement.read::<String, _>(column_name.as_str()).unwrap()),
            );
        }
        data.as_array_mut().unwrap().push(navaid_data);
    }

    HttpResponse::Ok().json(json!({"status": "success", "navaid" : data}))
}
