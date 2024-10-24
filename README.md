# tellcat

**tellcat** is a tool for displaying messages with ASCII art in the terminal. This README guides you on how to use the installation script or how to run the project manually.

<p align="center">
  <img src="https://github.com/user-attachments/assets/8ab53f0e-37e3-411e-9016-5b67ff425a8d" alt="image2">
</p>

## Requirements

Before you start, ensure you have the following:

- **Rust and Cargo**: Make sure you have Rust and Cargo installed on your system. You can check by running:
  ```bash
  rustc --version && cargo --version
  ```
  If not installed, visit rustup.rs for installation instructions.

- **ASCII-Compatible Font**: Your terminal should use a font that supports proper rendering of ASCII art. Monospaced fonts like Courier, Consolas, or Monaco are recommended for the best experience.

- **Terminal with ASCII Support**: Ensure your terminal emulator supports ASCII characters. Most modern terminals like Terminal on macOS, cmd or PowerShell on Windows, and gnome-terminal, xterm, or konsole on Linux should work fine.
  
## Installation and Use with Script

To install **tellcat** and run it automatically, follow these steps:

1. **Download or clone the project:**
   ```bash
   git clone <repository_url>
   cd tellcat
   ```

2. **Make the script executable:**
   ```bash
   chmod +x install.sh
   ```

3. **Run the installation script:**
   ```bash
   ./install.sh
   ```

   The script will build the project, install the executable to `/usr/local/bin`, and run **tellcat** with "Hello World".

## Manual Use

If you prefer not to use the script, or if you want to build and run **tellcat** manually:

### Building and Running

1. **Navigate to the project directory:**
   ```bash
   cd tellcat
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the program:**
   ```bash
   ./target/release/tellcat "Your Message Here"
   ```

   Replace "Your Message Here" with any message you want to display.

### Manual Installation

If you want to install it manually without the script:

1. **After building:**
   ```bash
   sudo cp ./target/release/tellcat /usr/local/bin/
   ```

2. **Make sure it's executable:**
   ```bash
   sudo chmod +x /usr/local/bin/tellcat
   ```

3. **Run from anywhere:**
   ```bash
   tellcat "Your Message Here"
   ```

### Using Cargo Run

For development or testing, you can use `cargo run` directly:

```bash
cargo run -- "Your Message Here"
```

This will compile (if necessary) and run your project with the given message without needing to find the release binary.

---

Make sure you have Rust and Cargo installed on your system to use these instructions. Enjoy using **tellcat**!
