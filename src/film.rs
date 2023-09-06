use crate::actor::Actor;
use serde_flat_path::flat_path;

#[flat_path]
#[derive(serde::Deserialize, Debug)]
pub struct Film {
    pub title: String,

    #[flat_path("credits.cast")]
    pub cast: Vec<Actor>,
}
