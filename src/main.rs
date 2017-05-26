extern crate tcod;
use tcod::{Console, RootConsole, BackgroundFlag};
use std::rand::Rng;

fn render (con: &mut RootConsole, x: i32, y: i32, dog_x: i_32, dog_y: i32 {


    con.clear();
    con.put_char(x, y, '@', BackgroundFlag::Set);
    con.put_char(dog_x, dog_y, 'd' BackgroundFlag::Set);
    con.flush();

}


fn main() {

    let con_x = 140;
    let con_y = 80;

    let mut con = RootConsole::initializer().size(con_x,con_y)
        .title("Dungeon Crawler")
        .init();
    let mut char_x = 40;
    let mut char_y = 25;
    let mut dog_x = 10;
    let mut dog_y = 10;

    render (&mut con, char_x, char_y, dog_x, dog_y);

    while !(con.window_closed()) {
        let physical_key = con.wait_for_keypress(true);

        if physical_key.pressed {

            match physical_key {

                Key{ code: Escape, ..} => break,

                key{ code: Up, ..} => {
                    if char_y >= 1 {
                        char_y -= 1;
                    }
                },

                key { code: Down, ..} => {
                    if char_y < (con_y - 1) {
                        char_y += 1;
                    }
                },

                Key {code: Left, ..} => {
                    if char_x >= 1 {
                        char_x -= 1;
                    }
                },

                Key { code: Right, ..} => {
                    if char_x < (con_x - 1) {
                        char_x += 1;
                    }
                },

                _ => {}

            }
        }


        let offset_x = std::rand::task_range().gen_range(0,3) - 1;
        if (dog_x + offset_x) > 0 && (dog_x + offset_x) < (con_x -1) {
            dog_x += offset_x;
        }

        let offset_y = std::rand::task_rng().gen_range(0,3) - 1;

        if (dog_y + offset_y) > 0 && (dog_y + offset_y) < (con_y - 1) {
            dog_y += offset_y;
        }

        render (&mut con, char_x, char_y, dog_x, dog_y);
    }
}
