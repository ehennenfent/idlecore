use idlecore::*;
use std::io::{self, Write};

fn main() {
    // Load resources and recipes from config file
    let (resources, recipes) = match load_resources_and_recipes("test/copper.toml") {
        Ok((r, rec)) => (r, rec),
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };

    // Create the game
    let mut game = Game::new(resources);

    // Add recipes to game and store their info for menu
    let mut recipe_menu: Vec<(usize, String, String)> = Vec::new(); // (menu_number, display_name, recipe_key)
    for (display_name, recipe_key, recipe) in recipes {
        if let Err(e) = game.add_recipe(&recipe_key, recipe) {
            eprintln!("Failed to add recipe {}: {}", recipe_key, e);
            return;
        }
        let menu_number = recipe_menu.len() + 1;
        recipe_menu.push((menu_number, display_name, recipe_key));
    }

    let quit_number = recipe_menu.len() + 1;
    let max_choice = quit_number;

    println!("Available recipes:");
    for (menu_num, display_name, _) in &recipe_menu {
        println!("{}. {}", menu_num, display_name);
    }
    println!("{}. Quit", quit_number);
    println!();

    let mut last_action: Option<String> = None;

    loop {
        display_inventory(&game);
        print!(
            "\nEnter your choice (1-{}) or press Enter to repeat: ",
            max_choice
        );
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

        let mut matched = false;
        for (menu_num, display_name, recipe_key) in &recipe_menu {
            if action == menu_num.to_string() {
                match game.tick(recipe_key) {
                    Ok(_) => {
                        println!("{}...", display_name);
                        last_action = Some(menu_num.to_string());
                    }
                    Err(e) => println!("Error: {}", e),
                }
                matched = true;
                break;
            }
        }

        if !matched {
            if action == quit_number.to_string() {
                println!("Thanks for playing!");
                break;
            } else {
                println!(
                    "Invalid choice. Please enter 1-{} or press Enter to repeat.",
                    max_choice
                );
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
