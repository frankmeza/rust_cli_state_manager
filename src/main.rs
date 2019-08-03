use std::env;

mod app;

fn main() {
    let state = app::AppState::new();
    println!("\n INITIAL_STATE IS {:?}", state);

    let args: Vec<String> = env::args().collect();
    let mutation_type = &args[1].clone();
    let mutation_value = &args[2].clone();

    let new_mutation = app::AppState::create_mutation(
        mutation_type,
        mutation_value,
    );

    println!("\n MUTATION IS {:?}", new_mutation);

    let updated_state = app::AppState::receive_mutation(
        state,
        &new_mutation,
    );

    println!("\n UPDATED STATE IS {:?}", updated_state);
}
