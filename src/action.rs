use crate::state::State;
use rand::Rng;

pub fn random(state: &State) -> u8 {
    let legal_actions = state.legal_actions();
    let action = rand::thread_rng().gen_range(0..legal_actions.len());

    legal_actions[action]
}

pub fn alpha_beta(state: &State) -> u8 {
    let mut best_action = 0;
    let mut alpha = -1;
    let mut str = [String::from(""), String::from("")];

    for action in state.legal_actions() {
        let score = -alpha_beta_evaluate(&state.next(action), -1, -alpha);
        if score > alpha {
            best_action = action;
            alpha = score;
        }
        str[0] = format!("{} {},", str[0], action);
        str[1] = format!("{} {},", str[1], score);
    }
    println!("action: {}", str[0]);
    println!("score: {}", str[1]);
    println!("\n");

    best_action
}

fn alpha_beta_evaluate(state: &State, mut alpha: i8, beta: i8) -> i8 {
    if state.is_lose() {
        return -1;
    }

    if state.is_draw() {
        return 0;
    }

    for action in state.legal_actions() {
        let score = -alpha_beta_evaluate(&state.next(action), -beta, -alpha);
        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            return alpha;
        }
    }

    alpha
}

pub fn mini_max(state: &State) -> u8 {
    let mut best_action = 0;
    let mut best_score = -1;
    let mut str = [String::from(""), String::from("")];

    for action in state.legal_actions() {
        let score = -mini_max_evaluate(&state.next(action));
        if score > best_score {
            best_action = action;
            best_score = score;
        }
        str[0] = format!("{} {},", str[0], action);
        str[1] = format!("{} {},", str[1], score);
    }
    println!("action: {}", str[0]);
    println!("score: {}", str[1]);
    println!("\n");

    best_action
}

fn mini_max_evaluate(state: &State) -> i8 {
    if state.is_lose() {
        return -1;
    }

    if state.is_draw() {
        return 0;
    }

    let mut best_score = -1;

    for action in state.legal_actions() {
        let score = -mini_max_evaluate(&state.next(action));
        if score > best_score {
            best_score = score;
        }
    }

    best_score
}

fn mini_max_plus_evaluate(state: &State, limit: i8) -> i8 {
    if state.is_lose() {
        return -1;
    }

    if state.is_draw() {
        return 0;
    }

    let mut best_score = -1;

    for action in state.legal_actions() {
        let score = -mini_max_plus_evaluate(&state.next(action), -best_score);
        if score > best_score {
            best_score = score;
        }

        if best_score >= limit {
            return best_score;
        }
    }

    best_score
}
