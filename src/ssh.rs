use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use ssh2::Session;

pub fn stream_cpu_usage<F>(mut callback: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(String),
{
    // 1. Connect to the SSH server
    let tcp = match TcpStream::connect("gram:22") {
        Ok(tcp) => tcp,
        Err(e) => {
            eprintln!("Failed to connect to gram:22. Please ensure the host is reachable and the Tailscale network is active.");
            return Err(e.into());
        }
    };

    // 2. Set up the SSH session
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // 3. Authenticate with the SSH agent
    // Make sure your SSH agent is running and has your key.
    if let Err(e) = sess.userauth_agent("jcchk") {
        eprintln!("Authentication failed. Is your SSH key added to the agent and authorized on 'gram'?");
        return Err(e.into());
    }

    // 4. Execute the performance counter command
    let mut channel = sess.channel_session()?;
    let command = "powershell -Command \"Get-Counter -Counter '\\Processor(_Total)\\% Processor Time' -Continuous\"";
    channel.exec(command)?;

    // 5. Read the output line by line and invoke the callback
    let mut reader = BufReader::new(channel);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break, // End of stream
            Ok(_) => {
                callback(line.trim().to_string());
                line.clear();
            }
            Err(e) => {
                eprintln!("Error reading from SSH channel: {}", e);
                break;
            }
        }
    }

    Ok(())
}
