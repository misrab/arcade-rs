extern crate sdl2;

// #[macro_use] asks the compiler to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro
// expansion happens before the concept of namespace even starts to _exist_ in
// the compilation timeline.
#[macro_use]
mod events;

// use events::Events;
use sdl2::pixels::Color;
// use std::thread;
// use std::time::Duration;

// We cannot call functions at top-level. However, `struct_events` is not your
// usual function: it's a macro. Which means that you can use a macro to do
// pretty much anything _normal_ code would.
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    }
}



fn main() {
    // Initialise SLD2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("Arcade RS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();


    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());


    loop {
        events.pump();

        if events.now.key_escape == Some(true) {
            break;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
