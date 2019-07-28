use super::{AppState, Color};

pub fn handle_change_color(
    color: &String,
    current_state: AppState,
) -> AppState {
    if color == "BLUE" {
        AppState { color: Color::Blue, ..current_state }

    } else if color == "RED" {
        AppState { color: Color::Red, ..current_state }

    } else if color == "YELLOW" {
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
    let is_daytime = if is_daytime_bool == "true" { true } else { false };

    AppState {
        is_daytime, ..current_state
    }
}

// pub fn handle_change_number_increment(
//     number_as_string: &String,
//     current_state: AppState,
// ) -> AppState {
//     let number: i32 = num.parse().unwrap();

//     if direction == "UP" {
//         let new_number = current_state.number + number;

//         AppState { number: new_number, ..current_state }

//     } else if direction == "DOWN" {
//         let new_number = current_state.number - number;

//         AppState { number: new_number, ..current_state }

//     } else {
//         AppState { ..current_state }
//     }
// }
