use frontmost::{app::FrontmostApp, start_nsrunloop, Detector};

#[derive(Debug)]
struct App {
    frontmost: String,
}

impl FrontmostApp for App {
    fn set_frontmost(&mut self, new_value: &str) {
        self.frontmost = new_value.to_string();
    }
    fn update(&mut self) {
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
