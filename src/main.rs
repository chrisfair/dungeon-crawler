extern crate tcod;
extern crate rand;

mod map;
mod rect;
mod tile;
mod object;

use tcod::console::*;
use tcod::colors::{self, Color};
use map::make_map;
use map::Map;
use object::Object;

// Actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// Size of the map in the window
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;


// Parameters for the autodungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const LIMIT_FPS: i32 = 20;


const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color {r: 50, g:50, b: 150};





fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall  = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    for object in objects {
        object.draw(con);
    }

    blit (con, (0,0), (MAP_WIDTH, MAP_HEIGHT), root, (0,0), 1.0, 1.0);
}



fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;


    let key = root.wait_for_keypress(true);
    match key {

        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }


        Key { code: Escape, ..} => return true,

        Key { code: Up, ..} => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),

        _ => {},
    }

    false
}


fn main (){
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_WIDTH)
        .title("Dungeon Crawler")
        .init();

    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);


    let (map, (player_x, player_y)) = make_map();
    
    let player = Object::new(player_x, player_y, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH /2  -5, SCREEN_HEIGHT /2, '@', colors::YELLOW);
    let mut objects = [player, npc];


    while !root.window_closed() {
        render_all(&mut root, &mut con, &objects, &map);
        root.flush();

        for object in &objects {
            object.clear(&mut con)
        }

        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);

        if exit {
            break
        }
    }
}

