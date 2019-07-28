use std::env;

mod app_state;

fn main() {
    let state = app_state::AppState::new();
    println!("INITIAL_STATE IS {:?}", state);

    let args: Vec<String> = env::args().collect();
    let mutation_type = &args[1].clone();
    let mutation_value = &args[2].clone();

    let new_mutation = app_state::AppState::create_mutation(mutation_type, mutation_value);

    println!("{:?}", new_mutation);
    println!("{}: {}", mutation_type, mutation_value);

    let updated_state = app_state::AppState::receive_mutation(state, &new_mutation);
    println!("{:?}", updated_state);
}

// state is:
// color, blue red, yellow
// number, 1, 10, 100
// shape, circle triangle square
// editor name

// CHANGE_COLOR
// INCREMENT, DECREMENT
// CHANGE_BOOLEAN
// EDITED_BY