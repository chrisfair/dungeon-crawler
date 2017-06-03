extern crate tcod;
extern crate rand;


use std::cmp;
use rand::Rng;
use rect::Rect;
use tile::Tile;
use consts::MAP_WIDTH;
use consts::MAP_HEIGHT;
use consts::MAX_ROOMS;
use consts::ROOM_MAX_SIZE;
use consts::ROOM_MIN_SIZE;


pub type Map = Vec<Vec<Tile>>;

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn make_map() -> (Map, (i32, i32)) {
    let mut map  = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut rooms = vec![];

    let mut starting_position = (0,0);


    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);

        let x  = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            create_room(new_room, &mut map);

            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                starting_position = (new_x, new_y);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room);

        }
    }

    (map, starting_position)
}


