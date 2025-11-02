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
    gpio::{InputConfig, Level, OutputConfig, Pull},
    main, peripherals,
    rmt::{PulseCode, Rmt, TxChannelConfig, TxChannelCreator},
    time::{Duration, Rate},
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

    let row0 = esp_hal::gpio::Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default());
    let row1 = esp_hal::gpio::Output::new(peripherals.GPIO1, Level::Low, OutputConfig::default());
    let row2 = esp_hal::gpio::Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let row3 = esp_hal::gpio::Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default());

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

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(peripherals.RMT, freq).unwrap();
    let mut channel = rmt
        .channel0
        .configure_tx(
            peripherals.GPIO20,
            TxChannelConfig::default().with_clk_divider(255),
        )
        .unwrap();

    let mut rows = [row0, row1, row2, row3];
    let cols = [col0, col1, col2 /*, col3*/];
    let mut state: [[bool; 3]; 4] = [[false; 3]; 4];
    let chars = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
        ['*', '0', '#'],
    ];

    let delay = Delay::new();

    loop {
        for (i, row) in rows.iter_mut().enumerate() {
            row.set_high();
            delay.delay_millis(10);
            for (j, input) in cols.iter().enumerate() {
                let input = input.is_high();
                if state[i][j] != input {
                    state[i][j] = input;
                    if input {
                        let char = chars[i][j];
                        println!("{char} clicked!");

                        let data = i * 3 + j;
                        let data = [
                            PulseCode::new((data & 0b0001 != 0).into(), 50, Level::Low, 50),
                            PulseCode::new((data & 0b0010 != 0).into(), 50, Level::Low, 50),
                            PulseCode::new((data & 0b0100 != 0).into(), 50, Level::Low, 50),
                            PulseCode::new((data & 0b1000 != 0).into(), 50, Level::Low, 50),
                            PulseCode::end_marker(),
                        ];

                        let transaction = channel.transmit(&data).unwrap();
                        channel = transaction.wait().unwrap();
                    }
                }
            }
            row.set_low();
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
