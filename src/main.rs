use std::{
    io,
};

mod app;

const CHOOSE: &str = "Choose your desired mutation by number: ";
const CHANGE_EDITED_BY: &str = "1. CHANGE_EDITED_BY";
const CHANGE_IS_DAYTIME: &str = "2. CHANGE_IS_DAYTIME";
const CHANGE_COLOR: &str = "3. CHANGE_COLOR";
const CHANGE_NUMBER_INCREMENT: &str = "4. CHANGE_NUMBER_INCREMENT";
const CHANGE_NUMBER_DECREMENT: &str = "5. CHANGE_NUMBER_DECREMENT";

fn print_mutation_types() -> () {
    let prompts = vec![
        "Choose your desired mutation by number: ",
        "1. CHANGE_EDITED_BY",
        "2. CHANGE_IS_DAYTIME",
        "3. CHANGE_COLOR",
        "4. CHANGE_NUMBER_INCREMENT",
        "5. CHANGE_NUMBER_DECREMENT",
    ];

    for p in &prompts {
        println!("{}", p);
    }
}

fn get_mutation_type(mtype: &String) -> &str {
    match mtype.as_ref() {
        // fn usize(number: &str) -> usize { number.trim().pa}
        "1" => {
            CHOOSE
        }
        "2" => {
            CHANGE_EDITED_BY
        }
        "3" => {
            CHANGE_IS_DAYTIME
        }
        "4" => {
            CHANGE_COLOR
        }
        "5" => {
            CHANGE_NUMBER_INCREMENT
        }
        "6" => {
            CHANGE_NUMBER_DECREMENT
        }
        "EXIT" => {
            "byeeee"
        }
        _ => {
            ""
        }
    }
}

fn main() {
    let state = app::AppState::new();

    println!("\n INITIAL_STATE IS {:?}\n", state);

    print_mutation_types();
    // creates receiver variable
    let mut choice = String::new();

    let mtype: String = io::stdin().read_line(&mut choice)
        .unwrap()
        .to_string();

    let mutation_type: &str = get_mutation_type(&mtype);

    println!("You chose {}.\nNow choose your desired mutation: ", mutation_type);

    let mutation_value: String = io::stdin().read_line(&mut choice)
        .unwrap()
        .to_string();

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
