use super::{AppState, Color};

// colors
const BLUE: &str = "BLUE";
const RED: &str = "RED";
const YELLOW: &str = "YELLOW";

// mutation types
const CHOOSE: &str = "Choose your desired mutation by number: ";
const CHANGE_EDITED_BY: &str = "CHANGE_EDITED_BY";
const CHANGE_IS_DAYTIME: &str = "CHANGE_IS_DAYTIME";
const CHANGE_COLOR: &str = "CHANGE_COLOR";
const CHANGE_NUMBER_INCREMENT: &str = "CHANGE_NUMBER_INCREMENT";
const CHANGE_NUMBER_DECREMENT: &str = "CHANGE_NUMBER_DECREMENT";
const EXIT: &str = "EXIT";
const DEFAULT_ERROR: &str = "ERROR";

pub fn handle_change_color(color: &str, current_state: AppState) -> AppState {
    match color {
        BLUE => AppState {
            color: Color::Blue,
            ..current_state
        },
        RED => AppState {
            color: Color::Red,
            ..current_state
        },
        YELLOW => AppState {
            color: Color::Yellow,
            ..current_state
        },
        _ => {
            println!("SOMETHING_DEFAULTED!!!");
            AppState { ..current_state }
        }
    }
}

pub fn handle_change_edited_by(edited_by: &String, current_state: AppState) -> AppState {
    AppState {
        edited_by: edited_by.to_string(),
        ..current_state
    }
}

pub fn handle_change_is_daytime(is_daytime_bool: &String, current_state: AppState) -> AppState {
    let is_daytime = if is_daytime_bool == "true" {
        true
    } else {
        false
    };

    AppState {
        is_daytime,
        ..current_state
    }
}

pub fn handle_change_number_increase(
    number_as_string: &String,
    current_state: AppState,
) -> AppState {
    let number: i32 = number_as_string.parse().unwrap();
    let new_number = current_state.number + number;

    AppState {
        number: new_number,
        ..current_state
    }
}

pub fn handle_change_number_decrease(
    number_as_string: &String,
    current_state: AppState,
) -> AppState {
    let number: i32 = number_as_string.parse().unwrap();
    let new_number = current_state.number - number;

    AppState {
        number: new_number,
        ..current_state
    }
}

pub fn print_mutation_types() -> () {
    let prompts = vec![
        CHOOSE,
        "press 1 to change a boolean:",
        CHANGE_IS_DAYTIME,
        "press 2 to change an enum:",
        CHANGE_COLOR,
        "press 3 to increment by your number :",
        CHANGE_NUMBER_INCREMENT,
        "press 4 to decrement by your number :",
        CHANGE_NUMBER_DECREMENT,
        "press 5 to change the edited_by field:",
        CHANGE_EDITED_BY,
        "EXIT to exit",
        EXIT,
    ];

    for p in &prompts {
        println!("{}", p);
    }
}

pub fn get_mutation_type(mutation_type: &str) -> &str {
    match mutation_type {
        "1" => CHANGE_EDITED_BY,
        "2" => CHANGE_IS_DAYTIME,
        "3" => CHANGE_COLOR,
        "4" => CHANGE_NUMBER_INCREMENT,
        "5" => CHANGE_NUMBER_DECREMENT,
        "EXIT" => EXIT,
        _ => DEFAULT_ERROR,
    }
}
