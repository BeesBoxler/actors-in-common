use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
pub struct Query<T: DeserializeOwned> {
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub results: Vec<T>,
}
