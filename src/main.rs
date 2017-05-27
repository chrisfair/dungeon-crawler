extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;


const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;


const LIMIT_FPS: i32 = 20;


const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color {r: 50, g:50, b: 150};

type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
struct Tile {

    blocked: bool,
    block_sight: bool,

}


impl Tile {
    pub fn empty() -> Self {
        Tile {blocked: false, block_sight: false}
    }

    pub fn wall -> Self {
        Tile {blocked: true, block_sight: true}
    }
}


#[derive(Debug)]
struct Object {

    x: i32,
    y: i32,
    char: char,
    color: Color,

}

impl Object {
    pub fn new (x: i32, y: i32, char: char, color: Color) -> Self {

        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32){

        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&mut self, con: &mut Console)
    {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&mut self, con: &mut Console)
    {
    con.put_char(self.x, self.y, ' ', BackgroundFlag::None)
    }



}

fn make_map() -> Map {
    let mut Map  = vec![vec![Tile::empty(); MAP_HEIGHT as usize], MAP_WIDTH as usize];

    map [30][22] = Tile::wall();
    map [50][22] = Tile::wall();
    
    map
}


fn render_all(root: @&mut Root, con: &mut Offscreen, objects: &[Object], map &Map) {


    


}



fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;


    let key = root.wait_for_keypress(true);
    match key {

        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }


        Key { code: Escape, ..} => return true,

        Key { code: Up, ..} => *player.y -= 1,
        Key { code: Down, .. } => *player.y += 1,
        Key { code: Left, .. } => *player.x -= 1,
        Key { code: Right, .. } => *player.x += 1,

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

    let player = Object::new(SCREEN_WIDTH /2, SCREEN_HEIGHT /2, '@', colors::WHITE);
    let npc = Objects::new(SCREEN_WIDTH /2  -5, SCREEN_HEIGHT /2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    let player = &mut objects[0];

    while !root.window_closed() {


        for object in &objects {

    
        let exit = handle_keys(&mut root, player);

        if exit {
            break
        }
    }
}

