use std::env;

#[derive(Debug)]
enum Mutation {
    ChangeEditedBy(String),
    Nothing(),
}

#[derive(Debug)]
enum Color {
    Blue,
}

#[derive(Debug)]
struct AppState {
    color: Color,
    number: i32,
    is_daytime: bool,
    edited_by: String,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            color: Color::Blue,
            is_daytime: true,
            edited_by: String::from("fraaank"),
            number: 0,
        }
    }

    fn receive_mutation(current_state: Self, mutation: &Mutation) -> AppState {
        match mutation {
            Mutation::ChangeEditedBy(edited_by) => AppState {
                edited_by: edited_by.to_string(), ..current_state
            },
            Mutation::Nothing() => AppState { ..current_state }
        }
    }

    fn create_mutation(mutation_type: &String, mutation_value: &String) -> Mutation {
        let mutation = if mutation_type == "CHANGE_EDITED_BY" {
            Mutation::ChangeEditedBy(mutation_value.to_string())
        } else {
            Mutation::Nothing()
        };

        mutation
    }
}


fn main() {
    let state = AppState::new();
    println!("INITIAL_STATE IS {:?}", state);

    let args: Vec<String> = env::args().collect();
    let mutation_type = &args[1].clone();
    let mutation_value = &args[2].clone();

    let new_mutation = AppState::create_mutation(mutation_type, mutation_value);

    println!("{:?}", new_mutation);
    println!("{}: {}", mutation_type, mutation_value);

    let updated_state = AppState::receive_mutation(state, &new_mutation);
    println!("{:?}", updated_state);
}

// state is:
// color, blue red, yellow
// number, 1, 10, 100
// shape, circle triangle square
// editor name

// CHANGE_COLOR
// INCREMENT, DECREMENT
// CHANGE_BOOLEAN
// EDITED_BY