use std::cmp::{max, min};
use std::isize;

const DATA: &'static str = include_str!("../data/input_22.txt");

pub fn main() -> Vec<String> {
    let hero = Character::wizard();
    let boss = Character::boss(DATA).unwrap();
    let s1 = find_best_solution(&hero, &boss, 0, isize::MAX);
    let s2 = find_best_solution(&hero, &boss, 1, isize::MAX);
    vec![s1.unwrap().to_string(), s2.unwrap().to_string()]
}

#[derive(Clone)]
enum Effect {
    Shield {
        turns: usize,
        armor: isize,
    },
    Poison {
        turns: usize,
        damage: isize,
    },
    Recharge {
        turns: usize,
        mana: isize,
    },
}

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> isize {
        match *self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn cast(&self, caster: &mut Character, enemy: &mut Character) {
        let cost = self.cost();
        caster.mana -= cost;
        caster.mana_spent += cost;
        match *self {
            Spell::MagicMissile => {
                // println!("Player casts Magic Missile, dealing 4 damage.");
                enemy.hp -= 4;
            }
            Spell::Drain => {
                // println!("Player casts Drain, dealing 2 damage, and healing 2 hit points.");
                enemy.hp -= 2;
                caster.hp += 2;
            }
            Spell::Shield => {
                // println!("Player casts Shield, increasing armor by 7.");
                caster.armor += 7;
                caster.install(Effect::Shield {
                    turns: 6,
                    armor: 7,
                })
            }
            Spell::Poison => {
                // println!("Player casts Poison.");
                caster.install(Effect::Poison {
                    turns: 6,
                    damage: 3,
                })
            }
            Spell::Recharge => {
                // println!("Player casts Recharge.");
                caster.install(Effect::Recharge {
                    turns: 5,
                    mana: 101,
                })
            }
        }
    }
}

#[derive(Clone)]
struct Character {
    hp: isize,
    damage: isize,
    armor: isize,
    mana: isize,
    // eh, not very general but that's OK
    shield: Option<Effect>,
    poison: Option<Effect>,
    recharge: Option<Effect>,
    mana_spent: isize,
}

impl Character {
    fn new() -> Character {
        Character {
            hp: 0,
            damage: 0,
            armor: 0,
            mana: 0,
            shield: None,
            poison: None,
            recharge: None,
            mana_spent: 0,
        }
    }

    fn wizard() -> Character {
        Character {
            hp: 50,
            mana: 500,
            ..Character::new()
        }
    }

    fn boss(data: &str) -> Option<Character> {
        let mut lines = data.lines();

        if let (Some(hp), Some(damage)) = (lines.next().map_or(None, |l| {
            scan_fmt!(l, "Hit Points: {}", isize)
        }),
                                           lines.next().map_or(None, |l| {
            scan_fmt!(l, "Damage: {}", isize)
        })) {

            Some(Character {
                hp: hp,
                damage: damage,
                ..Character::new()
            })
        } else {
            None
        }
    }

    fn install(&mut self, effect: Effect) {
        match effect {
            s @ Effect::Shield{..} => self.shield = Some(s),
            p @ Effect::Poison{..} => self.poison = Some(p),
            r @ Effect::Recharge{..} => self.recharge = Some(r),
        }
    }

    fn run_effects(&mut self, boss: &mut Character) {
        let mut clear_shield = false;
        let mut clear_poison = false;
        let mut clear_recharge = false;

        if let Some(Effect::Shield{ref mut turns, armor}) = self.shield {
            *turns -= 1;
            // println!("Shield's timer is now {}.", turns);
            if *turns == 0 {
                // println!("Shield wears off, decreasing armor by 7.");
                self.armor -= armor;
                clear_shield = true;
            }
        }
        if clear_shield {
            self.shield = None;
        }

        if let Some(Effect::Poison{ref mut turns, damage}) = self.poison {
            *turns -= 1;
            // println!("Poison's deals {} damage; its timer is now {}.", damage, turns);
            boss.hp -= damage;
            if *turns == 0 {
                // println!("Poison wears off.");
                clear_poison = true;
            }
        }
        if clear_poison {
            self.poison = None;
        }

        if let Some(Effect::Recharge{ref mut turns, mana}) = self.recharge {
            *turns -= 1;
            // println!("Recharge provides {} mana; its timer is now {}.", mana, turns);
            self.mana += mana;
            if *turns == 0 {
                // println!("Recharge wears off.");
                clear_recharge = true;
            }
        }
        if clear_recharge {
            self.recharge = None;
        }
    }

    fn valid_spells(&self) -> Vec<Spell> {
        let mut choices = Vec::new();

        if self.mana >= Spell::MagicMissile.cost() {
            choices.push(Spell::MagicMissile);
        }
        if self.mana >= Spell::Drain.cost() {
            choices.push(Spell::Drain);
        }
        if self.mana >= Spell::Shield.cost() && self.shield.is_none() {
            choices.push(Spell::Shield);
        }
        if self.mana >= Spell::Poison.cost() && self.poison.is_none() {
            choices.push(Spell::Poison);
        }
        if self.mana >= Spell::Recharge.cost() && self.recharge.is_none() {
            choices.push(Spell::Recharge);
        }
        choices
    }
}

fn find_best_solution(h: &Character, b: &Character, hard: isize, mut best: isize) -> Option<isize> {
    if h.mana_spent >= best {
        return None;
    }

    let mut h1 = h.clone();
    let mut b1 = b.clone();

    // println!("\n-- Player turn --");
    // println!("- Player has {} hit points, {} armor, {} mana", hero.hp, hero.armor, hero.mana);
    // println!("- Boss has {} hit points", boss.hp);

    h1.hp -= hard;

    if h1.hp < 1 {
        return None;
    }

    h1.run_effects(&mut b1);

    if b1.hp < 1 {
        return Some(h1.mana_spent);
    }

    h1.valid_spells()
        .iter()
        .filter_map(|spell| {
            let mut hero = h1.clone();
            let mut boss = b1.clone();

            spell.cast(&mut hero, &mut boss);

            if boss.hp < 1 {
                // println!("This kills the boss, and the player wins");
                // println!("## total mana spent was {} ##", hero.mana_spent);
                return Some(hero.mana_spent);
            }

            // println!("\n-- Boss turn --");
            // println!("- Player has {} hit points, {} armor, {} mana", hero.hp, hero.armor, hero.mana);
            // println!("- Boss has {} hit points", boss.hp);

            hero.run_effects(&mut boss);

            if boss.hp < 1 {
                return Some(hero.mana_spent);
            }

            let damage = max(1, boss.damage - hero.armor);

            // println!("Boss attacks for {} - {} = {} damage!", boss.damage, hero.armor, damage);

            hero.hp -= damage;

            if hero.hp < 1 {
                // println!("This kills the player, and the boss wins");
                return None;
            }

            let r = find_best_solution(&hero, &boss, hard, best);
            if let Some(new_best) = r {
               best = min(best, new_best);
            }
            r
        })
        .min()
}
