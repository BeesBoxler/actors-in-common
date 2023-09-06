#![allow(dead_code)]
mod actor;
mod film;
mod tmdb_api;

use actor::Actor;
use film::Film;
use tmdb_api::TmdbApi;

#[tokio::main]
async fn main() {
    let tmdb_api = TmdbApi::init();

    // Fight Club id: 550
    // Bullet Train id: 718930

    let film1 = tmdb_api.fetch_film("550").await.ok().unwrap();
    let film2 = tmdb_api.fetch_film("718930").await.ok().unwrap();

    println!("Comparing films {} and {}", film1.title, film2.title);
    println!();

    let actors_in_common = compare_films(film1, film2);

    for Actor { name, .. } in &actors_in_common {
        println!("- {name}");
    }

    println!();
    println!("Total: {}", actors_in_common.len());
}

fn compare_films(a: Film, b: Film) -> Vec<Actor> {
    a.cast
        .iter()
        .filter_map(|actor| {
            if b.cast.contains(actor) {
                Some(actor.clone())
            } else {
                None
            }
        })
        .collect()
}
