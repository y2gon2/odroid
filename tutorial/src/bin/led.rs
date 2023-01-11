// use rust_gpiozero::LED;

// fn main() {
//     loop {
//         let mut led = LED::new(17);

//         led.blink(2.0, 3.0);
//     }
// }

use gpio::{GpioIn, GpioOut};
use std::{thread, time};

fn main() {
    let mut gpio17 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();
    let mut signal = false;

    loop {
        gpio17.set_value(signal).expect("could not set");
        thread::sleep(time::Duration::from_secs(1));
        signal = !signal;        
    }
}