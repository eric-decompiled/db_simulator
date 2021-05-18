extern crate dotenv;
use crate::db::insert_event;
use crate::utils::random_time;
use postgres::Client;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::SystemTime;

/// Events stores event types as keys whose values are a collection of field names associated to their type.
pub type Events = HashMap<String, HashMap<String, String>>;
type EventTypeMap = HashMap<String, HashMap<String, Value>>;

/// Reads in a file specified by `$TYPE_FILE` into a representation of the events
pub fn parse_type_map() -> Events {
    let type_file = env::var("TYPE_FILE").expect("TYPE_FILE must be specified");
    let file = fs::read_to_string(type_file).expect("Unable to read type mapping file");
    let type_map: EventTypeMap = serde_json::from_str(&file).expect("Invalid type mapping file");
    let mut events: Events = HashMap::new();
    for (event_type, raw_fields) in type_map {
        let mut fields: HashMap<String, String> = HashMap::new();
        match &raw_fields["type_mapping"] {
            Value::String(v) => println!("e was a string {:?}", v),
            Value::Object(v) => {
                for (key, val) in v {
                    match val {
                        Value::String(type_str) => {
                            fields.insert(key.to_string(), type_str.to_string());
                        }
                        _ => {
                            println!("[WARN] Ignoring non string value in type mapping.")
                        }
                    }
                }
            }
            _ => println!("Unrecognized field in type mapping"),
        }
        events.insert(event_type, fields);
    }
    events
}

pub fn random_event_type<R: Rng>(events: &Events, rng: &mut R) -> String {
    let i = rng.gen_range(0..events.len());
    events
        .keys()
        .skip(i)
        .next()
        .expect("could not get random event type")
        .to_string()
}

pub fn insert_random_event<R: Rng>(
    db_client: &mut Client,
    events: &Events,
    rng: &mut R,
) -> Result<(String, Value), postgres::error::Error> {
    let (event_type, payload) = random_event(&events, rng);
    let time = random_time(rng);
    insert_event(db_client, &event_type, &payload, &time)?;
    Ok((event_type, payload))
}

/// Generates a random event with random values
fn random_event<R: Rng>(events: &Events, rng: &mut R) -> (String, Value) {
    let rand_key = random_event_type(events, rng);
    let rand_event = events.get(&rand_key).expect("Could not find event type");
    let mut payload = HashMap::new();
    for (field_name, field_type) in rand_event {
        let val = match &field_type[..] {
            "int" => json!(rng.gen::<u32>()),
            "bigint" => json!(rng.gen::<u64>()),
            "timestamp" => json!(random_time(rng)),
            "text" => json!("Placeholder text"),
            "boolean" => json!(rng.gen::<bool>()),
            _ => {
                println!("WARN unrecgonized type for {}", field_name);
                json!(null)
            }
        };
        // time is a toplevel field on the model, so skip inserting it into the payload
        if field_name != "time" {
            payload.insert(field_name.to_string(), val);
        }
    }
    (rand_key.to_string(), json!(payload))
}

/// A method to generate the same event everytime this is called. Used to demonstrate events must be unique.
pub fn insert_fixed_event(
    db_client: &mut Client,
) -> Result<(String, Value), postgres::error::Error> {
    let mut payload = HashMap::new();
    let event_type = "mint_coins";
    payload.insert("amount", 100);
    payload.insert("account_id", 7);
    let serialized_payload = json!(payload);
    insert_event(
        db_client,
        event_type,
        &serialized_payload,
        &SystemTime::UNIX_EPOCH,
    )?;
    Ok((event_type.to_string(), serialized_payload))
}
