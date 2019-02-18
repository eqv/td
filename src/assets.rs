use crate::direction::Dir;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ImgID {
    Archer,
    Arrow,
    Aura,
    BackgroundWater,
    BackgroundWave(usize),
    Card,
    Coin(usize),
    Cursor,
    CursorMap,
    Damage,
    DamageEnemy,
    DiscardPile,
    DrawPile,
    EmptySlot,
    Fire,
    FloorBuild,
    FloorSpawnDown,
    FloorSpawnLeft,
    FloorSpawnRight,
    FloorSpawnUp,
    FloorTarget,
    Freeze,
    Hex,
    NextWave,
    RPM,
    Range,
    RockEdge,
    SellTower,
    Shop,
    Smoke,
    Stone(usize),
    Take2,
    TileShadow,
    Tree1,
    Tree2,
    Tree3,
    Walk(Dir),
    Zombie,
}
use self::ImgID::*;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum FontID {
    Std,
}

pub struct Data {
    images: HashMap<ImgID, graphics::Image>,
    fonts: HashMap<FontID, graphics::Font>,
}

impl Data {
    pub fn new() -> Self {
        let images = HashMap::new();
        let fonts = HashMap::new();
        return Self { images, fonts };
    }

    fn load_img(&mut self, ctx: &mut Context, map: ImgID, path: &str) -> GameResult<()> {
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(map, img);
        return Ok(());
    }

    fn load_font(&mut self, ctx: &mut Context, map: FontID, path: &str) -> GameResult<()> {
        let fnt = graphics::Font::new(ctx, path)?;
        //let mut fnt = graphics::Font::default_font()?;
        self.fonts.insert(map, fnt);
        return Ok(());
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.load_font(ctx, FontID::Std, "/Typecast.ttf")?;
        self.load_img(ctx, Archer, "/archer.png")?;
        self.load_img(ctx, Arrow, "/arrow.png")?;
        self.load_img(ctx, Aura, "/aura.png")?;
        self.load_img(ctx, BackgroundWater, "/sea_bg.png")?;
        self.load_img(ctx, BackgroundWave(1), "/sea_wave1.png")?;
        self.load_img(ctx, BackgroundWave(2), "/sea_wave2.png")?;
        self.load_img(ctx, BackgroundWave(3), "/sea_wave3.png")?;
        self.load_img(ctx, BackgroundWave(4), "/sea_wave4.png")?;
        self.load_img(ctx, Card, "/card.png")?;
        self.load_img(ctx, Card, "/card.png")?;
        self.load_img(ctx, Coin(1), "/coin1.png")?;
        self.load_img(ctx, Coin(2), "/coin2.png")?;
        self.load_img(ctx, Coin(3), "/coin3.png")?;
        self.load_img(ctx, Cursor, "/cursor.png")?;
        self.load_img(ctx, CursorMap, "/cursor_map.png")?;
        self.load_img(ctx, Damage, "/damage.png")?;
        self.load_img(ctx, DamageEnemy, "/damage_enemy.png")?;
        self.load_img(ctx, DiscardPile, "/discard.png")?;
        self.load_img(ctx, DrawPile, "/cards.png")?;
        self.load_img(ctx, EmptySlot, "/empty_slot.png")?;
        self.load_img(ctx, Fire, "/fire.png")?;
        self.load_img(ctx, FloorBuild, "/floor_build.png")?;
        self.load_img(ctx, FloorSpawnDown, "/floor_spawn_down.png")?;
        self.load_img(ctx, FloorSpawnLeft, "/floor_spawn_left.png")?;
        self.load_img(ctx, FloorSpawnRight, "/floor_spawn_right.png")?;
        self.load_img(ctx, FloorSpawnUp, "/floor_spawn_up.png")?;
        self.load_img(ctx, Freeze, "/freeze.png")?;
        self.load_img(ctx, Freeze, "/freeze.png")?;
        self.load_img(ctx, Hex, "/tile_grass1.png")?;
        self.load_img(ctx, NextWave, "/next_wave.png")?;
        self.load_img(ctx, RPM, "/rpm.png")?;
        self.load_img(ctx, RPM, "/rpm.png")?;
        self.load_img(ctx, Range, "/range.png")?;
        self.load_img(ctx, Range, "/range.png")?;
        self.load_img(ctx, RockEdge, "/rock_edge.png")?;
        self.load_img(ctx, SellTower, "/sell_tower.png")?;
        self.load_img(ctx, Shop, "/shop.png")?;
        self.load_img(ctx, Smoke, "/smoke.png")?;
        self.load_img(ctx, Stone(1), "/stone1.png")?;
        self.load_img(ctx, Stone(2), "/stone2.png")?;
        self.load_img(ctx, Stone(3), "/stone3.png")?;
        self.load_img(ctx, Stone(4), "/stone4.png")?;
        self.load_img(ctx, Take2, "/take_2.png")?;
        self.load_img(ctx, Take2, "/take_2.png")?;
        self.load_img(ctx, TileShadow, "/tile_shadow.png")?;
        self.load_img(ctx, Tree1, "/tree1.png")?;
        self.load_img(ctx, Tree2, "/tree2.png")?;
        self.load_img(ctx, Tree3, "/tree3.png")?;
        self.load_img(ctx, Walk(Dir::East), "/tile_walk.png")?;
        self.load_img(ctx, Walk(Dir::NorthEast), "/tile_walk.png")?;
        self.load_img(ctx, Walk(Dir::NorthWest), "/tile_walk.png")?;
        self.load_img(ctx, Walk(Dir::SouthEast), "/tile_walk.png")?;
        self.load_img(ctx, Walk(Dir::SouthWest), "/tile_walk.png")?;
        self.load_img(ctx, Walk(Dir::West), "/tile_walk.png")?;
        self.load_img(ctx, Zombie, "/enemy.png")?;
        return Ok(());
    }

    pub fn get_i(&self, id: &ImgID) -> &graphics::Image {
        return &self.images[id];
    }

    pub fn get_font(&self) -> &graphics::Font {
        return &self.fonts[&FontID::Std];
    }
}
