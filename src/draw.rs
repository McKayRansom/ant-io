use macroquad::{
    color::{Color, colors},
    shapes::draw_rectangle,
};

use crate::{
    game::Game,
    insect::{ant::{Ant, AntColony, Scents}, spider::Spider},
    map::{CellType, Faction, Map},
    pos::Pos,
};

const FOOD_COLOR: Color = colors::GREEN;

pub const PILLBUG_COLOR: Color = colors::PINK;
pub const SPIDER_COLOR: Color = colors::ORANGE;

pub fn draw_game(game: &Game) {
    draw_map(&game.map);

    if game.show_scents != 0 {
        draw_scents(&game.ant_colonies[game.show_scents as usize - 1], &game.map);
    }
    for colony in &game.ant_colonies {
        draw_colony(colony, &game.map);
    }

    for bug in &game.pillbugs {
        draw_cell(&game.map, bug.insect.pos, PILLBUG_COLOR);
    }

    for spider in &game.spiders {
        draw_spider(spider, &game.map, SPIDER_COLOR);
    }

    draw_spider(&game.player, &game.map, colors::YELLOW);
}

pub fn draw_map(map: &Map) {
    let pos = map.screen_pos((0, 0).into());
    draw_rectangle(
        pos.x,
        pos.y,
        map.size.x as f32 * Map::TILE_SIZE_DEFAULT * map.camera.zoom,
        map.size.y as f32 * Map::TILE_SIZE_DEFAULT * map.camera.zoom,
        colors::DARKBROWN,
    );

    for y in 0..map.occupied.len() {
        let row = &map.occupied[y];
        for x in 0..row.len() {
            let point: Pos = Pos::new(x as i16, y as i16);
            let cell = &row[x];
            match cell.m_type {
                CellType::Empty => {}
                CellType::Food => draw_cell(map, point, FOOD_COLOR),
                CellType::Nest(_) => draw_cell(map, point, colors::WHITE),
                CellType::Rock => draw_cell(map, point, colors::GRAY),
                CellType::Wall => {}
            }
        }
    }
}

pub fn draw_scents(colony: &AntColony, map: &Map) {
    let mut pos = Pos::new(0, 0);
    for cell in colony.scents.grid.iter() {
        pos.x += 1;
        if pos.x >= map.size.x {
            pos.x = 0;
            pos.y += 1;
        }
        // draw scents
        if cell.get_scent(Scents::Attack) > 0 {
            let scent_alpha = cell.get_scent(Scents::Attack) as f32 / u8::MAX as f32;
            let mut color = colors::RED;
            color.a = scent_alpha / 2.;
            draw_cell(map, pos, color);
        } else if cell.get_scent(Scents::Food) > 0 {
            let scent_alpha = cell.get_scent(Scents::Food) as f32 / u8::MAX as f32;
            let mut color = colors::GREEN;
            color.a = scent_alpha / 2.;
            draw_cell(map, pos, color);
        } else if cell.get_scent(Scents::Nest) > 0 {
            let scent_alpha = cell.get_scent(Scents::Nest) as f32 / u8::MAX as f32;
            let mut color = colors::LIGHTGRAY;
            color.a = scent_alpha / 2.;
            draw_cell(map, pos, color);
        }
    }
}

pub fn draw_colony(colony: &AntColony, map: &Map) {
    for ants in &colony.workers {
        draw_ant(ants, &map, color(colony.faction));
    }
    for ants in &colony.soldiers {
        draw_ant(ants, &map, {
            let mut color = color(colony.faction);
            color.r += 0.2;
            color.g += 0.2;
            color.b += 0.2;
            color
        });
    }
}

pub fn draw_cell(map: &Map, pos: Pos, color: Color) {
    let rect = map.screen_rect(pos);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

pub fn draw_cell_small(map: &Map, pos: Pos, color: Color) {
    let rect = map.screen_rect(pos);
    let margin = rect.w / 4.;
    draw_rectangle(
        rect.x + margin,
        rect.y + margin,
        rect.w - margin / 2.,
        rect.h - margin / 2.,
        color,
    );
}

pub fn color(faction: Faction) -> Color {
    match faction {
        0 => colors::WHITE,
        1 => colors::BLUE,
        2 => colors::RED,
        _ => unimplemented!(),
    }
}

pub fn draw_ant(ant: &Ant, map: &Map, color: Color) {
    draw_cell(map, ant.insect.pos, color);
    if ant.has_food() {
        draw_cell_small(map, ant.insect.pos, FOOD_COLOR);
    }
}

pub fn draw_spider(spider: &Spider, map: &Map, color: Color) {
    draw_cell(&map, spider.insect.pos, color);
    if spider.digest > 0 {
        draw_cell_small(&map, spider.insect.pos, colors::WHITE);
    }
}
