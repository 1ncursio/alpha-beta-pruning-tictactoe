extern crate alpha_beta_pruning_rust;
use alpha_beta_pruning_rust::{action, state::State};
use std::{thread, time::Instant};

fn main() {
    let num: u64 = 1000000;
    let start = Instant::now();
    let result = thread::Builder::new()
        .stack_size(num as usize * 0xFF)
        .spawn(move || {
            alpha_beta_vs_random();
        })
        .unwrap()
        .join();
    // random_vs_random();
    // mini_max_vs_random();
    match result {
        Ok(()) => println!("Time elapsed in Alpha beta is: {:?}", start.elapsed()),
        Err(e) => println!("{:?}", e),
    }

    // println!("Time elapsed in Alpha beta is: {:?}", start.elapsed());
}

/** alpha beta ai vs random ai */
fn alpha_beta_vs_random() {
    let mut state = State::new(None, None, 0);

    loop {
        if state.is_done() {
            break;
        }

        println!("start loop");

        let action = if state.is_first_player() {
            action::alpha_beta(&mut state)
        } else {
            action::random(&mut state)
        };

        println!("{}", action);

        state = state.next(action);

        println!("{}", state);
    }
}

/** alpha beta ai vs random ai */
pub fn mini_max_vs_random() {
    let mut state = State::new(None, None, 0);

    loop {
        if state.is_done() {
            break;
        }

        let action = if state.is_first_player() {
            action::mini_max(&mut state)
        } else {
            action::random(&mut state)
        };

        state = state.next(action);

        println!("{}", state);
    }
}

/** random ai vs random ai */
pub fn random_vs_random() {
    let mut state = State::new(None, None, 0);

    loop {
        if state.is_done() {
            break;
        }

        let action = action::random(&mut state);
        state = state.next(action);

        println!("{}", state);
    }
}
