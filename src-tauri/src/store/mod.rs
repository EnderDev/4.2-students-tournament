use std::{path::PathBuf, collections::HashMap, hash::Hash};

use tauri::{App, api::path::home_dir, Wry, EventLoopMessage};
use tauri_plugin_store::{StoreBuilder, Store};

use crate::store::events::api::EventsStore;

use self::teams::TeamsStore;

pub mod events;
pub mod teams;

fn get_data_dir() -> PathBuf {
    home_dir().expect("Failed to find home directory")
        .join(PathBuf::from(".students_tournament"))
}

fn init_events_store(application: &App) -> Store<Wry> {
    let mut store = StoreBuilder::new(
        application.handle(), 
        get_data_dir().join(PathBuf::from("events.store.json"))
    ).build();

    match store.load() {
        Ok(_) => {},
        Err(err) => {
            println!("Error {}", err)
        }
    }
    store.save().expect("Failed to save events store");

    store
}

fn init_teams_store(application: &App) -> Store<Wry> {
    let mut store = StoreBuilder::new(
        application.handle(), 
        get_data_dir().join(PathBuf::from("teams.store.json"))
    ).build();

    match store.load() {
        Ok(_) => {},
        Err(err) => {
            println!("Error {}", err)
        }
    }
    store.save().expect("Failed to save teams store");

    store
}

pub struct AllStores {
    pub events: EventsStore,
    pub teams: TeamsStore
}

pub fn init(application: &App) -> AllStores {
    let mut events = EventsStore { store: init_events_store(application) };
    let mut teams = TeamsStore { store: init_teams_store(application) };

    events.init();
    teams.init();

    AllStores { 
        events,
        teams
    }
}