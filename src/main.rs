use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

mod direction;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 50;

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Debug)]
struct Wall {}

struct State {
    ecs: World,
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<direction::Position>();
    let mut players = ecs.write_storage::<Player>();

    // Recall that smaller y means more at the top.
    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<direction::Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<direction::Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(direction::Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    add_walls(&mut gs);

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(direction::Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    rltk::main_loop(context, gs)
}

fn add_walls(gs: &mut State) -> () {
    for x in 0..MAP_WIDTH {
        // Top wall
        make_wall(gs, x, 0).build();
        // Bottom wall
        make_wall(gs, x, MAP_HEIGHT - 1).build();
    }
    for y in 0..MAP_HEIGHT {
        // Left wall
        make_wall(gs, 0, y).build();
        // Bottom wall
        make_wall(gs, MAP_WIDTH - 1, y).build();
    }
}

fn make_wall(gs: &mut State, x: i32, y: i32) -> EntityBuilder<'_> {
    // Top wall
    gs.ecs
        .create_entity()
        .with(direction::Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('#'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
}
