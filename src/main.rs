use rand::Rng;
use std::process::exit;
use std::thread;
use std::io::{self, Write};
use std::time::Duration;

struct Character {
    name: String,
    health: i8,
    attack: i8,
    defense: i8,
    gold: i8,
}

// Helper functions related to dice rolling
fn roll_dice(min: i8, max: i8) -> i8 {
    rand::thread_rng().gen_range(min..=max)
}

fn roll_d20() -> i8 {
    roll_dice(1, 20)
}

fn roll_d6() -> i8 {
    roll_dice(1, 6)
}

// Helper function related to advancing the game
fn tick() -> bool {
    let hostile_index = roll_d6();
    let player_index = roll_d6();
    
    hostile_index == player_index
}

// Sleep for game pacing
fn sleep_two_seconds() {
    sleep_seconds(2);
}

fn sleep_seconds(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

// Print the current stats of the character
fn print_score(character: &Character) {
    println!("{} has found {} gold.", character.name, character.gold);
    println!("Stats: ");
    println!("Health: {}", character.health);
    println!("Attack: {}", character.attack);
    println!("Defense: {}", character.defense);
}

fn create_enemy() -> Character {
    let first_names = vec!["Goblin", "Orc", "Troll", "Dragon", "Giant"];
    let last_names = vec!["Warrior", "Ranger", "Mage", "Cleric", "Priest"];

    let chosen_first_name_index = roll_dice(0, first_names.len() as i8 - 1) as usize;
    let chosen_first_name = first_names[chosen_first_name_index];

    let chosen_last_name_index = roll_dice(0, last_names.len() as i8 - 1) as usize;
    let chosen_last_name = last_names[chosen_last_name_index];

    Character {
        name: format!("{} {}", chosen_first_name, chosen_last_name),
        health: (roll_d6() * 10).min(127),
        attack: roll_d6(),
        defense: roll_d6(),
        gold: roll_dice(0, 3),
    }
}

fn create_character() -> Character {
    let first_names = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Henry", "Ivy", "Jack"];
    let last_names = vec!["Kilroy", "Great", "Mischievous", "Norris", "Oswald", "Parker", "Quince", "Robinson", "Smith", "Taylor"];

    let chosen_first_name_index = roll_dice(0, first_names.len() as i8 - 1) as usize;
    let chosen_first_name = first_names[chosen_first_name_index];

    let chosen_last_name_index = roll_dice(0, last_names.len() as i8 - 1) as usize;
    let chosen_last_name = last_names[chosen_last_name_index];

    let name = format!("{} {}", chosen_first_name, chosen_last_name);

    Character {
        name,
        health: roll_dice(80, 100),
        attack: roll_dice(5, 10),
        defense: roll_dice(5, 10),
        gold: 0,
    }
}

fn find_treasure(player_character: &mut Character) {
    let result = roll_dice(0, 100) as i32;
    if result < 10 {
        println!("You found nothing...");
        return;
    }

    println!("You found something... ({})", result);
    sleep_two_seconds();

    match result {
        11..=39 => {
            println!("You found +1 gold!");
            player_character.gold = (player_character.gold + 1).min(127);
        }
        40..=59 => {
            println!("You found +1 health!");
            player_character.health = (player_character.health + 1).min(127);
        }
        60..=79 => {
            println!("You found +1 attack!");
            player_character.attack = (player_character.attack + 1).min(127);
        }
        80..=89 => {
            println!("You found +1 defense!");
            player_character.defense = (player_character.defense + 1).min(127);
        }
        90..=99 => {
            println!("You found a variety of treasures!");
            player_character.gold = (player_character.gold + 1).min(127);
            player_character.health = (player_character.health + 1).min(127);
            player_character.attack = (player_character.attack + 1).min(127);
            player_character.defense = (player_character.defense + 1).min(127);
        }
        100 => {
            println!("You found the legendary treasure! +10 to all stats!");
            player_character.gold = (player_character.gold + 10).min(127);
            player_character.health = (player_character.health + 10).min(127);
            player_character.attack = (player_character.attack + 10).min(127);
            player_character.defense = (player_character.defense + 10).min(127);
        }
        _ => unreachable!(),
    }
}

fn attack_turn(attacker: &Character, defender: &mut Character) {
    println!("{} attacks {}!", attacker.name, defender.name);
    sleep_two_seconds();
    
    let attack_roll = roll_d20() as i32 + attacker.attack as i32;
    println!("{} rolled {} for attack", attacker.name, attack_roll);
    sleep_two_seconds();
    
    if attack_roll > defender.defense as i32 {
        let damage = roll_d6();
        defender.health = (defender.health - damage).max(-128);
        println!("{} hits for {} damage!", attacker.name, damage);
    } else {
        println!("{} misses!", attacker.name);
    }
    
    println!("{} has {} health remaining", defender.name, defender.health);
    sleep_two_seconds();
}

fn roll_initiative(player: &Character, enemy: &Character) -> bool {
    loop {
        println!("Rolling for initiative...");
        sleep_two_seconds();

        let player_roll = roll_d20();
        println!("{} rolled {}", player.name, player_roll);
        sleep_two_seconds();

        let enemy_roll = roll_d20();
        println!("{} rolled {}", enemy.name, enemy_roll);
        sleep_two_seconds();

        if player_roll > enemy_roll {
            println!("{} wins the initiative", player.name);
            return true;
        } else if enemy_roll > player_roll {
            println!("{} wins the initiative", enemy.name);
            return false;
        } else {
            println!("It's a tie! Rolling again...");
            sleep_two_seconds();
        }
    }
}

fn combat_sequence(player_character: &mut Character, hostile_character: &mut Character) {
    println!("{} has engaged in combat with {}", player_character.name, hostile_character.name);

    let mut player_turn = roll_initiative(player_character, hostile_character);

    while player_character.health > 0 && hostile_character.health > 0 {
        if player_turn {
            attack_turn(player_character, hostile_character);
            if hostile_character.health <= 0 { break; }
        } else {
            attack_turn(hostile_character, player_character);
            if player_character.health <= 0 { break; }
        }
        player_turn = !player_turn; // Switch turns
    }

    if player_character.health > 0 {
        println!("{} has defeated {}!", player_character.name, hostile_character.name);
        let gold_reward = hostile_character.gold;
        player_character.gold = (player_character.gold + gold_reward).min(127);
        println!("{} gained {} gold!", player_character.name, gold_reward);
    } else {
        println!("{} has been defeated by {}!", player_character.name, hostile_character.name);
        println!("Game Over!");
        exit(0);
    }
}

fn game_loop(player_character: &mut Character) {
    loop {
        let mut encounter = tick();
        if encounter {
            let mut hostile_character = create_enemy();
            combat_sequence(player_character, &mut hostile_character);
        } else {
            find_treasure(player_character);
        }

        print_score(player_character);
        print!("Would you like to continue? (yes/no): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "yes" => continue,
            "no" => {
                println!("\nFinal score:");
                print_score(player_character);
                println!("Thanks for playing!");
                return;
            }
            _ => {
                println!("Invalid input. Please enter 'yes' or 'no'.");
                continue;
            }
        }
    }
}

fn main() {
    println!("Hello, welcome to Monster Battler!");
    println!("The goal of monster battler is to create your hero and defeat the monsters in the game");

    println!("Choose your character: ");
    let mut characters = vec![create_character(), create_character(), create_character()];

    for (index, character) in characters.iter().enumerate() {
        println!("{}: {}", index + 1, character.name);
        println!("Health: {}", character.health);
        println!("Attack: {}", character.attack);
        println!("Defense: {}", character.defense);
        println!();
    }

    println!("Enter the number of the character you want to use: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let character_index = usize::from_str_radix(&input.trim(), 10).expect("Invalid input");
    
    let mut player_character = characters.remove(character_index - 1);
    println!("You chose: {} as your champion", player_character.name);

    game_loop(&mut player_character);
}