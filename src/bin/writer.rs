extern crate db_simulator;
use self::db_simulator::*;
use crate::events::insert_random_event;
use crate::utils::{setup, sleep_to_cadence};
use rand::thread_rng;
use std::time::{Duration, SystemTime};

/// Writes a random event into the DB every 5 seconds
fn main() -> Result<(), std::io::Error> {
    let (mut db_client, events) = setup();
    let mut rng = thread_rng();
    let cadence = Duration::from_secs(5);
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

#[test]
fn check_insert() {
    let (mut db_client, events) = setup();
    let mut rng = rand::thread_rng();
    let result = insert_random_event(&mut db_client, &events, &mut rng);
    assert!(result.is_ok());
}

#[test]
fn check_unique() {
    use rand::Rng;
    let (mut db_client, _) = setup();
    let mut rng = thread_rng();
    let nonce = rng.gen();
    assert!(events::insert_fixed_event(&mut db_client, nonce).is_ok());
    assert!(events::insert_fixed_event(&mut db_client, nonce).is_err());
}
