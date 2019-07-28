enum Mutation {
    ChangeEditedBy(String),
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
        let app_state = AppState {
            color: Color::Blue,
            is_daytime: true,
            edited_by: String::from("fraaank"),
            number: 0,
        };

        return app_state
    }

    fn receive_mutation(current_state: Self, mutation: Mutation) -> AppState {
        match mutation {
            Mutation::ChangeEditedBy(edited_by) => AppState {
                edited_by, ..current_state
            },
        }
    }

    fn init_mutations() -> Mutation {
        Mutation::ChangeEditedBy(String::from("edited_by"))
    }
}

fn main() {
    let state = AppState::new();
    println!("{:?}", state);

    let mutations = AppState::init_mutations();

    let updated_state = AppState::receive_mutation(state, mutations);
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