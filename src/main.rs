//! Welcome to dagsec.
//!
//! `cargo run` prints where to get the desktop app for the computer you're on.
//! The app itself is built with Rust and Tauri; this repository hosts its releases.

const LATEST: &str = "https://github.com/hasanerman/dagsec-desktop/releases/latest/download";

const LOGO: &str = r"
     _
  __| | __ _  __ _ ___  ___  ___
 / _` |/ _` |/ _` / __|/ _ \/ __|
| (_| | (_| | (_| \__ \  __/ (__
 \__,_|\__,_|\__, |___/\___|\___|
             |___/
";

/// The installer for an operating system, as `std::env::consts::OS` names it.
fn installer(os: &str) -> Option<(&'static str, &'static str)> {
    match os {
        "windows" => Some(("Windows 10 and 11", "dagsec-setup.exe")),
        "macos" => Some(("macOS, Apple silicon and Intel", "dagsec.dmg")),
        "linux" => Some(("Debian, Ubuntu and Kali", "dagsec_amd64.deb")),
        _ => None,
    }
}

fn main() {
    println!("{LOGO}");
    println!("Welcome to dagsec: supply-chain security for the age of AI coding agents.\n");
    println!("  - Stops AI agents from installing packages that don't exist or have known vulnerabilities");
    println!("  - Code Guard checks the code your agent writes, on your computer");
    println!("  - Team rules and plans every member's agent follows\n");

    match installer(std::env::consts::OS) {
        Some((name, file)) => println!("Download for {name}:\n  {LATEST}/{file}\n"),
        None => println!("Downloads for every platform:\n  https://dagsec.net/#desktop\n"),
    }
    println!("Docs: https://docs.dagsec.net   Dashboard: https://app.dagsec.net");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_desktop_platform_has_an_installer() {
        for os in ["windows", "macos", "linux"] {
            assert!(installer(os).is_some(), "{os}");
        }
        assert!(installer("freebsd").is_none());
    }
}
