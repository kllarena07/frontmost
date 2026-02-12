use frontmost::{start_nsrunloop, Detector};
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
