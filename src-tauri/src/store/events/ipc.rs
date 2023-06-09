use std::sync::{Mutex, Arc};

use serde_json::Value;
use tauri_plugin_store::JsonValue;
use tauri::{State, Window, Manager};

use crate::store::{AllStores, is_data_locked};

use super::EventType;

// Tauri command that creates a new event
#[tauri::command]
pub fn events__create_event(
    window: Window,
    stores: State<'_, Arc<Mutex<AllStores>>>, 
    name: &str,
    kind: EventType,
    max_points: u16,
    max_teams: Option<u16>,
) -> Result<Value, std::string::String> {
    // Checks if the data is locked before callingg the API.
    match is_data_locked() {
        true => {
            return Err("Unable to make any more changes. Data is locked.".to_string())
        },
        false => {}
    }

    match stores.lock().unwrap().events.create_event(name, kind, max_points, max_teams) {
        Ok(res) => {
            // If the result was OK we fire an event to the frontend to say an event was created
            window.emit_all("events__on_event_created", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
}

// Tauri command that gets all events
#[tauri::command]
pub fn events__get_all_events(
    stores: State<'_, Arc<Mutex<AllStores>>>
) -> Vec<JsonValue> {
    stores.lock().unwrap().events.get_all_events()
}

// Tauri command that deletes an event
#[tauri::command]
pub fn events__delete_event(
    window: Window,
    stores: State<'_, Arc<Mutex<AllStores>>>,
    id: u64
) -> Result<(), String> {
    // Checks if the data is locked before calling the API.
    match is_data_locked() {
        true => {
            return Err("Unable to make any more changes. Data is locked.".to_string())
        },
        false => {}
    }

    match stores.lock().unwrap().events.delete_event(id) {
        Ok(res) => {
            // If the result was OK we fire an event to the frontend to say an event was deleted
            window.emit_all("events__on_event_deleted", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
    
}