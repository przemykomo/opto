#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{InputConfig, OutputConfig, Pull},
    main, peripherals,
    time::Duration,
};
use esp_println::println;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut row0 = esp_hal::gpio::Output::new(
        peripherals.GPIO0,
        esp_hal::gpio::Level::Low,
        OutputConfig::default(),
    );
    let mut row1 = esp_hal::gpio::Output::new(
        peripherals.GPIO1,
        esp_hal::gpio::Level::Low,
        OutputConfig::default(),
    );
    let mut row2 = esp_hal::gpio::Output::new(
        peripherals.GPIO2,
        esp_hal::gpio::Level::Low,
        OutputConfig::default(),
    );
    let mut row3 = esp_hal::gpio::Output::new(
        peripherals.GPIO3,
        esp_hal::gpio::Level::Low,
        OutputConfig::default(),
    );

    let col0 = esp_hal::gpio::Input::new(
        peripherals.GPIO5,
        InputConfig::default().with_pull(Pull::Down),
    );
    let col1 = esp_hal::gpio::Input::new(
        peripherals.GPIO6,
        InputConfig::default().with_pull(Pull::Down),
    );
    let col2 = esp_hal::gpio::Input::new(
        peripherals.GPIO7,
        InputConfig::default().with_pull(Pull::Down),
    );

    // Last column keeps getting false reading, probably the contacts have too large resistance
    // let col3 = esp_hal::gpio::Input::new(
    //     peripherals.GPIO8,
    //     InputConfig::default().with_pull(Pull::Down),
    // );

    let mut rows = [row0, row1, row2, row3];
    let cols = [col0, col1, col2 /*, col3*/];

    let delay = Delay::new();

    loop {
        for (i, row) in rows.iter_mut().enumerate() {
            row.set_high();
            delay.delay_millis(10);
            for (j, col) in cols.iter().enumerate() {
                if col.is_high() {
                    println!("Row: {i}, col: {j}");
                }
            }
            row.set_low();
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
