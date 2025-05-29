use macroquad::{
    color::{Color, colors},
    shapes::draw_rectangle,
};

use crate::{
    ant::{Ant, AntColony},
    map::{CellType, Faction, Map},
    pos::Pos,
};

const FOOD_COLOR: Color = colors::GREEN;

pub fn draw_map(map: &Map) {
    draw_rectangle(
        map.offset_x,
        map.offset_y,
        map.game_size - 20.,
        map.game_size - 20.,
        colors::BLACK,
    );

    for y in 0..map.occupied.len() {
        let row = &map.occupied[y];
        for x in 0..row.len() {
            let point: Pos = Pos::new(x as i16, y as i16);
            let cell = &row[x];
            if cell.is_type(CellType::Food) {
                draw_cell(map, point, FOOD_COLOR);
            } else if matches!(cell.m_type, CellType::Nest(_)) {
                draw_cell(map, point, colors::WHITE);
            }
        }
    }
}

pub fn draw_colony(colony: &AntColony, map: &Map) {
    for ants in &colony.ants {
        draw_ant(ants, &map, colony.faction);
    }
    // for (pos, cell) in colony.scents.iter() {
    //     // draw scents
    //     if cell.get_scent(Scents::Food) > 0 {
    //         let scent_alpha = cell.get_scent(Scents::Food) as f32 / u8::MAX as f32;
    //         let mut color = colors::RED;
    //         color.a = scent_alpha;
    //         draw_cell(map, *pos, color);
    //     } else if cell.get_scent(Scents::Nest) > 0 {
    //         let scent_alpha = cell.get_scent(Scents::Nest) as f32 / u8::MAX as f32;
    //         let mut color = colors::LIGHTGRAY;
    //         color.a = scent_alpha;
    //         draw_cell(map, *pos, color);
    //     }
    // }
}

pub fn draw_cell(map: &Map, pos: Pos, color: Color) {
    draw_rectangle(
        map.offset_x + pos.x as f32 * map.sq_size,
        map.offset_y + pos.y as f32 * map.sq_size,
        map.sq_size,
        map.sq_size,
        color,
    );
}

pub fn draw_cell_small(map: &Map, pos: Pos, color: Color) {
    let margin = map.sq_size / 4.;
    draw_rectangle(
        map.offset_x + pos.x as f32 * map.sq_size + margin,
        map.offset_y + pos.y as f32 * map.sq_size + margin,
        map.sq_size - margin * 2.,
        map.sq_size - margin * 2.,
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

pub fn draw_ant(ant: &Ant, map: &Map, faction: Faction) {
    let color = color(faction);
    draw_cell(map, ant.pos, color);
    if ant.has_food() {
        draw_cell_small(map, ant.pos, FOOD_COLOR);
    }

    // for pos in &self.body {
    //     grid.draw_cell(*pos, self.body_color);
    // }
}

pub fn draw_player(ant: &Ant, map: &Map) {
    draw_cell(map, ant.pos, colors::YELLOW);
    if ant.has_food() {
        draw_cell_small(map, ant.pos, FOOD_COLOR);
    }
}
