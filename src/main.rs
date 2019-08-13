use std::{
    io,
};

mod app;

const CHOOSE: &str = "Choose your desired mutation by number: ";
const CHANGE_EDITED_BY: &str = "CHANGE_EDITED_BY";
const CHANGE_IS_DAYTIME: &str = "CHANGE_IS_DAYTIME";
const CHANGE_COLOR: &str = "CHANGE_COLOR";
const CHANGE_NUMBER_INCREMENT: &str = "CHANGE_NUMBER_INCREMENT";
const CHANGE_NUMBER_DECREMENT: &str = "CHANGE_NUMBER_DECREMENT";

fn print_mutation_types() -> () {
    let prompts = vec![
        CHOOSE,
        "press 1 for :",
        CHANGE_EDITED_BY,
        "press 2 for :",
        CHANGE_IS_DAYTIME,
        "press 3 for :",
        CHANGE_COLOR,
        "press 4 for :",
        CHANGE_NUMBER_INCREMENT,
        "press 5 for :",
        CHANGE_NUMBER_DECREMENT,
    ];

    for p in &prompts {
        println!("{}", p);
    }
}

fn get_mutation_type(mtype: &str) -> &str {
    match mtype {
        "1" => {
            CHANGE_EDITED_BY
        }
        "2" => {
            CHANGE_IS_DAYTIME
        }
        "3" => {
            CHANGE_COLOR
        }
        "4" => {
            CHANGE_NUMBER_INCREMENT
        }
        "5" => {
            CHANGE_NUMBER_DECREMENT
        }
        "EXIT" => {
            "byeeee"
        }
        _ => {
            "very virus"
        }
    }
}

fn main() {
    let state = app::AppState::new();

    println!("\n INITIAL_STATE IS {:?}\n", state);
    print_mutation_types();

    // creates receiver variable
    let mut c1 = String::new();
    io::stdin().read_line(&mut c1)
        .unwrap()
        .to_string();

    let choice_type: &str = c1.as_str().trim();
    let mutation_type: &str = get_mutation_type(&choice_type);

    println!("You chose {}.\nNow choose your desired mutation: ", mutation_type);

    // creates a second receiver variable
    let mut c2 = String::new();
    io::stdin().read_line(&mut c2)
        .unwrap()
        .to_string();

    let mutation_value: &str = c2.as_str().trim();
    println!("mutation value : {}", mutation_value);

    let new_mutation = app::AppState::create_mutation(
        &mutation_type,
        &mutation_value,
    );

    println!("\n MUTATION IS {:?}", new_mutation);

    let updated_state = app::AppState::receive_mutation(
        state,
        &new_mutation,
    );

    println!("\n UPDATED STATE IS {:?}", updated_state);
}
