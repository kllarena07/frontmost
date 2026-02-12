# Frontmost
## ✍ About
A crate that dynamically captures the currently focused application on macOS, powered by [`objc2`](https://docs.rs/objc2/latest/objc2/index.html), [`objc2_foundation`](https://docs.rs/objc2-foundation/latest/objc2_foundation/), and [`objc2_app_kit`](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/) Rust bindings.

## 🤔 Why Frontmost

One might assume that you could simply use the [`applications`](https://crates.io/crates/applications) crate by placing `ctx.get_frontmost_application().unwrap()` inside a loop and call it a day. However, **this approach does not work as expected in practice**.

Why? Because macOS uses an event-driven architecture, and a blocking loop prevents the main [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) from processing events. The [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) powers macOS's asynchronous notification system, so blocking it disrupts normal event handling.

To work with this architecture, the code instead creates an observer (such as an NSWorkspace notification observer) that triggers when the user switches focus to a different application, specifically utilizing the `NSWorkspaceDidActivateApplicationNotification` notification. This allows your code to respond to application changes without blocking the run loop.

## 📖 How to Use
Check [examples](https://github.com/kllarena07/frontmost/tree/main/examples) for an example of frontmost put into use.

1. Bring the `frontmost` module and dependencies into scope
```
use frontmost::{Detector, start_nsrunloop};
use objc2::rc::Retained;
use objc2_foundation::NSString;
```
2. Create the callback function that will be triggered when the user switches to a different application
```
fn handle_app_change(frontmost_app: Retained<NSString>) {
    println!("Application activated: {}", frontmost_app);
}
```
3. Initialize a `Detector` singleton by calling the `init` function and pass your callback function into it
```
Detector::init(handle_app_change);
```
4. Start the event loop using the `start_nsrunloop!()` macro
```
start_nsrunloop!();
```

### Complete Example
```rust
use frontmost::{Detector, start_nsrunloop};
use objc2::rc::Retained;
use objc2_foundation::NSString;

fn main() {
    fn handle_app_change(frontmost_app: Retained<NSString>) {
        println!("Application activated: {}", frontmost_app);
    }

    Detector::init(handle_app_change);

    println!("Monitoring application activations. Press Ctrl+C to stop.");
    start_nsrunloop!();
}
```

## 👾 Bugs or vulnerabilities

If you find any bugs or vulnerabilities, please contact me on my Twitter using the link below.

_Made with ❤️ by [krayondev](https://x.com/krayondev)_
