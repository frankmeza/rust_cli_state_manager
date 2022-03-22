use std::io;

mod app;
use app::utils;

fn main() {
    let mut state = app::AppState::new();
    println!("\n INITIAL_STATE IS {:?}\n", state);

    loop {
        utils::print_mutation_types();

        // creates receiver variable for type
        let mut choice_one = String::new();
        io::stdin().read_line(&mut choice_one).unwrap().to_string();

        let choice_type: &str = choice_one.as_str().trim();
        let mutation_type: &str = utils::get_mutation_type(&choice_type);

        println!(
            "You chose {}.\nNow choose your desired mutation: ",
            mutation_type
        );

        // creates a second receiver variable for value
        let mut choice_two = String::new();
        io::stdin().read_line(&mut choice_two).unwrap().to_string();

        let mutation_value: &str = choice_two.as_str().trim();
        println!("mutation value : {}", mutation_value);

        let new_mutation = app::AppState::create_mutation(&mutation_type, &mutation_value);

        println!("\n MUTATION IS {:?}", new_mutation);

        let updated_state = app::AppState::receive_mutation(state, &new_mutation);

        state = updated_state;

        println!("\n UPDATED STATE IS {:?}\n", state);
    }
}
