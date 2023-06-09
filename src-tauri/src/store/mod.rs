use std::{path::PathBuf};

use tauri::{App, api::path::home_dir, Wry};
use tauri_plugin_store::{StoreBuilder, Store};

use crate::store::events::api::EventsStore;
use crate::store::individuals::api::IndividualsStore;
use crate::store::teams::api::TeamsStore;

use self::results::api::ResultsStore;

pub mod events;
pub mod individuals;
pub mod teams;
pub mod results;

// Helper function for getting the data directory for the apps config files
pub fn get_data_dir() -> PathBuf {
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

fn init_individuals_store(application: &App) -> Store<Wry> {
    let mut store = StoreBuilder::new(
        application.handle(), 
        get_data_dir().join(PathBuf::from("individuals.store.json"))
    ).build();

    match store.load() {
        Ok(_) => {},
        Err(err) => {
            println!("Error {}", err)
        }
    }
    store.save().expect("Failed to save individuals store");

    store
}

fn init_results_store(application: &App) -> Store<Wry> {
    let mut store = StoreBuilder::new(
        application.handle(), 
        get_data_dir().join(PathBuf::from("results.store.json"))
    ).build();

    match store.load() {
        Ok(_) => {},
        Err(err) => {
            println!("Error {}", err)
        }
    }
    store.save().expect("Failed to save results store");

    store
}

// Helper function to check if the data lock exists.
pub fn is_data_locked() -> bool {
    let lock_file = get_data_dir().join(PathBuf::from("data.lock"));

    lock_file.exists()
}

pub struct AllStores {
    pub events: EventsStore,
    pub teams: TeamsStore,
    pub individuals: IndividualsStore,
    pub results: ResultsStore
}

// Initialises all the data stores in the application and creates a struct instance to hold them.
// Each init_x_store function will load the data from the disk and deserialise it into a API-understandable format.
// The stores are then stored into a main struct called AllStores.
pub fn init(application: &App) -> AllStores {
    let mut events = EventsStore { store: init_events_store(application) };
    let mut teams = TeamsStore { store: init_teams_store(application) };
    let mut individuals = IndividualsStore { store: init_individuals_store(application) };
    let mut results = ResultsStore { store: init_results_store(application) };

    events.init();
    teams.init();
    individuals.init();
    results.init();

    AllStores { 
        events,
        teams,
        individuals,
        results
    }
}