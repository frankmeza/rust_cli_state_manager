
# rust_cli_state_manager
A toy Rust CLI app to manage a few bits of state

`cargo build`  

`cargo run` with two arguments

one of these:

- "CHANGE_EDITED_BY"
  - you can put in any value
- "CHANGE_IS_DAYTIME"
  - true | false
- "CHANGE_COLOR"
  - BLUE | RED | YELLOW
- "CHANGE_NUMBER_INCREMENT"
  - number value
- "CHANGE_NUMBER_DECREMENT"
  - number value

  NEXT ITERATION:
  - when the app runs, it waits for an enum to choose between the mutation types, ex. input 1 for INCREMENT and a second input asking for that value. App stays open and active until an exit option is chosen.  
  
