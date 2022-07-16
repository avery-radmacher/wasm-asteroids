pub mod eventloop;
mod game;
mod geom;
mod input;
mod math;
mod render_path;
mod rng;
mod ship;
mod time;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(module = "/js/demo.js")]
extern "C" {
    fn svg_set_path(val: &str);
}

fn putstr(s: &str) {
    console::log_1(&s.into());
}

use eventloop::{Event, EventLoop};
use time::{Duration, Instant};

use game::Game;
use render_path::render_game;

fn duration_to_ms(duration: &Duration) -> f64 {
    (duration.as_secs() as f64) * 1e3 + (duration.subsec_nanos() as f64) / 1e6
}
#[no_mangle]
#[wasm_bindgen(start)]
pub extern "C" fn my_main() {
    let mut game = Box::new(Game::new());

    let _start = Instant::now();

    let mut event_loop = EventLoop::new(Box::new(move |event, event_loop| {
        let game = game.as_mut();
        match event {
            Event::KeyDown {
                code,
                chr: _,
                flags: _,
            } => {
                game.inputs.key_down(code, &game.config);
            }
            Event::KeyUp {
                code,
                chr: _,
                flags: _,
            } => {
                game.inputs.key_up(code, &game.config);
            }
            Event::AnimationFrame => {
                let frame_start = Instant::now();
                game.tick();
                let tick_time = frame_start.elapsed();

                let render_start = Instant::now();
                let mut buf = String::new();
                render_game(&mut buf, game);
                svg_set_path(&buf);
                let render_time = render_start.elapsed();
                let frame_time = frame_start.elapsed();

                if game.tick % 512 == 0 {
                    putstr(&format!(
                        "tick time: {:.3}ms\nrender time: {:.3}ms\ntotal time: {:.3}",
                        duration_to_ms(&tick_time),
                        duration_to_ms(&render_time),
                        duration_to_ms(&frame_time)
                    ));
                }

                event_loop.request_animation_frame();
            }
        }
    }));
    putstr("event loop started");
    event_loop.request_animation_frame();
}
