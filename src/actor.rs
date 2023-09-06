#[derive(serde::Deserialize, Debug, Clone)]
pub struct Actor {
    pub name: String,
    pub id: usize,
}

impl std::cmp::PartialEq for Actor {
    fn eq(&self, other: &Actor) -> bool {
        self.id == other.id
    }
}
