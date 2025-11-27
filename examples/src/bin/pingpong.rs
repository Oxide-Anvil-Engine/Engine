use core::application;

fn main() {
    let mut app = App::new();

    app.set_update_callback(|renderer, inputs, delta_time| {
        // Update logic here
    });

    app.run();
}
