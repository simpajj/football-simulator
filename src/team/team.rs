#[derive(Debug)]
pub struct Team {
  name: &'static str,
  elo: u32
}

impl Team {
  pub fn new(name: &'static str, elo: u32) -> Team {
    Team {
      name: name,
      elo: elo
    }
  }
}

