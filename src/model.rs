#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub done: bool
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub enum FilterView {
    #[default]
    All,
    Active,
    Completed
}

impl FilterView {
    pub const VALUES: [Self;3] = [Self::All, Self::Active, Self::Completed];

    pub fn as_string(&self) -> String {
        match self {
            Self::All => "All".to_string(),
            Self::Active => "Active".to_string(),
            Self::Completed => "Completed".to_string()
        }
    }

    pub fn get_link(&self) -> String {
      match self {
          Self::All => "#/".to_string(),
          Self::Active => "#/active".to_string(),
          Self::Completed => "#/completed".to_string()
      }
  }
}