use crate::monster::Enemy;
use crate::{menu_loop, Player};
use std::io;

pub(crate) fn fight_loop(player: &mut Player, monster: &mut Enemy) {
    let mut in_fight: bool;
    print!("\x1B[2J\x1B[1;1H");
    println!("------------------------------------------");
    println!("A {} appears!", monster.name);
    in_fight = true;
    while in_fight {
        player_turn(player, monster, &mut in_fight);
    }
}

fn player_turn(player: &mut Player, monster: &mut Enemy, in_fight: &mut bool) {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "Your HP: {} | {} HP: {}",
            player.hp, monster.name, monster.hp
        );
        println!("press 1 to attack");
        println!("press 2 to run");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess {
            1 => {
                let damage = player.ap - monster.dp;
                if damage < 0 {
                    monster.hp -= 1;
                } else {
                    monster.hp -= damage;
                }
                if monster.hp <= 0 {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("You have defeated the monster!");
                    *in_fight = false;
                    player.cs_counter += monster.cs;
                    if player.cs_counter >= player.cs {
                        player.cs_counter = 0;
                        player.hp += 10;
                        player.ap += 5;
                        player.dp += 5;
                        println!("You have leveled up!");
                    }
                    break;
                }
                enemy_turn(player, monster, in_fight);
                break;
            }
            2 => {
                print!("\x1B[2J\x1B[1;1H");
                println!("You ran away!");
                *in_fight = false;
                break;
            }
            _ => continue,
        }
    }
}

fn enemy_turn(player: &mut Player, monster: &mut Enemy, in_fight: &mut bool) {
    let damage = monster.ap - player.dp;
    if damage < 0 {
        player.hp -= 1;
    } else {
        player.hp -= damage;
    }
    if player.hp <= 0 {
        print!("\x1B[2J\x1B[1;1H");
        println!("You have been defeated!");
        *in_fight = false;
        menu_loop(player);
    }
}
