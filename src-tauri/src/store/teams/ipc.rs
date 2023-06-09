use std::sync::{Mutex, Arc};

use serde_json::Value;
use tauri_plugin_store::JsonValue;
use tauri::{State, Window, Manager};

use crate::store::{AllStores, is_data_locked};

use super::TeamPlayer;

// Tauri command for creating a team.
#[tauri::command]
pub fn teams__create_team(
    window: Window,
    stores: State<'_, Arc<Mutex<AllStores>>>, 
    name: &str,
    events_ids_entered: Vec<u64>
) -> Result<Value, std::string::String> {
    // Checks if the data is locked before callingg the API.
    match is_data_locked() {
        true => {
            return Err("Unable to make any more changes. Data is locked.".to_string())
        },
        false => {}
    }

    match stores.lock().unwrap().teams.create_team(name, events_ids_entered) {
        Ok(res) => {
            // If the result was OK we fire an event to the frontend to say a team was created
            window.emit_all("teams__on_team_created", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
}

// Tauri command for adding a player to a team
#[tauri::command]
pub fn teams__add_player_to_team(
    window: Window,
    stores: State<'_, Arc<Mutex<AllStores>>>, 
    id: u64,
    player: TeamPlayer
) -> Result<(), std::string::String> {
    // Checks if the data is locked before calling the API.
    match is_data_locked() {
        true => {
            return Err("Unable to make any more changes. Data is locked.".to_string())
        },
        false => {}
    }

    // Call the API
    match stores.lock().unwrap().teams.add_player_to_team(id, player) {
        Ok(res) => {
            // If the result was OK we fire an event to the frontend to say a player was added to a team
            window.emit_all("teams__on_team_player_added", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
}

// Tauri command for removing a player from a team
#[tauri::command]
pub fn teams__remove_player_from_team(
    window: Window,
    stores: State<'_, Arc<Mutex<AllStores>>>, 
    team_id: u64,
    player_id: u64
) -> Result<(), std::string::String> {
    // Checks if the data is locked before calling the API.
    match is_data_locked() {
        true => {
            return Err("Unable to make any more changes. Data is locked.".to_string())
        },
        false => {}
    }

    // Call the API
    match stores.lock().unwrap().teams.remove_player_from_team(team_id, player_id) {
        Ok(res) => {
            // If the result was OK we fire an event to the frontend to say a player was removed from a team
            window.emit_all("teams__on_team_player_deleted", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
}

// Tauri command for editing a team's events
#[tauri::command]
pub fn teams__edit_events(
    window: Window,
    stores: State<'_, Arc<Mutex<AllStores>>>, 
    team_id: u64,
    events_ids_entered: Vec<u64>
) -> Result<(), std::string::String> {
    // Checks if the data is locked before calling the API.
    match is_data_locked() {
        true => {
            return Err("Unable to make any more changes. Data is locked.".to_string())
        },
        false => {}
    }

    // Call the API
    match stores.lock().unwrap().teams.edit_events(team_id, events_ids_entered) {
        Ok(res) => {
            // Just fire the team created event here as it will update the frontend regardless
            window.emit_all("teams__on_team_created", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
}

// Tauri command for getting all the teams from the store
#[tauri::command]
pub fn teams__get_all_teams(
    stores: State<'_, Arc<Mutex<AllStores>>>, 
) -> Vec<JsonValue> {
    // Don't need to check if its locked as this is a read only command

    stores.lock().unwrap().teams.get_all_teams()
}

// Tauri command for deleting a team
#[tauri::command]
pub fn teams__delete_team(
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

    // Call the API
    match stores.lock().unwrap().teams.delete_team(id) {
        Ok(res) => {
            // If the result was OK we fire an event to the frontend to say a team was deleted
            window.emit_all("teams__on_team_deleted", res.clone()).expect("Failed to dispatch event");
            Ok(res.clone())
        }, 
        Err(err) => {
            Err(err)
        }
    }
}
