use actix_web::{Responder, web};
use serde_json::{Value, Map};

use crate::state::read_file;


pub async fn get() -> impl Responder {
  let state: Map<String, Value> = read_file("./state.json");

  return web::Json(state);
}