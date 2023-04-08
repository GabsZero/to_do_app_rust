use serde::ser::{Serialize, Serializer};

pub enum TaskStatus {
  DONE,
  PENDING
}

impl TaskStatus {
 pub fn stringify(&self) -> String {
    match &self {
      &Self::DONE => "Done".to_string(),
      &Self::PENDING => "Pending".to_string()
    }
  }

 
}

impl Serialize for TaskStatus {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where 
    S: Serializer,
    {
      Ok(
        serializer.serialize_str(
          &self.stringify().as_str()
        )?
      )
    }
}