extern crate alpha_beta_pruning_rust;
use alpha_beta_pruning_rust::{action, state::State};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    // random_vs_random();
    alpha_beta_vs_random();
    // mini_max_vs_random();
    let duration = start.elapsed();

    println!("Time elapsed in Alpha beta is: {:?}", duration);
}

/** alpha beta ai vs random ai */
fn alpha_beta_vs_random() {
    let mut state = State::new(None, None);

    loop {
        if state.is_done() {
            break;
        }

        let action = if state.is_first_player() {
            action::alpha_beta(&state)
        } else {
            action::random(&state)
        };

        state = state.next(action);

        println!("{}", state);
    }
}

/** alpha beta ai vs random ai */
fn mini_max_vs_random() {
    let mut state = State::new(None, None);

    loop {
        if state.is_done() {
            break;
        }

        let action = if state.is_first_player() {
            action::mini_max(&state)
        } else {
            action::random(&state)
        };

        state = state.next(action);

        println!("{}", state);
    }
}

/** random ai vs random ai */
fn random_vs_random() {
    let mut state = State::new(None, None);

    loop {
        if state.is_done() {
            break;
        }

        let action = action::random(&state);
        state = state.next(action);

        println!("{}", state);
    }
}
