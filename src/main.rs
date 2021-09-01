use rand::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResizinatorError {
    #[error("Window not found '{0}'")]
    WindowNotFound(String),
    #[error("unknown data store error")]
    Unknown,
}
pub(crate) type Result<V, E = ResizinatorError> = ::std::result::Result<V, E>;

trait Resizinator {
    fn resize(&self, x: usize, y: usize, width: usize, height: usize);
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod win;

fn main() -> Result<()> {
    // Configure these with your set up.
    let window_name = "My Window Name";
    let resolution_max_x = 1920;
    let resolution_max_y = 1080;
    let dpi_scale = 1.5;
    let apply_random_position = true;

    let mut rng = rand::thread_rng();
    let max_x = (resolution_max_x as f32 * dpi_scale) as usize;
    let max_y = (resolution_max_y as f32 * dpi_scale) as usize;

    #[cfg(target_os = "windows")]
    let resizinator = win::WinResizinator::new(window_name)?;
    #[cfg(target_os = "linux")]
    let resizinator = linux::LinuxResizinator::new(window_name)?;

    loop {
        // Generate a random position and random size.
        let (x, y) = if apply_random_position {
            let x = rng.gen_range::<usize, _>(0..max_x);
            let y = rng.gen_range::<usize, _>(0..max_y);
            (x, y)
        } else {
            (0, 0)
        };
        let width = rng.gen_range::<usize, _>(0..max_x);
        let height = rng.gen_range::<usize, _>(0..max_y);

        resizinator.resize(x, y, width, height);

        // Wait a random amount of time.
        let delay_max = 150;
        let delay = rng.gen_range::<usize, _>(1..delay_max);
        // Curve the delay so we get more short delays than long delays
        let delay =
            ((1.0 - (delay as f32 / delay_max as f32)).powf(3.0) * delay_max as f32) as usize;
        std::thread::sleep(std::time::Duration::from_millis(delay as u64));
    }
}
