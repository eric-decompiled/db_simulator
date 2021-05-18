extern crate db_simulator;
use self::db_simulator::*;
use crate::db::{establish_connection, init_tables, query_events};
use crate::events::{parse_type_map, random_event_type};
use crate::utils::{random_interval, sleep_to_cadence};
use dotenv::dotenv;
use std::time::{Duration, SystemTime};

/// Queries the DB for a random event type between a random interval every 100 ms
fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let mut db_client = establish_connection();
    init_tables(&mut db_client).expect("could not init tables");
    let events = parse_type_map();
    let mut rng = rand::thread_rng();
    let cadence = Duration::from_millis(100);
    loop {
        let start = SystemTime::now();
        let interval = random_interval(&mut rng);
        let event_type = random_event_type(&events, &mut rng);
        match query_events(&mut db_client, &event_type, interval[0], interval[1]) {
            Ok(results) => {
                println!(
                    "Found {} results of type: {} between {:?} and {:?}",
                    results.len(),
                    event_type,
                    interval[0],
                    interval[1]
                );
            }
            Err(e) => println!("Problem querying for events: {:?}", e),
        };
        let exec_time = start.elapsed().expect("Could not get elapsed time");
        sleep_to_cadence(cadence, exec_time);
    }
}
