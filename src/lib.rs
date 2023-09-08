pub mod actor;
pub mod film;
pub mod query;
pub mod tmdb_api;

use actor::Actor;
use film::Film;

pub fn compare_films(a: &Film, b: &Film) -> Vec<Actor> {
    a.credits
        .cast
        .iter()
        .filter_map(|actor| {
            if b.credits.cast.contains(actor) {
                Some(actor.clone())
            } else {
                None
            }
        })
        .collect()
}
