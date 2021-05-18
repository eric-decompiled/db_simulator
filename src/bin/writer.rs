extern crate db_simulator;
use self::db_simulator::*;
use crate::db::{establish_connection, init_tables};
use crate::events::{insert_random_event, parse_type_map};
use crate::utils::sleep_to_cadence;
use dotenv::dotenv;
use std::time::{Duration, SystemTime};

/// Writes a random event into the DB every 5 seconds
fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let mut db_client = establish_connection();
    init_tables(&mut db_client).expect("could not initialize tables");
    let events = parse_type_map();
    let cadence = Duration::from_secs(5);
    let mut rng = rand::thread_rng();
    loop {
        let start = SystemTime::now();
        match insert_random_event(&mut db_client, &events, &mut rng) {
            Ok((event_type, payload)) => {
                println!("Inserted {} type with payload: {:?}", event_type, payload)
            }
            Err(e) => println!("could not insert event {:?}", e),
        }
        let exec_time = start.elapsed().expect("Could not get exec time");
        sleep_to_cadence(cadence, exec_time);
    }
}
