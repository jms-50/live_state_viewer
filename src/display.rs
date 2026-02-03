use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const SPINNER_CHARS: [char; 4] = ['|', '/', '-', '\\'];

pub struct Renderer {
    frame: usize,
}

impl Renderer {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        execute!(io::stdout(), cursor::Hide).expect("Failed to hide cursor");
        Renderer { frame: 0 }
    }

    pub fn draw(&mut self, cpu_usage: f64) {
        // Clear the line
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine)
        )
        .expect("Failed to clear line");

        // Choose spinner character
        let spinner_char = SPINNER_CHARS[self.frame % SPINNER_CHARS.len()];
        self.frame += 1;

        // Map CPU usage to delay (0-100% -> 500ms-10ms)
        // Inverse relationship: higher usage = lower delay
        let delay_ms = 500.0 - (cpu_usage * 4.9);
        let delay = Duration::from_millis(delay_ms.max(10.0) as u64);

        // Print the animation frame
        print!(
            "CPU: [{:<25}] {:.2}% {}",
            "=".repeat((cpu_usage / 4.0).round() as usize),
            cpu_usage,
            spinner_char
        );
        io::stdout().flush().expect("Failed to flush stdout");

        // Sleep for the calculated delay
        thread::sleep(delay);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // Cleanup terminal on drop
        execute!(io::stdout(), cursor::Show).expect("Failed to show cursor");
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}
