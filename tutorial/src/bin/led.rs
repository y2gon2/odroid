// use rust_gpiozero::LED;

// fn main() {
//     loop {
//         let mut led = LED::new(17);

//         led.blink(2.0, 3.0);
//     }
// }

// use gpio::{GpioIn, GpioOut};
// use std::{thread, time};

// fn main() {
//     let mut gpio17 = gpio::sysfs::SysFsGpioOutput::open(17).unwrap();
//     let mut signal = false;

//     loop {
//         gpio17.set_value(signal).expect("could not set");
//         thread::sleep(time::Duration::from_secs(1));
//         signal = !signal;        
//     }
// }

use c2_mmap_pio::{Device, PinId, Value};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut odroid = Device::new()?;
    let mut led_pin = odroid.output_pin(PinId::Phy7)?;
    let blink_interval = Duration::from_millis(500);

    loop {
        led_pin.set_value(Value::High);
        sleep(blink_interval);
        led_pin.set_value(Value::Low);
        sleep(blink_interval);
    }
}