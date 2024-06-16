mod model;
mod render;

use crate::{model::PacmanModel, render::PacmanRender};
use rust_pixel::game::Game;

#[cfg(target_arch = "wasm32")]
use rust_pixel::render::adapter::web::{WebAdapter, WebCell};
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct PacmanGame {
    g: Game<PacmanModel, PacmanRender>,
}

pub fn init_game() -> PacmanGame {
    let m = PacmanModel::new();
    let r = PacmanRender::new();
    let mut g = Game::new(m, r, "pacman");
    g.init();
    PacmanGame { g }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PacmanGame {
    pub fn new() -> Self {
        init_game()
    }

    pub fn tick(&mut self, dt: f32) {
        self.g.on_tick(dt);
    }

    pub fn key_event(&mut self, t: u8, e: web_sys::Event) {
        let pe = input_events_from_web(t, e, self.g.context.adapter.get_base().ratio_x, self.g.context.adapter.get_base().ratio_y);
        self.g.context.input_events.push(pe);
    }

    pub fn web_buffer(&self) -> *const WebCell {
        self.g.context.adapter.as_any().downcast_ref::<WebAdapter>().unwrap().web_buf.as_slice().as_ptr()
    }
}

pub fn run() -> Result<(), JsValue> {
    let mut g = init_game().g;
    g.run().unwrap();
    Ok(())
}