use std::env;

mod app_state;

fn main() {
    let state = app_state::AppState::new();
    println!("\n INITIAL_STATE IS {:?}", state);

    let args: Vec<String> = env::args().collect();
    let mutation_type = &args[1].clone();
    let mutation_value = &args[2].clone();

    let new_mutation = app_state::AppState::create_mutation(
        mutation_type,
        mutation_value,
    );

    println!("\n MUTATION IS {:?}", new_mutation);


    let updated_state = app_state::AppState::receive_mutation(
        state,
        &new_mutation,
    );

    println!("\n UPDATED STATE IS {:?}", updated_state);
}
