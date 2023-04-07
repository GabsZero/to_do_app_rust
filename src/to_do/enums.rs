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

  pub fn from_string(input: String) -> Self {
    match input.as_str() {
      "DONE" => TaskStatus::DONE,
      "PENDING" => TaskStatus::PENDING,
      _ => panic!("input {} not supported", input)
    }
  }
}