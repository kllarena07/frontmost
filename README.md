# Frontmost
## ✍ About
A crate that dynamically captures the currently focused application on macOS, powered by [`objc2`](https://docs.rs/objc2/latest/objc2/index.html), [`objc2_foundation`](https://docs.rs/objc2-foundation/latest/objc2_foundation/), and [`objc2_app_kit`](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/) Rust bindings.

## 🤔 Why Frontmost

One might assume that you could simply use the [`applications`](https://crates.io/crates/applications) crate by placing `ctx.get_frontmost_application().unwrap()` inside a loop and call it a day. However, **this approach does not work as expected in practice**.

Why? Because macOS uses an event-driven architecture, and a blocking loop prevents the main [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) from processing events. The [`NSRunLoop`](https://developer.apple.com/documentation/Foundation/RunLoop?language=objc) powers macOS's asynchronous notification system, so blocking it disrupts normal event handling.

To work with this architecture, the code instead creates an observer (such as an NSWorkspace notification observer) that triggers when the user switches focus to a different application, specifically utilizing the `NSWorkspaceDidActivateApplicationNotification` notification. This allows your code to respond to application changes without blocking the run loop.

## 📖 How to Use
Check [examples](https://github.com/kllarena07/frontmost/tree/main/examples) for an example of frontmost put into use.

1. Bring the `frontmost` module into scope
```
use frontmost::app::FrontmostApp;
use frontmost::{Detector, start_nsrunloop};
```
2. Define your application struct that will hold state
```
#[derive(Debug)]
struct MyApp {
    frontmost: String,
}
```
3. Implement the `FrontmostApp` trait for your struct
- `set_frontmost(&mut self, new_value: &str)` - Called when app switches, receives app name
- `update(&self)` - Called after setting, for side effects like logging or triggers
```
impl FrontmostApp for MyApp {
    fn set_frontmost(&mut self, new_value: &str) {
        self.frontmost = new_value.to_string();
    }
    
    fn update(&self) {
        println!("Application activated: {}", self.frontmost);
    }
}
```
4. Initialize the Detector by passing a boxed instance
```
Detector::init(Box::new(my_app));
```
5. Start the event loop using the `start_nsrunloop!()` macro
```
start_nsrunloop!();
```

### Complete Example
```rust
use frontmost::app::FrontmostApp;
use frontmost::{Detector, start_nsrunloop};

#[derive(Debug)]
struct App {
    frontmost: String,
}

impl FrontmostApp for App {
    fn set_frontmost(&mut self, new_value: &str) {
        self.frontmost = new_value.to_string();
    }
    
    fn update(&self) {
        println!("Application activated: {}", self.frontmost);
    }
}

fn main() {
    let my_app = App {
        frontmost: String::new(),
    };

    Detector::init(Box::new(my_app));

    println!("Monitoring application activations. Press Ctrl+C to stop.");
    start_nsrunloop!();
}
```

## 👾 Bugs or vulnerabilities

If you find any bugs or vulnerabilities, please contact me on my Twitter using the link below.

_Made with ❤️ by [krayondev](https://x.com/krayondev)_
