# Rust AR iOS Project

This project demonstrates how to build an augmented reality (AR) application using Rust for the core logic, interfacing with ARKit on iOS. The project is set up to be developed primarily in VS Code, with Xcode used for building and deploying to iOS devices.

## Features

- 3D object placement in AR space (cubes and spheres)
- Plane detection and visualization
- Touch interaction with AR content
- Cross-language communication between Rust and Swift
- Complete development workflow using VS Code and Xcode

## Prerequisites

- macOS (required for iOS development)
- Xcode 14.0+ with Command Line Tools
- iOS device with ARKit support (iPhone 6s or newer, running iOS 11+)
- VS Code with the following extensions:
  - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
  - [Xcode Helper](https://marketplace.visualstudio.com/items?itemName=harrisonlo.xcode-helper) (optional)
- Rust and Cargo installed

## Project Structure

```
rust_ar_ios/
├── Cargo.toml              # Rust project configuration
├── src/
│   └── lib.rs              # Rust code for AR functionality
├── ios/
│   └── RustARIOS/          # Xcode project
│       ├── RustARIOS/
│       │   ├── AppDelegate.swift
│       │   ├── ViewController.swift
│       │   ├── Info.plist
│       │   └── ...
│       └── ...
└── .vscode/
    ├── settings.json       # VS Code configuration
    └── tasks.json          # VS Code tasks
```

## Setup Instructions

### 1. Install Rust and iOS Targets

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add iOS targets
rustup target add aarch64-apple-ios x86_64-apple-ios

# Install cargo-lipo for universal binary creation
cargo install cargo-lipo
```

### 2. Clone and Setup the Project

```bash
# Clone the repository
git clone https://github.com/yourusername/ARLens.git
cd ARLens

# Install dependencies
# (If you're starting from scratch, copy the Cargo.toml and source files first)
cargo build
```

### 3. Configure VS Code

Create or update `.vscode/settings.json`:

```json
{
    "rust-analyzer.cargo.target": "aarch64-apple-ios",
    "rust-analyzer.check.allTargets": false,
    "rust-analyzer.check.extraArgs": [
        "--target",
        "aarch64-apple-ios"
    ]
}
```

Create or update `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Build Rust for iOS",
            "type": "shell",
            "command": "cargo build --target aarch64-apple-ios --release",
            "group": {
                "kind": "build",
                "isDefault": true
            }
        },
        {
            "label": "Open Xcode Project",
            "type": "shell",
            "command": "open ios/RustARIOS/RustARIOS.xcodeproj",
            "problemMatcher": []
        }
    ]
}
```

### 4. Create Xcode Project

If you haven't created the Xcode project yet:

1. Open Xcode
2. Create a new iOS app project (File > New > Project)
3. Choose "App" under iOS
4. Name it "RustARIOS"
5. Choose Swift for language
6. Save it in the `ios` directory of your project

### 5. Configure Xcode Project

1. In Xcode, select your project in the navigator
2. Go to the "Signing & Capabilities" tab
3. Add the "ARKit" capability
4. Go to the "Info" tab and add these privacy descriptions:
   - `NSCameraUsageDescription`: "AR features require camera access"

### 6. Add Swift Bridge Code

Copy the Swift code from the `simplified-ios-bridge` artifact to your Xcode project. Replace or create these files:
- `ViewController.swift`
- `AppDelegate.swift`

### 7. Build the Rust Library

In VS Code, use the "Build Rust for iOS" task or run the following command:

```bash
cargo build --target aarch64-apple-ios --release
```

This will create a static library at `target/aarch64-apple-ios/release/librust_ar_ios.a`.

### 8. Link Rust Library with Xcode

1. In Xcode, go to your target's Build Phases
2. Click "+" under "Link Binary With Libraries"
3. Click "Add Other..." then "Add Files..."
4. Navigate to your project's `target/aarch64-apple-ios/release/` directory
5. Select `librust_ar_ios.a` and click "Open"
6. Also add these frameworks:
   - ARKit
   - Metal
   - SceneKit
   - UIKit

## Development Workflow

1. Edit Rust code in VS Code
2. Build the Rust library using VS Code task or terminal command
3. Open the Xcode project (can use VS Code task)
4. Run the app on your iOS device from Xcode

When you make changes to your Rust code, you'll need to rebuild the library and then build the Xcode project again.

## Running the App

1. Connect your iOS device to your Mac
2. Select your device in Xcode's device menu
3. Click the "Run" button in Xcode
4. Allow camera access when prompted
5. Point your device at a flat surface to detect planes
6. Tap the buttons to place objects in AR space

## Troubleshooting

### Common Issues

1. **"Library not found" error**: Make sure the path to your Rust library in Xcode is correct. The path might need to be updated after cleaning the project.

2. **ARKit not working**: Ensure your device is compatible and privacy permissions are set correctly in Info.plist.

3. **Rust build errors**: Check that you have the correct iOS targets installed.

4. **"Symbol not found" errors**: Make sure all required frameworks are linked in Xcode.

5. **App crashes on startup**: Check the Xcode console for error messages. Common issues include missing frameworks or incorrect library paths.

### Solutions

- Run `rustup update` to ensure your Rust installation is current
- Clean and rebuild both the Rust library and Xcode project
- Check that the library name in Swift matches your Cargo.toml configuration
- Try rebuilding with a device connected directly rather than using a simulator

## Extending the Project

Once you have the basic app running, you can extend it with:

1. More complex AR interactions
2. Custom 3D models and textures
3. Advanced plane detection and object placement algorithms
4. UI controls for interacting with AR content
5. Persistent AR content using ARKit's world mapping features

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Bevy Engine](https://bevyengine.org/)
- [ARKit Documentation](https://developer.apple.com/documentation/arkit)
- [Rust FFI Documentation](https://doc.rust-lang.org/nomicon/ffi.html)