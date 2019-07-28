// enum MutationType {
//     CHANGE_COLOR,
//     CHANGE_NUMBER,
//     CHANGE_IS_DAYTIME,
//     CHANGE_EDITED_BY,
// }

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
    fn create_initial_app_state() -> AppState {
        let app_state = AppState {
            color: Color::Blue,
            is_daytime: true,
            edited_by: "fraaank".to_string(),
            number: 0,
        };

        return app_state
    }
}

fn main() {
    let state = AppState::create_initial_app_state();

    println!("{:?}", state)
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