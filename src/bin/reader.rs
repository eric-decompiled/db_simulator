extern crate db_simulator;
use self::db_simulator::*;
use crate::db::query_events;
use crate::events::random_event_type;
use crate::utils::{random_interval, setup, sleep_to_cadence};
use std::time::{Duration, SystemTime};

/// Queries the DB for a random event type between a random interval every 100 ms
fn main() -> Result<(), std::io::Error> {
    let (mut db_client, events) = setup();
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

#[test]
fn check_reader() {
    let (mut db_client, _) = setup();
    let _ = events::insert_fixed_event(&mut db_client, 1);
    let _ = events::insert_fixed_event(&mut db_client, 2);
    let _ = events::insert_fixed_event(&mut db_client, 3);

    match query_events(
        &mut db_client,
        "mint_coins",
        SystemTime::UNIX_EPOCH,
        SystemTime::UNIX_EPOCH + Duration::from_secs(1),
    ) {
        Ok(results) => assert!(results.len() >= 3, "should find at least 3 results"),
        Err(_) => assert!(false, "query should not error"),
    }
}
