mod ssh;
mod parser;
mod display;

fn main() {
    println!("Connecting to 'gram' to get CPU usage...");

    let mut renderer = display::Renderer::new();

    if let Err(e) = ssh::stream_cpu_usage(|line| {
        if let Some(cpu_usage) = parser::parse_cpu_usage(&line) {
            renderer.draw(cpu_usage);
        }
    }) {
        eprintln!("Error: {}", e);
    }
}
