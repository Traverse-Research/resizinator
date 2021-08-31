use rand::prelude::*;
use std::process::Command;

fn main() {
    // Configure these with your set up.
    let window_name = "My Window Name";
    let resolution_max_x = 1920;
    let resolution_max_y = 1080;
    let dpi_scale = 1.5;


    let mut rng = rand::thread_rng();
    let max_x = (resolution_max_x as f32 * dpi_scale) as usize;
    let max_y = (resolution_max_y as f32 * dpi_scale) as usize;
    loop {


        // Use wmctrl to move the window to a random spot with a random size.
        let x = rng.gen_range::<usize, _>(0..max_x);
        let y = rng.gen_range::<usize, _>(0..max_y);
        let width = rng.gen_range::<usize, _>(0..max_x);
        let height = rng.gen_range::<usize, _>(0..max_y);
        let args = format!("0,{},{},{},{}", x, y, width, height);
        Command::new("sh")
            .arg("-c")
            .arg(format!("wmctrl -r '{}' -e '{}'", window_name, args))
            .output()
            .expect("failed to execute process");

        // Wait a random amount of time
        let delay_max = 150;
        let delay = rng.gen_range::<usize, _>(1..delay_max);
        // Curve the delay so we get more short delays than long delays
        let delay = ((1.0 - (delay as f32 / delay_max as f32)).powf(3.0) * delay_max as f32) as usize;
        std::thread::sleep(std::time::Duration::from_millis(delay as u64));
    }
}
