use serde_json::json;
use serde_json::Value;
use tauri::{Wry};
use tauri_plugin_store::{ Store, JsonValue };

use super::EventType;

pub struct EventsStore {
    pub store: Store<Wry>
}

// The EventsStore implementation
impl EventsStore {
    // Helper function for saving the store
    pub fn save(&mut self) -> () {
        self.store.save().expect("Failed to save EventsStore");
    }

    // Inits the store and its default keys
    pub fn init(&mut self) -> () {
        if !self.store.has("events") {
            self.store.insert("events".to_string(), json!([]))
                .expect("Failed to create default empty array in EventsStore");
        }

        self.save();
    }

    // Gets all the events from the store
    pub fn get_all_events(&mut self) -> Vec<JsonValue> {
        let all_events = &mut self.store.get("events").unwrap().as_array().cloned().unwrap();

        all_events.clone()
    }

    // Finds an event using a predicate
    pub fn find_event_by<T: for<'a> FnMut(&'a &JsonValue) -> bool>(&mut self, mut predicate: T) -> Result<JsonValue, &str> {
        let all_events = self.get_all_events();

        let binding = all_events.to_vec();
        let result = binding.iter().clone().find(&mut predicate);

        match result {
            Some(event) => Ok(event.to_owned().to_owned()),
            None => Err("Failed to find event using predicate"),
        }
    }

    // Finds an event using its ID
    pub fn get_event_by_id(&mut self, id: u64) -> Result<JsonValue, String> {
        match self.find_event_by(|x| x.get("id")
            .expect("Failed to get ID in get_event_by_id for iterator")
            .as_u64()
            .unwrap() == id
        ) {
            Ok(event) => Ok(event.to_owned().to_owned()),
            Err(_) => Err(format!("No event found with ID {}", id)),
        }
    }

    // Creates a new event using a name, kind, max points, and max teams
    pub fn create_event(&mut self, name: &str, kind: EventType, max_points: u16, max_teams: Option<u16>) -> Result<Value, String> {
        let all_events = &mut self.get_all_events();

        match self.find_event_by(|x| x.get("name")
            .expect("Failed to get name for create_event existing check")
            .as_str()
            .unwrap() == name
        ) {
            Ok(_) => {
                return Err(format!("Event with name '{}' already exists.", name))
            },
            _ => {}
        };

        let _max_teams: Option<u16> = match max_teams.is_some() {
            true => match kind == EventType::Team {
                true => Some(max_teams.unwrap()),
                false => None
            },
            false => None
        };

        let data = json!({
            "id": all_events.len() + 1,
            "name": name,
            "kind": kind,
            "max_points": max_points,
            "max_teams": _max_teams
        });

        all_events.push(data.clone());

        let _ = &self.store.insert("events".to_string(), serde_json::to_value(all_events).unwrap());

        self.save();

        Ok(data)
    }

    // Deletes an event using its ID
    pub fn delete_event(&mut self, id: u64) -> Result<(), String> {
        let all_events = &mut self.get_all_events().to_owned();

        match self.find_event_by(|x| x.get("id")
            .expect("Failed to get id for delete_event existing check")
            .as_u64()
            .unwrap() == id
        ) {
            Ok(_) => {},
            _ => {
                return Err(format!("Event with ID '{}' does not exist.", id))
            }
        };

        let filtered: Vec<&JsonValue> = all_events
            .iter()
            .filter(|x| x
                .get("id")
                .expect("Failed to get ID in iterator")
                .as_u64()
                .expect("Failed to cast as u64") != id
            ).collect::<_>();

        let _ = &self.store.insert("events".to_string(), serde_json::to_value(filtered).unwrap());

        self.save();

        Ok(())
    }
}