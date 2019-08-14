use super::{AppState, Color};

// colors
const BLUE: &str ="BLUE";
const RED: &str = "RED";
const YELLOW: &str = "YELLOW";

// mutation types
const CHOOSE: &str = "Choose your desired mutation by number: ";
const CHANGE_EDITED_BY: &str = "CHANGE_EDITED_BY";
const CHANGE_IS_DAYTIME: &str = "CHANGE_IS_DAYTIME";
const CHANGE_COLOR: &str = "CHANGE_COLOR";
const CHANGE_NUMBER_INCREMENT: &str = "CHANGE_NUMBER_INCREMENT";
const CHANGE_NUMBER_DECREMENT: &str = "CHANGE_NUMBER_DECREMENT";


pub fn handle_change_color(
    color: &String,
    current_state: AppState,
) -> AppState {
    if color == BLUE {
        AppState { color: Color::Blue, ..current_state }

    } else if color == RED {
        AppState { color: Color::Red, ..current_state }

    } else if color == YELLOW {
        AppState { color: Color::Yellow, ..current_state }

    } else {
        println!("SOMETHING_DEFAULTED!!!");
        AppState { ..current_state }
    }
}

pub fn handle_change_edited_by(
    edited_by: &String,
    current_state: AppState,
) -> AppState {
    AppState {
        edited_by: edited_by.to_string(),
        ..current_state
    }
}

pub fn handle_change_is_daytime(
    is_daytime_bool: &String,
    current_state: AppState,
) -> AppState {
    let is_daytime = if is_daytime_bool == "true"
        { true }
        else { false };

    AppState {
        is_daytime, ..current_state
    }
}

pub fn handle_change_number_increase(
    number_as_string: &String,
    current_state: AppState,
) -> AppState {
    let number: i32 = number_as_string.parse().unwrap();
    let new_number = current_state.number + number;

    AppState { number: new_number, ..current_state }
}

pub fn handle_change_number_decrease(
    number_as_string: &String,
    current_state: AppState,
) -> AppState {
    let number: i32 = number_as_string.parse().unwrap();
    let new_number = current_state.number - number;

    AppState { number: new_number, ..current_state }
}

pub fn print_mutation_types() -> () {
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

pub fn get_mutation_type(mtype: &str) -> &str {
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
