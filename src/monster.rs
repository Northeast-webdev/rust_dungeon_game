use rand::Rng;

pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub ap: i32,
    pub dp: i32,
    pub cs: i32,
    pub pos: (usize, usize),
}

pub(crate) fn monster_gen(player_pos: (usize, usize)) -> Enemy {
    let mut rng = rand::thread_rng();
    let hp = rng.gen_range(50..100);
    let ap = rng.gen_range(5..10);
    let dp = rng.gen_range(5..10);
    let cs = rng.gen_range(10..20);
    let mut monster = Enemy {
        name: name_gen(),
        hp: 0,
        ap: 0,
        dp: 0,
        cs: 0,
        pos: (rng.gen_range(2..18), rng.gen_range(2..8)),
    };
    if player_pos == monster.pos {
        monster.pos = (rng.gen_range(2..18), rng.gen_range(2..8));
    }
    monster.hp = hp;
    monster.ap = ap;
    monster.dp = dp;
    monster.cs = cs;
    monster
}

fn name_gen() -> String {
    let mut rng = rand::thread_rng();
    let names = vec!["Goblin", "Orc", "Troll", "Dragon", "Giant", "Golem"];
    let name = names[rng.gen_range(0..names.len())];
    String::from(name)
}
