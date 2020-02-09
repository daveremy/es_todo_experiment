#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate eventsourcing_derive;

use eventsourcing::eventstore::OrgEventStore;
use eventsourcing::prelude::*;
use uuid::Uuid;

mod domain;
use domain::{TodoCommand, TodoDispatcher, TodoState};

fn main() {
    let todo_store = OrgEventStore::new("localhost", 2113);
    let todo_id = Uuid::new_v4();
    let initial_state = TodoState {
        id: todo_id,
        generation: 0,
    };

    let add_todo = TodoCommand::Add(todo_id, "Take out the garbage.".to_owned(), 1);

    let res =
        TodoDispatcher::dispatch(&initial_state, &add_todo, &todo_store, &todo_id.to_string());
    println!("dispatch results - {:#?}", res);
}
