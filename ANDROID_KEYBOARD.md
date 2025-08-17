# Android Virtual Keyboard Support for egui Applications

This module provides automatic virtual keyboard support for egui applications running on Android devices.

## Overview

The Android keyboard module solves the common problem where the virtual keyboard doesn't appear when focusing on text fields in egui applications on Android. This is a known issue discussed in:

- [egui Issue #5590](https://github.com/emilk/egui/issues/5590)
- [android-activity PR #178](https://github.com/rust-mobile/android-activity/pull/178)

## Features

- 🔧 **Automatic keyboard detection**: Shows keyboard when text fields gain focus
- 🔇 **Automatic keyboard hiding**: Hides keyboard when text fields lose focus
- 🛡️ **Exception handling**: Robust JNI exception handling for stability
- 🎯 **Focus tracking**: Efficient tracking of UI focus state changes
- 🔄 **State management**: Prevents unnecessary JNI calls for performance

## Implementation Details

### How it Works

1. **Focus Detection**: Uses `egui::Context::memory().focused()` to detect when any text field has focus
2. **JNI Integration**: Calls Android's `InputMethodManager.showSoftInput()` and `hideSoftInputFromWindow()` methods
3. **State Tracking**: Maintains keyboard state to avoid redundant calls
4. **Exception Safety**: Comprehensive exception handling for JNI operations

### Core Functions

```rust
// Check if any text field currently has focus
pub fn check_ui_keyboard_focus(ctx: &egui::Context) -> bool

// Show the Android virtual keyboard
pub fn show_soft_input(show_implicit: bool) -> Result<(), Box<dyn std::error::Error>>

// Hide the Android virtual keyboard
pub fn hide_soft_input() -> Result<(), Box<dyn std::error::Error>>

// Initialize Android context (currently a placeholder)
pub fn try_init_from_eframe()
```

## Integration in Your App

### 1. Dependencies

Add to your `Cargo.toml`:

```toml
[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
ndk-sys = "0.6"
ndk = "0.9"
```

### 2. Module Integration

In your `lib.rs`:

```rust
mod android_keyboard;
pub use android_keyboard::{show_soft_input, hide_soft_input, check_ui_keyboard_focus};
```

### 3. App Update Loop

In your app's `update` method:

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Android keyboard handling
    #[cfg(target_os = "android")]
    {
        let should_show_keyboard = crate::android_keyboard::check_ui_keyboard_focus(ctx);

        if should_show_keyboard != self.android_keyboard_shown {
            if should_show_keyboard {
                if let Err(e) = crate::android_keyboard::show_soft_input(true) {
                    log::warn!("Failed to show Android keyboard: {}", e);
                } else {
                    self.android_keyboard_shown = true;
                }
            } else {
                if let Err(e) = crate::android_keyboard::hide_soft_input() {
                    log::warn!("Failed to hide Android keyboard: {}", e);
                } else {
                    self.android_keyboard_shown = false;
                }
            }
        }
    }

    // Rest of your UI code...
}
```

### 4. App State

Add keyboard state tracking to your app struct:

```rust
pub struct MyApp {
    // ... other fields

    #[serde(skip)]
    android_keyboard_shown: bool,
}
```

## Testing

### Debug Controls

In debug builds on Android, you can add manual keyboard controls:

```rust
#[cfg(all(target_os = "android", debug_assertions))]
ui.collapsing("🔧 Test Android Keyboard", |ui| {
    ui.horizontal(|ui| {
        if ui.button("📱 Show Keyboard").clicked() {
            let _ = crate::android_keyboard::show_soft_input(true);
        }
        if ui.button("🚫 Hide Keyboard").clicked() {
            let _ = crate::android_keyboard::hide_soft_input();
        }
    });
});
```

### Focus State Monitoring

Monitor focus state for debugging:

```rust
let focus_state = crate::android_keyboard::check_ui_keyboard_focus(&ui.ctx());
ui.label(format!("Focus: {}", if focus_state { "Active" } else { "Inactive" }));
```

## Building for Android

### Prerequisites

```bash
# Install Android NDK and set environment variables
export ANDROID_NDK_ROOT=/path/to/android-ndk
export PATH=$PATH:$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
```

### Cargo Configuration

Create `.cargo/config.toml`:

```toml
[target.aarch64-linux-android]
ar = "aarch64-linux-android-ar"
linker = "aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
ar = "arm-linux-androideabi-ar"
linker = "armv7a-linux-androideabi21-clang"
```

### Building

```bash
# For 64-bit ARM
cargo build --target aarch64-linux-android --release

# For 32-bit ARM
cargo build --target armv7-linux-androideabi --release
```

## Troubleshooting

### Common Issues

1. **"Android app context not initialized"**

   - Solution: The Android context initialization is still a work in progress
   - Workaround: Manually initialize if you have access to JVM/Activity pointers

2. **Keyboard doesn't appear**

   - Check that your app has `android:hasCode="false"` in the manifest (if using NativeActivity)
   - Verify that the view has focus and is focusable
   - Check logcat for JNI exceptions

3. **Performance issues**
   - The implementation uses state tracking to minimize JNI calls
   - Focus checks are lightweight and called every frame

### Logging

Enable debug logging to monitor keyboard operations:

```bash
# During development
export RUST_LOG=debug

# Or in your app
env_logger::init();
```

Look for log messages like:

- "Android soft keyboard show command sent"
- "Android keyboard shown due to focus change"
- "Failed to show Android keyboard: ..."

## Future Improvements

- [ ] Automatic Android context detection from eframe/winit
- [ ] Support for GameActivity (in addition to NativeActivity)
- [ ] Keyboard height detection and layout adjustment
- [ ] IME composition text support
- [ ] Configuration options for keyboard behavior

## Credits

This implementation is based on the community work in:

- [rust-mobile/android-activity#178](https://github.com/rust-mobile/android-activity/pull/178)
- Solutions by [@wareya](https://github.com/wareya) and [@shadow3aaa](https://github.com/shadow3aaa)
- Discussion in [emilk/egui#5590](https://github.com/emilk/egui/issues/5590)

## License

This module follows the same license as the parent project (MIT/Apache 2.0).
