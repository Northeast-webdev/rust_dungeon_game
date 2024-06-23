mod fight;
mod map;
mod monster;
use std::io;
struct Player {
    hp: i32,
    ap: i32,
    dp: i32,
    name: String,
    cs: i32,
    cs_counter: i32,
}

fn main() {
    // make a game where you get to attack every turn
    let mut player = Player {
        hp: 100,
        ap: 40,
        dp: 20,
        name: String::from("Test"),
        cs: 80,
        cs_counter: 0,
    };
    menu_loop(&mut player);
}

fn change_name(player: &mut Player) {
    println!("Enter your new name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    player.name = name;
    print!("\x1B[2J\x1B[1;1H");
}

fn game_loop(player: &mut Player, is_in_game: &mut bool) {
    map::show_map(player, is_in_game);
}

fn menu_loop(player: &mut Player) {
    let mut is_in_game = false;
    loop {
        if is_in_game {
            game_loop(player, &mut is_in_game);
        }
        print!("\x1B[2J\x1B[1;1H");
        println!("Welcome {}", player.name);
        println!("press 1 to enter the game");
        println!("press 2 to change your name");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        print!("\x1B[2J\x1B[1;1H");
        match guess {
            1 => {
                is_in_game = true;
            }
            2 => change_name(player),
            _ => continue,
        }
    }
}
