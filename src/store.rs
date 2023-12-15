#[derive(Clone, Default)]
pub struct State {
    counter: isize,
}

impl State {
    pub fn get_counter(&self) -> isize {
        self.counter
    }
}

#[derive(Clone)]
pub enum Action {
    Increment(isize)
}

#[derive(Default)]
pub struct AppStore {
    actions: Vec<Action>,
    pub state: State
}

impl AppStore {
    pub fn reduce(&mut self) {
        let mut state = self.state.clone();
        let actions = self.actions.clone();
        for action in actions {
            match action {
                Action::Increment(amount) => {
                    let new_amount = state.counter.checked_add(amount);
                    if let Some(new_amount) = new_amount {
                        state.counter = new_amount;
                    }
                },
            }
        }
        self.state = state;
        self.actions.clear();
    }

    pub fn dispatch(&mut self, action: Action) {
        self.actions.push(action);
    }
}