use std::process::Command;

pub fn set_brightness(brightness_level: f32) {
    if brightness_level < 0.0 || brightness_level > 1.0 {
        panic!("Received invalid brightness level")
    }
    Command::new("brightness")
        .arg(format!("{}", brightness_level).as_str())
        .output()
        .expect("Failed to change brightness");
}
