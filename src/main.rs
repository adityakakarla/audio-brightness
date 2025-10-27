use crate::brightness::set_brightness;
use cpal::Device;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::NAN;
use std::io::stdin;
use std::panic;

mod brightness;

fn main() {
    let host = cpal::default_host();
    println!("Below are your possible input devices. Enter the number of your desired device");
    let devices = host.input_devices().expect("Could not get input devices");
    for (i, device) in devices.enumerate() {
        println!(
            "Device {}: {}",
            i + 1,
            device.name().expect("Could not get device name")
        )
    }

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Did not enter a valid string");

    let mut user_device_number: usize = user_input
        .trim()
        .parse()
        .expect("Could not parse your desired device");

    if user_device_number == 0 {
        panic!("0 is not a valid device number")
    }

    user_device_number -= 1;

    let devices_vec: Vec<Device> = host
        .input_devices()
        .expect("Could not get input devices")
        .collect();

    if user_device_number >= devices_vec.len() {
        panic!("Entered device number is invalid");
    }

    let user_device = &devices_vec[user_device_number];

    let config = user_device
        .default_input_config()
        .expect("No default config found");

    let mut min_audio = NAN;
    let mut max_audio = NAN;
    let mut audio_levels: Vec<f32> = Vec::new();
    let mut audio_levels_ptr = &audio_levels;

    let stream = user_device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut total_sum = 0.0;
                for val in data.iter() {
                    total_sum += val.abs();
                }
                let average = total_sum / data.len() as f32;

                if min_audio.is_nan() && max_audio.is_nan() {
                    min_audio = average;
                    max_audio = average;
                    set_brightness(0.5);
                } else if average < min_audio {
                    min_audio = average;
                    set_brightness(0.25);
                } else if average > max_audio {
                    max_audio = average;
                    set_brightness(0.75);
                } else {
                    let scaled_value = (average - min_audio) / (max_audio - min_audio);
                    set_brightness(scaled_value);
                }
            },
            move |err| {
                panic!("{}", err);
            },
            None,
        )
        .expect("Could not build stream");

    stream.play().unwrap();
    println!("Stream running. Press Enter to stop.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
