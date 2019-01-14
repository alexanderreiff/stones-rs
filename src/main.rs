#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn establish_connection() -> PgConnection {
    use std::env;

    let database_url = env::var("DATABASE_URL").expect("Env var `DATABASE_URL` must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to: {}", database_url))
}

fn main() {
    let _conn = establish_connection();
    println!("Hello, world!");
}

#[cfg(test)]
mod diesel_tests {
    use super::*;
    use crate::schema::stones::dsl::*;

    #[test]
    fn test_setup() {
        let conn = establish_connection();
        let results = stones.load::<models::Stone>(&conn);

        if let Ok(results) = results {
            println!("Tracking {} stones", results.len());
            assert_eq!(0, results.len());
            return;
        }
        panic!("Error loading stones");
    }
}
