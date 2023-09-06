#![allow(dead_code)]
mod actor;
mod film;
mod query;
mod tmdb_api;

use actor::Actor;
use film::Film;
use futures;
use tmdb_api::TmdbApi;

#[tokio::main]
async fn main() {
    let tmdb_api = TmdbApi::init();

    let titles = get_titles_from_args();

    let films = futures::future::join_all(
        futures::future::join_all(titles.iter().map(|title| async {
            let results = tmdb_api.search_film(&title.clone()).await.unwrap();
            if results.results.len() == 0 {
                std::process::exit(-1);
            }
            results.results[0].id
        }))
        .await
        .iter()
        .map(|id| async { tmdb_api.fetch_film(*id).await.unwrap() }),
    )
    .await;

    println!("Comparing films {} and {}", films[0].title, films[1].title);
    println!();

    let actors_in_common = compare_films(&films[0], &films[1]);

    for Actor { name, .. } in &actors_in_common {
        println!("- {name}");
    }

    println!();
    println!("Total: {}", actors_in_common.len());
}

fn get_titles_from_args() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Expected 2 titles as arguments.");
        std::process::exit(1);
    }

    args[1..3].to_vec()
}

fn compare_films(a: &Film, b: &Film) -> Vec<Actor> {
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
