use std::time::{Duration, Instant};
use engine_core::{
    app::{App, AppDesc},
    frames::FrameManagerDesc,
    EventLoopError,
    ControlFlow,
    WindowAttributes,
    Tokio,
};

fn main() -> Result<(), EventLoopError> {
    let app_desc = AppDesc::new(
        ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16)),
        FrameManagerDesc::new(60.0, Duration::from_secs(60), Tokio::new()),
        WindowAttributes::default()
            .with_title("Ping Pong Game"),
    );
    let app = App::new(app_desc);

    // app.set_update_callback(|renderer, inputs, delta_time| {
    //     // Update logic here
    // });

    app.run()
}
