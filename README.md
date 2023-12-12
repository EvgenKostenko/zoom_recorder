# Zoom Meeting Recording Automator

## Overview
This Rust-based program is designed to automatically start and stop recordings when Zoom meetings begin and end on a macOS computer. It monitors the system for active Zoom meetings and controls recording devices accordingly, streamlining the process of capturing meeting audio.

## Features
- **Automatic Detection**: Detects the start and end of Zoom meetings on your macOS device.
- **MIDI Control**: Sends MIDI commands to control recording hardware.
- **Customizable**: Easy to adapt for different MIDI devices or meeting applications.

## Requirements
- macOS system
- Rust programming environment
- MIDI-compatible recording device
- Zoom meetings running on the host computer

## Installation and Setup

### Prerequisites
- Install Rust: Follow the instructions on [the official Rust website](https://www.rust-lang.org/tools/install) to install the Rust programming environment.
- Install required Rust crates: This project requires external crates like `midir` and `regex`. Add these to your `Cargo.toml` file.

### Downloading the Program
Clone this repository to your local machine using the following command:
```bash
git clone https://github.com/EvgenKostenko/zoom_recorder.git
```

### Configuration
Ensure your MIDI device is connected and identified by the system.
Modify the MIDI command parameters in the code if necessary to match your hardware specifications.
Usage
To run the program, navigate to the cloned directory and execute:

```bash
Copy code
cargo run
```
The program will start monitoring for Zoom meetings and control the recording status based on the meeting's presence.

## Contributing
Contributions to enhance the functionality or extend the compatibility of this program are welcome. Please feel free to fork this repository and submit pull requests.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer
This software is provided "as is", without warranty of any kind. Use it at your own risk.

## Acknowledgements
Thanks to all contributors and users for their interest and feedback.
Special thanks to mido and regex crates for making MIDI handling and pattern matching possible in Rust.