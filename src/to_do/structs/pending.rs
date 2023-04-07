use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
  pub super_struct: Base
}

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}

impl Pending {
  pub fn new(input_title: &str) -> Self {
    let base = Base{
      title: input_title.to_string(),
      status: TaskStatus::PENDING
    };
    return Pending{super_struct: base}
  }
}