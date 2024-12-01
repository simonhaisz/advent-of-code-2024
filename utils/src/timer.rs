use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer { start: Instant::now() }
    }
}

impl Default for Timer {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("Completed in {} seconds", self.start.elapsed().as_secs_f64());
    }
}