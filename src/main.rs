use idlecore::*;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Set up resources
    let mut resources = ResourceMap::new();
    let copper_ore = resources.create_resource("copper_ore");
    let copper_bars = resources.create_resource("copper_bars");
    let copper_pickaxes = resources.create_resource("copper_pickaxes");

    // Create the game
    let mut game = Game::new(resources);

    // Create recipes
    // Mine copper ore: 1 every 60 ticks
    let mine_ore_recipe = Recipe {
        ingredients: HashMap::new(),
        outputs: {
            let mut map = HashMap::new();
            map.insert(copper_ore, 1.0);
            map
        },
        requires: HashMap::new(),
        ticks: 2,
    };

    // Smelt copper bars: 1 every 30 ticks, requires 1 copper ore
    let smelt_bars_recipe = Recipe {
        ingredients: {
            let mut map = HashMap::new();
            map.insert(copper_ore, 1.0);
            map
        },
        outputs: {
            let mut map = HashMap::new();
            map.insert(copper_bars, 1.0);
            map
        },
        requires: HashMap::new(),
        ticks: 3,
    };

    // Make copper pickaxes: 1 every 120 ticks, requires 2 copper bars
    let make_pickaxe_recipe = Recipe {
        ingredients: {
            let mut map = HashMap::new();
            map.insert(copper_bars, 2.0);
            map
        },
        outputs: {
            let mut map = HashMap::new();
            map.insert(copper_pickaxes, 1.0);
            map
        },
        requires: HashMap::new(),
        ticks: 6,
    };

    // Add recipes to game
    game.add_recipe("mine_ore", mine_ore_recipe).unwrap();
    game.add_recipe("smelt_bars", smelt_bars_recipe).unwrap();
    game.add_recipe("make_pickaxes", make_pickaxe_recipe)
        .unwrap();

    println!("Welcome to the Copper Idle Game!");
    println!("Available recipes:");
    println!("1. Mine copper ore");
    println!("2. Smelt copper bars");
    println!("3. Make copper pickaxes");
    println!("4. Quit");
    println!();

    let mut last_action: Option<String> = None;

    loop {
        display_inventory(&game);
        print!("\nEnter your choice (1-4) or press Enter to repeat: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        let action = if choice.is_empty() {
            if let Some(ref last) = last_action {
                last.clone()
            } else {
                println!("No previous action to repeat. Please select an action first.");
                continue;
            }
        } else {
            choice.to_string()
        };

        match action.as_str() {
            "1" => {
                match game.tick("mine_ore") {
                    Ok(_) => {
                        println!("Mining copper ore...");
                        last_action = Some("1".to_string());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                match game.tick("smelt_bars") {
                    Ok(_) => {
                        println!("Smelting copper bars...");
                        last_action = Some("2".to_string());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                match game.tick("make_pickaxes") {
                    Ok(_) => {
                        println!("Making copper pickaxes...");
                        last_action = Some("3".to_string());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                println!("Thanks for playing!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1-4 or press Enter to repeat.");
            }
        }

        println!("Time: {}", game.time);
    }
}

fn display_inventory(game: &Game) {
    println!("\n=== Current Inventory ===");
    for resource_name in &game.resources.resource_names {
        let resource = game.resources.get_resource(resource_name);
        let amount = game.inventory[resource];
        println!("{}: {:.1}", resource_name, amount);
    }
}
