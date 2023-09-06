use crate::actor::Actor;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Film {
    pub title: String,
    pub id: usize,

    #[serde(default)]
    pub credits: Credits,
}

#[derive(Default, Debug, serde::Deserialize, Clone)]
pub struct Credits {
    pub cast: Vec<Actor>,
}
