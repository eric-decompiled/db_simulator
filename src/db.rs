use postgres::error::Error;
use postgres::{Client, NoTls};
extern crate dotenv;
use serde_json::Value;
use sha256::digest;
use std::env;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Event {
    id: i32,
    event_type: String,
    body: Value,
    time: SystemTime,
}

pub fn establish_connection() -> Client {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&db_url, NoTls).expect("Could not connect to the DB")
}

/// idempotently creates needed tables
pub fn init_tables(db_client: &mut Client) -> Result<(), Error> {
    db_client.batch_execute(
        "CREATE TABLE IF NOT EXISTS events (
            id SERIAL PRIMARY KEY,
            type TEXT NOT NULL,
            body JSONB,
            time timestamp,
            fingerprint TEXT NOT NULL UNIQUE
        );
        CREATE INDEX IF NOT EXISTS events_time_type_idx ON events (time, type);",
    )?;
    Ok(())
}

pub fn query_events(
    db_client: &mut Client,
    event_type: &str,
    from: SystemTime,
    to: SystemTime,
) -> Result<Vec<Event>, Error> {
    let mut events = vec![];
    for row in db_client.query(
        "SELECT id, type, body, time 
         FROM events
         WHERE time BETWEEN $1 AND $2
         AND type = $3",
        &[&from, &to, &event_type],
    )? {
        events.push(Event {
            id: row.get(0),
            event_type: row.get(1),
            body: row.get(2),
            time: row.get(3),
        });
    }
    Ok(events)
}

pub fn insert_event(
    db_client: &mut Client,
    event_type: &str,
    payload: &Value,
    time: &SystemTime,
) -> Result<(), Error> {
    let print = finger_print(event_type, payload, time);
    db_client.execute(
        "INSERT INTO events (type, body, time, fingerprint) VALUES ($1, $2, $3, $4)",
        &[&event_type, payload, time, &print],
    )?;
    Ok(())
}

fn finger_print(event_type: &str, payload: &Value, time: &SystemTime) -> String {
    let print = format!("{:?},{:?} {:?}", event_type, payload, time);
    digest(print)
}
