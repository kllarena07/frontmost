# Frontmost
## ✍ About
A crate that dynamically captures the currently focused application on macOS, powered by [`objc2`](https://docs.rs/objc2/latest/objc2/index.html), [`objc2_foundation`](https://docs.rs/objc2-foundation/latest/objc2_foundation/), and [`objc2_app_kit`](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/) Rust bindings.

## 🤔 Why Frontmost

One might assume that you could simply use the [`applications`](https://crates.io/crates/applications) crate by placing `ctx.get_frontmost_application().unwrap()` inside a loop and be done with it. However, this approach does not work as expected in practice.

Why? Because macOS uses an event-driven architecture, and a blocking loop prevents the main [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) from processing events. The [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) powers macOS's asynchronous notification system, so blocking it disrupts normal event handling.

To work with this architecture, the code instead creates an observer (such as an NSWorkspace notification observer) that triggers when the user switches focus to a different application, specifically utilizing the `NSWorkspaceDidActivateApplicationNotification` notification. This allows your code to respond to application changes without blocking the run loop.

## 📖 How to Use
1. Bring the `frontmost` module and crate into scope
```
mod frontmost;
use frontmost::Detector;
```
2. Create the callback function that will be used when the observer detects that the user has changed apps
```
use objc2_app_kit::NSRunningApplication;

fn handle_app_change(ns_running_application: &NSRunningApplication) {
    unsafe {
        let frontmost_app_name = ns_running_application
            .localizedName()
            .expect("Failed to capture application localizedName");
        println!("Application activated: {}", frontmost_app_name);
    }
}
```
3. Initialize a `Detector` singleton by calling the `init` function and pass your callback function into it
```
Detector::init(handle_app_change);
```
5. Start the event loop using the `start_nsrunloop!()` macro

## 👾 Bugs or vulnerabilities

If you find any bugs or vulnerabilities, please contact me on my Twitter using the link below.

_Made with ❤️ by [krayondev](https://x.com/krayondev)_
