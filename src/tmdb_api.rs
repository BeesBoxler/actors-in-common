

use serde::de::DeserializeOwned;

use crate::{actor::Actor, film::Film, query::Query};

static TMDB_URL: &str = "https://api.themoviedb.org/3/";

pub struct TmdbApi {
    key: String,
}

impl TmdbApi {
    pub fn init() -> Self {
        let key = dotenv::var("TMDB_KEY").expect("Please set `TMDB_KEY` environment variable");

        TmdbApi { key }
    }

    pub async fn search_film(&self, query: &str) -> Result<Query<Film>, reqwest::Error> {
        self.get::<Query<Film>>("search/movie", &format!("query={query}"))
            .await
    }

    pub async fn fetch_actor(&self, id: &str) -> Result<Actor, reqwest::Error> {
        self.get::<Actor>(&format!("person/{id}"), "").await
    }

    pub async fn fetch_film(&self, id: usize) -> Result<Film, reqwest::Error> {
        self.get::<Film>(&format!("movie/{id}"), "append_to_response=credits")
            .await
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        append: &str,
    ) -> Result<T, reqwest::Error> {
        reqwest::get(format!("{TMDB_URL}/{path}?api_key={}&{append}", self.key))
            .await?
            .json::<T>()
            .await
    }
}
