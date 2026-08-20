//Outside scope Declaration
#![no_std]
#![no_main]
use core::time::Duration;

//Modules
use cyw43::aligned_bytes;
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::{Duration, Timer};
use panic_probe as _;
use static_cell::StaticCell;

use ed_utl::*;

mod enums;
use crate::enums::enums::EnumStates::*;

//====================================
// Async Tasks
//====================================

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<
        'static,
        cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>,
        cyw43::Cyw43439,
    >,
) -> ! {
    runner.run().await
}

// TO DO DISPLAY - TASK
#[embassy_executor::task]
async fn display() {}

// TO DO HEART BEAT - TASK

#[embassy_executor::task]
async fn heart_beat() {
    /*   let Pin_25_info = ed_utl::Information {
        pinnumber: 25,
        name: "Heartbeat Pin",
    };

    let mut Pin25 = ed_utl::OutputValue {
        value: 0.0,
        outout_on: false,
        outputname: "",
        information: Pin_25_info,
    };*/

    let sleep = Duration::from_secs(1);

    loop {
        info!("heart beat on");

        info!("heart beat off ");
    }
}

//====================================
// Main
//====================================

// TO DO MAIN SEQUENCE - TASK
#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(spawner: Spawner) {
    //Setup
    let main_loop = State_Init;

    //====================================
    // Main loop
    //====================================
    loop {
        match main_loop {
            //When main start it goaes trough the init sequence
            State_Init => {}

            //Idle State, waiting for the next trigger request
            State_Idle => {}

            // starts the trigger request
            State_Update => {}

            // Error - preventing from program break
            State_Error => {}
        }
    }
}
