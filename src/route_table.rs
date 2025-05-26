use anyhow::{Result, anyhow};
use std::process::Command;

/// Gets the system's route table using the `route`, `netstat`, or `ip`
/// commands, depending on the operating system.
///
/// On Windows, the `route print` command is used. On macOS, the `netstat -nr`
/// command is used. On other operating systems, the `ip route` command is used.
///
/// The output of the command is printed to the console, and the exit status of
/// the command is returned. If the command fails, an error is returned.
pub fn get_route_table() -> Result<()> {
    let (cmd, args) = if cfg!(target_os = "windows") {
        ("route", &["print"][..])
    } else if cfg!(target_os = "macos") {
        ("netstat", &["-nr"][..])
    } else {
        ("ip", &["route"][..])
    };
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Route table:\n{}", stdout);
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(anyhow!("Error executing command: {}", stderr))
            }
        }
        Err(e) => Err(anyhow!("Error executing command: {}", e)),
    }
}
