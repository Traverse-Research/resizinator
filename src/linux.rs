use crate::{Resizinator, Result};
use std::process::Command;

pub(crate) struct LinuxResizinator {
    window_name: String,
}

impl LinuxResizinator {
    pub(crate) fn new(window_name: &str) -> Result<Self> {
        Ok(Self {
            window_name: window_name.into(),
        })
    }
}

impl Resizinator for LinuxResizinator {
    fn resize(&self, x: usize, y: usize, width: usize, height: usize) {
        // Use wmctrl to move and resize the window.
        let args = format!("0,{},{},{},{}", x, y, width, height);
        Command::new("sh")
            .arg("-c")
            .arg(format!("wmctrl -r '{}' -e '{}'", &self.window_name, args))
            .output()
            .expect("failed to execute process");
    }
}
