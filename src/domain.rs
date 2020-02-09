const DOMAIN_VERSION: &str = "1.0";

use eventsourcing::{Aggregate, AggregateState, Result};
use uuid::Uuid;

#[derive(Debug)]
pub enum TodoCommand {
    Add(Uuid, String, u8),
}

#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://github.com/daveremy/todo")]
#[derive(Serialize, Deserialize, Debug, Clone, Event)]
pub enum TodoEvent {
    TodoAdded(Uuid, String, u8),
}

#[derive(Debug, Clone)]
pub struct TodoState {
    // State needed to enforce invariants
    pub id: Uuid,
    pub generation: u64,
}

impl AggregateState for TodoState {
    fn generation(&self) -> u64 {
        self.generation
    }
}

pub struct Todo;

impl Aggregate for Todo {
    type Event = TodoEvent;
    type Command = TodoCommand;
    type State = TodoState;

    fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State> {
        let todo_state = match &*evt {
            TodoEvent::TodoAdded(id, _text, _priority) => TodoState {
                id: *id,
                generation: state.generation + 1,
            },
        };
        Ok(todo_state)
    }

    fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
        // SHOULD DO: validate state and command
        let evt = match &*cmd {
            TodoCommand::Add(id, text, priority) => {
                TodoEvent::TodoAdded(*id, text.to_string(), *priority)
            }
        };

        // if validation passes...
        Ok(vec![evt])
    }
}

#[derive(Dispatcher)]
#[aggregate(Todo)]
pub struct TodoDispatcher;
