extern crate mouse_keyboard_input;

use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut x = 1;
    let mut device = VirtualDevice::new();

    thread::sleep(Duration::from_secs(1));
    loop {
        x +=1;
        if x > 250{
            break;
        }
        println!("{}",x);
        device.press(KEY_LEFTCTRL).expect("Unable to press Left Ctrl");
        device.press(KEY_V).expect("Unable to press Left Ctrl");
        device.release(KEY_V).expect("Unable to press Left Ctrl");
        device.release(KEY_LEFTCTRL).expect("Unable to press Left Ctrl");
    }
    // for _ in 1..600{
    //     for x in [KEY_A,KEY_B]{
    //         device.press(x).unwrap();
    //         device.release(x).unwrap();
    //     }
    // }
}
