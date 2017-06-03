use consts::MAP_HEIGHT;
use consts::MAP_WIDTH;


use tcod::console::*;
use tcod::Console;
use object::Object;
use map::Map;
use tcod::console::blit;
use consts::COLOR_DARK_WALL;
use consts::COLOR_DARK_GROUND;

pub fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {

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


pub fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
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

