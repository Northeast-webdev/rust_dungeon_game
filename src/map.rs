use crate::{
    fight::fight_loop,
    monster::{monster_gen, Enemy},
    Player,
};
use rand::Rng;
use std::io;

struct Map {
    area: Vec<Vec<&'static str>>,
    player_pos: (usize, usize),
    enemies: Vec<Enemy>,
}

fn generate_map() -> Vec<Vec<&'static str>> {
    let mut rng = rand::thread_rng();
    let rows = rng.gen_range(10..20);
    let cols = rng.gen_range(10..40);
    let mut map = vec![vec!["·"; cols]; rows];
    let cloned_map = map.clone();
    for row in map.iter_mut() {
        // make the borders walls
        let len = row.len();
        row[0] = "#";
        row[len - 1] = "#";

        // make the first and last row full walls
        if &cloned_map[0] == row || &cloned_map[cloned_map.len() - 1] == row {
            for tile in row.iter_mut() {
                *tile = "#";
            }
        }
    }
    map
}

pub(crate) fn show_map(player: &mut Player, is_in_game: &mut bool) {
    // draw a simple map with the player in the middle

    let mut map = Map {
        area: generate_map(),
        player_pos: (5, 5),
        enemies: vec![],
    };
    for _ in 0..5 {
        map.enemies.push(monster_gen(map.player_pos));
    }
    loop {
        print!("\x1B[2J\x1B[1;1H");
        //remove dead enemies
        map.enemies.retain(|enemy| enemy.hp > 0);
        if map.enemies.is_empty() {
            println!("You have defeated all the enemies!");
            *is_in_game = false;
            break;
        }
        if map.enemies.iter().any(|enemy| map.player_pos == enemy.pos) {
            fight_loop(
                player,
                &mut map
                    .enemies
                    .iter_mut()
                    .find(|enemy| map.player_pos == enemy.pos)
                    .unwrap(),
            );
        }
        for (y, row) in map.area.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if map.enemies.iter().any(|enemy| (x, y) == enemy.pos) && (x, y) != map.player_pos {
                    print!("E")
                } else if (x, y) == map.player_pos {
                    print!("P");
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        match input {
            "w" => {
                if map.player_pos.1 > 1 {
                    map.player_pos.1 -= 1;
                }
            }
            "s" => {
                if map.player_pos.1 < map.area.len() - 2 {
                    map.player_pos.1 += 1;
                }
            }
            "a" => {
                if map.player_pos.0 > 1 {
                    map.player_pos.0 -= 1;
                }
            }
            "d" => {
                if map.player_pos.0 < map.area[0].len() - 2 {
                    map.player_pos.0 += 1;
                }
            }
            _ => continue,
        }
    }
}
