use actix_web::HttpRequest;
use serde_json::{Map, Value};
use crate::processes::process_input;
use crate::to_do::enum_factory::to_do_factory;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;


pub async fn create(request: HttpRequest) -> String {
  let state: Map<String, Value> = read_file("./state.json");
  let title: String = match request.match_info().get("title") {
    Some(title) => title.to_string(),
    None => panic!("Title is missing")
  };
  let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

  process_input(item, "create".to_string(), &state);

  return format!("{} created", title);
}