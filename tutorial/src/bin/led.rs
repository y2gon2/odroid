use rust_gpiozero::LED;

fn main() {
    loop {
        let mut led = LED::new(17);

        led.blink(2.0, 3.0);
    }
}