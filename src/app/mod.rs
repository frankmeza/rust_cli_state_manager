mod mutation;
pub mod utils;

use mutation::Mutation;

const CHANGE_EDITED_BY: &str = "CHANGE_EDITED_BY";
const CHANGE_IS_DAYTIME: &str = "CHANGE_IS_DAYTIME";
const CHANGE_COLOR: &str = "CHANGE_COLOR";
const CHANGE_NUMBER_INCREMENT: &str = "CHANGE_NUMBER_INCREMENT";
const CHANGE_NUMBER_DECREMENT: &str = "CHANGE_NUMBER_DECREMENT";

#[derive(Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    Yellow,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub color: Color,
    pub number: i32,
    pub is_daytime: bool,
    pub edited_by: String,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            edited_by: String::from("fraaank"),
            color: Color::Blue,
            is_daytime: true,
            number: 0,
        }
    }

    pub fn receive_mutation(current_state: Self, mutation: &Mutation) -> AppState {
        match mutation {
            // change color
            Mutation::ChangeColor(color) => {
                utils::handle_change_color(color, current_state)
            }

            // change edited_by
            Mutation::ChangeEditedBy(edited_by) => {
                utils::handle_change_edited_by(edited_by, current_state)
            },

            // change is_daytime
            Mutation::ChangeIsDayTime(value) => {
                utils::handle_change_is_daytime(value, current_state)
            },

            // change number
            Mutation::ChangeNumberIncrease(number_as_string) => {
                utils::handle_change_number_increase(number_as_string, current_state)
            },

            Mutation::ChangeNumberDecrease(number_as_string) => {
                utils::handle_change_number_decrease(number_as_string, current_state)
            }

            // default
            Mutation::NoMutation() => AppState { ..current_state }
        }
    }

    pub fn create_mutation(mutation_type: &str, mutation_value: &str) -> Mutation {
        let value = String::from(mutation_value);

        match mutation_type {
            CHANGE_EDITED_BY => {
                Mutation::ChangeEditedBy(value)
            }
            CHANGE_IS_DAYTIME => {
                Mutation::ChangeIsDayTime(value)
            }
            CHANGE_COLOR => {
                Mutation::ChangeColor(value)
            }
            CHANGE_NUMBER_INCREMENT => {
                Mutation::ChangeNumberIncrease(value)
            }
            CHANGE_NUMBER_DECREMENT => {
                Mutation::ChangeNumberDecrease(value)
            }
            _ => {
                Mutation::NoMutation()
            }
        }
    }
}