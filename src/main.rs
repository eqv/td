#![feature(range_contains)]
extern crate ggez;
extern crate nalgebra;
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate serde_derive;
use ggez::conf;
use ggez::event;
use ggez::graphics::{self, FilterMode};
use ggez::ContextBuilder;
use std::env;
use std::path;

mod algebra;
mod assets;
mod background;
mod buffs;
mod camera;
mod card;
mod card_deck;
mod debuffs;
mod dijkstra;
mod direction;
mod effects;
mod end_state;
mod enemies;
mod enemy;
mod event_handler;
mod gui;
mod map;
mod menu_state;
mod overlay_state;
mod pile_overlay;
mod player;
mod playing_state;
mod projectiles;
mod shop_overlay;
mod tile;
mod tower;
mod tower_stats;
mod towers;
mod utils;
mod wave;
use crate::assets::Data;
use crate::event_handler::GameState;
use crate::menu_state::MenuState;

pub fn main() {
    let c = conf::Conf::new();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    let (mut ctx, mut event_loop) = ContextBuilder::new("HexTD", "coco & leex")
        .conf(c)
        .add_resource_path(path)
        .build()
        .expect("couldn't create game context");

    let mut data = Data::new();
    data.init(&mut ctx).expect("couldn't load resources");

    let mut init_state = Box::new(MenuState::new());
    init_state.set_data(data);

    let events = &mut event_handler::GameEventHandler::new(init_state);

    graphics::set_default_filter(&mut ctx, FilterMode::Nearest);
    match event::run(&mut ctx, &mut event_loop, events) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
