use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;

use crate::assets::{Data, ImgID};
use crate::buffs::{self, BuffType};
use crate::enemies::Enemies;
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::projectiles::{Projectile, Projectiles};
use crate::utils::load_specs;

#[derive(Debug, Deserialize, Clone)]
pub struct TowerBaseStats {
    pub damage: usize,
    pub projectile_speed: f32,
    pub range: f32,
    pub rpm: usize,
    pub price: usize,
}

pub struct Tower {
    id: usize,
    cooldown: usize,
    map_position: (usize, usize),
    buff_to_level: HashMap<BuffType, usize>,
}

impl Tower {
    pub fn new(map_position: (usize, usize)) -> Self {
        let buff_to_level = HashMap::new();
        return Self {
            id: 0,
            map_position,
            cooldown: 0,
            buff_to_level,
        };
    }

    pub fn add_buff(&mut self, buff: BuffType) {
        self.buff_to_level.insert(buff, 1);
    }

    pub fn get_buffs(&self) -> &HashMap<BuffType, usize> {
        return &self.buff_to_level;
    }

    pub fn tick(
        &mut self,
        enemies: &Enemies,
        projectiles: &mut Projectiles,
        stats: &TowerBaseStats,
    ) {
        self.cooldown = self.cooldown.saturating_sub(1);
        if let Some(enemy_id) = enemies.weakest_enemy_in_range(
            stats.range,
            GameMap::tile_center(self.map_position.0, self.map_position.1),
        ) {
            if self.cooldown == 0 {
                let mut projectile = Projectile::new(
                    GameMap::tile_center(self.map_position.0, self.map_position.1),
                    self.id,
                    enemy_id,
                    stats.damage,
                    stats.projectile_speed,
                );
                buffs::calc_buff_projectile_effect(self, &mut projectile);
                projectiles.spawn(projectile);
                // 60 sec per minute / rpm * 60 ticks per second
                self.cooldown = 3600 / stats.rpm;
            }
        }
    }
}

pub struct Towers {
    pub base_stats: TowerBaseStats,
    built: HashMap<usize, Tower>,
    position_to_towerid: HashMap<(usize, usize), usize>,
    next_tower_id: usize,
}

impl Towers {
    pub fn new() -> Self {
        let base_stats = load_specs::<TowerBaseStats>("tower")[0].clone();
        let built = HashMap::new();
        let position_to_towerid = HashMap::new();
        return Self {
            base_stats,
            built,
            position_to_towerid,
            next_tower_id: 0,
        };
    }

    pub fn spawn(&mut self, mut tower: Tower) {
        tower.id = self.next_tower_id;
        self.next_tower_id += 1;
        self.position_to_towerid
            .insert(tower.map_position.clone(), tower.id);
        self.built.insert(tower.id, tower);
    }

    pub fn has_building(&self, x: usize, y: usize) -> bool {
        return self.position_to_towerid.contains_key(&(x, y));
    }

    pub fn remove_tower(&mut self, x: usize, y: usize) {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            self.built.remove(id);
            self.position_to_towerid.remove(&(x, y));
        }
    }

    pub fn get_tower_mut(&mut self, x: usize, y: usize) -> Option<&mut Tower> {
        if let Some(id) = self.position_to_towerid.get(&(x, y)) {
            return self.built.get_mut(&id);
        }
        return None;
    }

    pub fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for (_id, t) in self.built.iter() {
            graphics::draw_ex(
                ctx,
                data.get_i(&ImgID::Archer),
                graphics::DrawParam {
                    // src: src,
                    dest: GameMap::tile_center(t.map_position.0, t.map_position.1),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    pub fn tick(state: &mut GameState) {
        for (_id, t) in state.towers.built.iter_mut() {
            t.tick(
                &state.enemies,
                &mut state.projectiles,
                &state.towers.base_stats,
            )
        }
    }
}
