use super::enums::TaskStatus;
use super::structs::done::Done;
use super::structs::pending::Pending;

pub enum ItemTypes{
  Pending(Pending),
  Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
  match status {
    TaskStatus::DONE => {
      ItemTypes::Done(Done::new(title))
    },
    TaskStatus::PENDING => {
      ItemTypes::Pending(Pending::new(title))
    }
  }
}