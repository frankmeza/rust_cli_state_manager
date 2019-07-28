// use crate::app_state::mutation::Mutation;

mod mutation;
mod utils;

use mutation::Mutation;

#[derive(Debug)]
pub enum Color {
    Blue,
    Red,
    Yellow,
}

#[derive(Debug)]
pub struct AppState {
    pub color: Color,
    pub number: i32,
    pub is_daytime: bool,
    pub edited_by: String,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            color: Color::Blue,
            is_daytime: true,
            edited_by: String::from("fraaank"),
            number: 0,
        }
    }

    pub fn receive_mutation(current_state: Self, mutation: &Mutation) -> AppState {
        match mutation {
            // change color
            Mutation::ChangeColor(color) => utils::handle_change_color(color, current_state),

            // change edited_by
            Mutation::ChangeEditedBy(edited_by) => AppState {
                edited_by: edited_by.to_string(), ..current_state
            },

            // change is_daytime
            Mutation::ChangeIsDayTime(value) => {
                let is_daytime = if value == "true" { true } else { false };

                AppState {
                    is_daytime, ..current_state
                }
            },

            // change number
            Mutation::ChangeNumber(num, direction) => {
                 let number: i32 = num.parse().unwrap();

                 if direction == "UP" {
                     let new_number = current_state.number + number;

                     AppState { number: new_number, ..current_state }

                 } else if direction == "DOWN" {
                     let new_number = current_state.number - number;

                     AppState { number: new_number, ..current_state }

                 } else {
                     AppState { ..current_state }
                 }
             },

            // default
            Mutation::Nothing() => AppState { ..current_state }
        }
    }

    pub fn create_mutation(mutation_type: &String, mutation_value: &String) -> Mutation {
        let value = mutation_value.to_string();

        let mutation = if mutation_type == "CHANGE_EDITED_BY" {
            Mutation::ChangeEditedBy(value)

        } else if mutation_type == "CHANGE_IS_DAYTIME" {
            Mutation::ChangeIsDayTime(value)

        } else if mutation_type == "CHANGE_COLOR" {
            Mutation::ChangeColor(value)

        } else if mutation_type == "CHANGE_NUMBER" {
            let true_or_false = true;

            if true_or_false {
                Mutation::ChangeNumber(value, String::from("UP"))
            } else {
                Mutation::ChangeNumber(value, String::from("DOWN"))
            }

        } else {
            Mutation::Nothing()
        };

        mutation
    }
}
