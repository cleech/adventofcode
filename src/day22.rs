use std::cmp::max;

const DATA: &'static str = include_str!("../data/input_22.txt");

pub fn main() -> Vec<String> {
    let mut hero = Character::wizard();
    let mut boss = Character::boss(DATA);
    hero.attack(&mut boss);
    vec![]
}

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
                println!("Player casts Magic Missile, dealing 4 damage.");
                enemy.hp -= 4;
            }
            Spell::Drain => {
                println!("Player casts Drain, dealing 2 damage, and healing 2 hit points.");
                enemy.hp -= 2;
                caster.hp += 2;
            }
            Spell::Shield => {
                println!("Player casts Shield, increasing armor by 7.");
                caster.armor += 7;
                caster.install(Effect::Shield {
                    turns: 6,
                    armor: 7,
                })
            }
            Spell::Poison => {
                println!("Player casts Poison.");
                caster.install(Effect::Poison {
                    turns: 6,
                    damage: 3,
                })
            }
            Spell::Recharge => {
                println!("Player casts Recharge.");
                caster.install(Effect::Recharge {
                    turns: 5,
                    mana: 101,
                })
            }
        }
    }
}

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

    fn boss(data: &str) -> Character {
        let mut lines = data.lines();
        let hp = scan_fmt!(lines.next().unwrap(), "Hit Points: {}", isize).unwrap();
        let damage = scan_fmt!(lines.next().unwrap(), "Damage: {}", isize).unwrap();

        Character {
            hp: hp,
            damage: damage,
            ..Character::new()
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
            println!("Shield's timer is now {}.", turns);
            if *turns == 0 {
                println!("Shield wears off, decreasing armor by 7.");
                self.armor -= armor;
                clear_shield = true;
            }
        }
        if clear_shield {
            self.shield = None;
        }

        if let Some(Effect::Poison{ref mut turns, damage}) = self.poison {
            *turns -= 1;
            println!("Poison's deals {} damage; its timer is now {}.",
                     damage,
                     turns);
            boss.hp -= damage;
            if *turns == 0 {
                println!("Poison wears off.");
                clear_poison = true;
            }
        }
        if clear_poison {
            self.poison = None;
        }

        if let Some(Effect::Recharge{ref mut turns, mana}) = self.recharge {
            *turns -= 1;
            println!("Recharge provides {} mana; its timer is now {}.",
                     mana,
                     turns);
            self.mana += mana;
            if *turns == 0 {
                println!("Recharge wears off.");
                clear_recharge = true;
            }
        }
        if clear_recharge {
            self.recharge = None;
        }
    }

    fn attack(&mut self, boss: &mut Character) -> bool {
        loop {
            println!("\n-- Player turn --");
            println!("- Player has {} hit points, {} armor, {} mana",
                     self.hp,
                     self.armor,
                     self.mana);
            println!("- Boss has {} hit points", boss.hp);
            self.run_effects(boss);

            // FIXME pick a spell
            // self.valid_spells().get(0).unwrap().cast(self, boss);
            self.valid_spells().last().unwrap().cast(self, boss);

            if boss.hp < 1 {
                println!("This kills the boss, and the player wins");
                println!("## total mana spent was {} ##", self.mana_spent);
                return true;
            }

            println!("\n-- Boss turn --");
            println!("- Player has {} hit points, {} armor, {} mana",
                     self.hp,
                     self.armor,
                     self.mana);
            println!("- Boss has {} hit points", boss.hp);
            self.run_effects(boss);

            let damage = max(1, boss.damage - self.armor);
            println!("Boss attacks for {} - {} = {} damage!",
                     boss.damage,
                     self.armor,
                     damage);
            self.hp -= damage;
            if self.hp < 1 {
                println!("This kills the player, and the boss wins");
                return false;
            }
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

