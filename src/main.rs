extern crate alpha_beta_pruning_rust;
use alpha_beta_pruning_rust::state::State;
use rand::Rng;

fn main() {
    let mut state = State::new(None, None);

    loop {
        if state.is_done() {
            break;
        }

        let action = random_action(&state);
        state = state.next(action);

        println!("{}", state);
    }
}

fn random_action(state: &State) -> u8 {
    let legal_actions = state.legal_actions();
    let action = rand::thread_rng().gen_range(0..legal_actions.len());

    legal_actions[action]
}
