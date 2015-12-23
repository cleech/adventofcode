use std::cmp::max;

const DATA: &'static str = include_str!("../data/input_21.txt");

pub fn main() -> Vec<String> {
    let boss = Character::boss(DATA);
    vec![least_expensive_win(&boss).unwrap().to_string(),
         most_expensive_loss(&boss).unwrap().to_string()]
}

#[derive(Debug, PartialEq)]
struct Item<'a> {
    name: &'a str,
    cost: usize,
    damage: isize,
    armor: isize,
}

macro_rules! item {
    ($n:expr, $c:expr, $d:expr, $a:expr) => 
        (Item { name: $n, cost: $c, damage: $d, armor: $a })
}

static WEAPONS: [Item<'static>; 5] = [item!("Dagger", 8, 4, 0),
                                      item!("Shortsword", 10, 5, 0),
                                      item!("Warhammer", 25, 6, 0),
                                      item!("Longsword", 40, 7, 0),
                                      item!("Greataxe", 74, 8, 0)];

static ARMOR: [Item<'static>; 6] = [item!("NONE", 0, 0, 0),
                                    item!("Leather", 13, 0, 1),
                                    item!("Chainmail", 31, 0, 2),
                                    item!("Splintmail", 53, 0, 3),
                                    item!("Bandedmail", 75, 0, 4),
                                    item!("Platemail", 102, 0, 5)];

static RINGS: [Item<'static>; 8] = [item!("NONE 1", 0, 0, 0),
                                    item!("NONE 2", 0, 0, 0),
                                    item!("Damage +1", 25, 1, 0),
                                    item!("Damage +2", 50, 2, 0),
                                    item!("Damage +3", 100, 3, 0),
                                    item!("Defense +1", 20, 0, 1),
                                    item!("Defense +2", 40, 0, 2),
                                    item!("Defense +3", 80, 0, 3)];

struct Character {
    hp: isize,
    damage: isize,
    armor: isize,
}

impl Character {
    fn new() -> Character {
        Character {
            hp: 100,
            damage: 0,
            armor: 0,
        }
    }

    fn boss(data: &str) -> Character {
        let mut lines = data.lines();
        let hp = scan_fmt!(lines.next().unwrap(), "Hit Points: {}", isize).unwrap();
        let damage = scan_fmt!(lines.next().unwrap(), "Damage: {}", isize).unwrap();
        let armor = scan_fmt!(lines.next().unwrap(), "Armor: {}", isize).unwrap();

        Character {
            hp: hp,
            damage: damage,
            armor: armor,
        }
    }

    fn reset(&mut self) {
        self.damage = 0;
        self.armor = 0;
    }

    fn equip(&mut self, items: &[&Item]) {
        self.reset();
        for item in items {
            self.damage += item.damage;
            self.armor += item.armor;
        }
    }

    fn attack(&self, boss: &Character) -> bool {
        let damage = max(1, self.damage - boss.armor);
        let boss_damage = max(1, boss.damage - self.armor);
        let mut hp = self.hp;
        let mut boss_hp = boss.hp;
        loop {
            boss_hp -= damage;
            if boss_hp < 1 {
                return true;
            }
            hp -= boss_damage;
            if hp < 1 {
                return false;
            }
        }
    }
}

fn equipment_builds() -> Box<Iterator<Item = [&'static Item<'static>; 4]>> {
    box RINGS.iter()
             .flat_map(move |r1| {
                 RINGS.iter()
                      .filter(move |&r2| r2 != r1)
                      .flat_map(move |r2| {
                          WEAPONS.iter()
                                 .flat_map(move |w| {
                                     ARMOR.iter()
                                          .map(move |a| [w, a, r1, r2])
                                 })
                      })
             })
}

fn least_expensive_win(boss: &Character) -> Option<usize> {
    let mut hero = Character::new();

    equipment_builds()
        .filter(|e| {
            hero.equip(e);
            hero.attack(&boss)
        })
        .map(|e| e.iter().fold(0, |cost, &i| cost + i.cost))
        .min()
}

fn most_expensive_loss(boss: &Character) -> Option<usize> {
    let mut hero = Character::new();

    equipment_builds()
        .filter(|e| {
            hero.equip(e);
            !hero.attack(&boss)
        })
        .map(|e| e.iter().fold(0, |cost, &i| cost + i.cost))
        .max()
}
