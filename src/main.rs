use std::thread;
use std::time::Duration;
use ctru::prelude::*;
use ctru::services::gfx::Screen;
use ctru::services::hid::Hid;

fn map_slider_to_brightness(slider: f32, min: u8, max: u8) -> u8 {
    let brightness = min as f32 + ((max - min) as f32 * slider);
    brightness as u8
}

fn set_brightness(brightness: u8) -> u8 {
    let safe_brightness = brightness.clamp(10, 100);

    //Placeholder - need to use a ctru-rs function or make direct system call
    safe_brightness
}

fn main() {
    //Initialize ctru
    let apt = Apt::init().unwrap();
    let mut hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let mut console = Console::init(gfx.top_scen.borrow_mut()).unwrap();

    //Config
    const UPDATE_INTERVAL: u64 = 100; //Watches for slider pos every 100ms
    const MIN_BRIGHTNESS: u8 = 10; //KEEP THIS AT 10!!!!!!!!!
    const MAX_BRIGHTNESS: u8 = 100;

    //State tracking
    let mut prev_slider_value = 0.0;
    let mut current_brightness = 50; //Start at 50% brightness

    //Set initial brightness
    let initial_slider = hid.slider_3d();
    set_brightness(map_slider_to_brightness(
        initial_slider,
        MIN_BRIGHTNESS,
        MAX_BRIGHTNESS
    ));

    println!("3D Slider now controls brightness");
    println!("Min: {}% | Max: {}%", MIN_BRIGHTNESS, MAX_BRIGHTNESS);

    while apt.main_loop() {
        hid.scan_input

        //Check if START is pressed to exit
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Get current slider value
        let slider_value = hid.slider_3d();
        
        //Only update if slider value has significantly changed
        if (slider_value = prev_slider_value).abs() > 0.01 {
            prev_slider_value = slider_value;

            //Map slider value to brightness range
            let target_brightness = map_slider_to_brightness(
                slider_value,
                MIN_BRIGHTNESS,
                MAX_BRIGHTNESS
            );

            //Apply brightness
            current_brightness = set_brightness(target_brightness);

            println!("Slider: {:.2} | Brightness: {}%", slider_value, current_brightness);
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        thread::sleep(Duration::from_millis(UPDATE_INTERVAL));
    }
}