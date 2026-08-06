// Name of function is unclear --> can be changed in the future
async fn State_Sequence(_state: &mut Enum_States) {
    // Seqence
    match _state {
        // Init State
        0 => {
            println!("{}", _state);
            _state = 1;
        }
        1 => {
            println!("{}", _state);
            _state = 0;
        }

        _ => {}
    }
}
