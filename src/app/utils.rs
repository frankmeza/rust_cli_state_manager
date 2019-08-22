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
const EXIT: &str = "EXIT";
const DEFAULT_ERROR: &str = "ERROR";


pub fn handle_change_color(
    color: &String,
    current_state: AppState,
) -> AppState {
    // input needs to be all caps, i.e. "BLUE" | "RED" | "YELLOW"
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
        { true } else { false };

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
        "\npress 1 to change a boolean:",
        CHANGE_IS_DAYTIME,
        "\npress 2 to change an enum:",
        CHANGE_COLOR,
        "\npress 3 to increment by your number:",
        CHANGE_NUMBER_INCREMENT,
        "\npress 4 to decrement by your number:",
        CHANGE_NUMBER_DECREMENT,
        "\npress 5 to change the edited_by field:",
        CHANGE_EDITED_BY,
        "\n EXIT to exit",
        EXIT,
    ];

    for p in &prompts {
        println!("{}", p);
    }
}

pub fn get_mutation_type(mtype: &str) -> &str {
    match mtype {
        "1" => {
            CHANGE_IS_DAYTIME
        }
        "2" => {
            CHANGE_COLOR
        }
        "3" => {
            CHANGE_NUMBER_INCREMENT
        }
        "4" => {
            CHANGE_NUMBER_DECREMENT
        }
        "5" => {
            CHANGE_EDITED_BY
        }
        "EXIT" => {
            EXIT
        }
        _ => {
            DEFAULT_ERROR
        }
    }
}
