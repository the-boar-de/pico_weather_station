#![allow(warnings)]
#[repr(i32)]
pub enum EnumApiRequest {
    // Init State
    Api_Init = 0,
    //Idle State
    Api_Idle = 10,
    //Update State
    Api_Update = 20,
    //Error State
    Api_Error = 90,
}

#[repr(i32)]
pub enum EnumStates {
    // Init State
    State_Init = 0,
    //Idle State
    State_Idle = 10,
    //Update State
    State_Update = 20,
    //Error State
    State_Error = 90,
}
