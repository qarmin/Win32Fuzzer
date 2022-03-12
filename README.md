# Win32 Fuzzer

This repository contains fuzzer to test Windows API functions(Win32 API).

I created it to help Wine to better mimic Windows OS and allow running more Windows software on Linux. 

Looks that currently tests - Classes: "165", Functions: "10540", Arguments: "35174"

This project works fine with `windows-rs 0.33.0`, so it may require rewrite to support newer versions.

## How to use it
- Install wine(7.3 works fine, any lower version may cause crashes, any newer may also crash, but this is bug in Wine, not in this project)
- Install GNU Rust inside Wine(I used 1.59.0, but any higher version should work fine) - https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers
- Clone/Download `windows-rs` repository(required to get info about windows api functions) - `git clone https://github.com/microsoft/windows-rs.git`
- Clone this repository - `git clone https://github.com/qarmin/Win32Fuzzer.git`
- Compile and run project with 1 argument set as path to `windows-rs` repository `cargo run /home/test/windows-rs`
- Go to WinProject(which is inside this project) `cd WinProject/`
- Compile this project `wine cargo run`
- Generated binaries should be available here `target/debug/wine_project.exe`
- By default, binary should not crash, because all problematic functions are excluded in `settings.rs` file, so feel free to comment required lines.
- Setup if needed `settings.txt` file, template is already available in path `WinProject/settings.txt`