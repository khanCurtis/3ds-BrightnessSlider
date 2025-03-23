# 3DS-BrightnessSlider

A Rust implementation to repurpose the 3D slider on New Nintendo 3DS systems as a brightness controller.

## Overview

This project transforms the 3D slider on New Nintendo 3DS systems into a brightness controller. The unused 3D slider becomes a practical tool for quick brightness adjustments without navigating through system menus.

## Features

- Convert the 3D slider to control screen brightness
- Configurable minimum and maximum brightness levels
- Real-time response with minimal CPU usage
- Clean exit using START button
- Written in safe, performant Rust

## Requirements

- New Nintendo 3DS or New Nintendo 3DS XL with custom firmware (CFW)
- [devkitPro](https://devkitpro.org/) with devkitARM installed
- Rust toolchain with rust-3ds environment

Installation
Setting Up Development Environment

Install devkitPro with 3DS support:
For Windows:

Use the installer from the devkitPro website

For macOS:
bash

### Building the Project

1. Clone this repository:
   ```bash
   git clone https://github.com/khanCurtis/3DS-BrightnessSlider.git
   cd 3DS-BrightnessSlider
   ```

2. Build the project:
   ```bash
   cargo 3ds build --release
   ```

3. The output file will be located at `target/armv6k-nintendo-3ds/release/3ds-brightness-slider.3dsx`

### Installing on Your 3DS

1. Copy the `.3dsx` file to your 3DS SD card in the `/3ds/` directory
2. Launch the Homebrew Launcher on your 3DS
3. Find and run the "BrightnessSlider" application

## Usage

1. Launch the app from the Homebrew Launcher
2. Use the 3D slider to adjust screen brightness:
   - Slide up for higher brightness
   - Slide down for lower brightness
3. Press START to exit the application

## Technical Details

### How It Works

This application intercepts the 3D slider input values and maps them to brightness levels. It uses the ctru-rs crate to interact with 3DS hardware through safe Rust code.

The core functions:
- Read the 3D slider value (0.0 to 1.0)
- Map this value to a brightness percentage (10% to 100%)
- Set the screen brightness through system calls
- Update in real-time as the slider moves

### Configuration

You can modify these constants in the source code:

```rust
const UPDATE_INTERVAL: u64 = 100;  //Watches for slider pos every 100ms
const MIN_BRIGHTNESS: u8 = 10;
const MAX_BRIGHTNESS: u8 = 100;
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The [ctru-rs](https://github.com/rust3ds/ctru-rs) team for Rust bindings to libctru
- The Nintendo 3DS homebrew community
- All contributors to the rust-3ds project

## Disclaimer

This software is unofficial and is not affiliated with Nintendo. Use at your own risk. Modifying your device may void your warranty.
