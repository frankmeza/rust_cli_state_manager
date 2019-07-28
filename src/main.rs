struct Mutation {
    key: String,
    value: String,
}

#[derive(Debug)]
enum Color {
    Blue,
    // Red,
    // Yellow,
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
        let app_state = AppState {
            color: Color::Blue,
            is_daytime: true,
            edited_by: String::from("fraaank"),
            number: 0,
        };

        return app_state
    }

    fn receive_mutation(current_state: Self, mutation: Mutation) -> AppState {
        // let edited = "edited_by";

        match mutation.key.trim() {
            "edited_by" => {
                AppState { edited_by: mutation.value, ..current_state }
            }
            _ => {
                AppState { ..current_state }
            }
        }
    }
}

fn main() {
    let state = AppState::new();
    println!("{:?}", state);

    let m = Mutation {
        key: String::from("edited_by"),
        value: String::from("edited_by"),
    };

    let updated_state = AppState::receive_mutation(state, m);
    println!("{:?}", updated_state)

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