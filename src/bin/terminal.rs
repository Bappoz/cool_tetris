use std::io::{self, Write};
use std::time::{Duration, Instant};
use tetris::engine::position::Pos;
use tetris::interface::tetris::{Direction, Tetris};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_cell_char(typ: Option<&str>) -> &str {
    match typ {
        Some("I") => "🟦",
        Some("O") => "🟨",
        Some("T") => "🟪",
        Some("S") => "🟩",
        Some("Z") => "🟥",
        Some("J") => "🔵",
        Some("L") => "🟧",
        _ => "⬛",
    }
}

fn draw_game(tetris: &Tetris) {
    clear_screen();
    
    println!("╔════════════════════════════════════════╗");
    println!("║           🎮 TETRIS GAME 🎮            ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  Score: {:<30} ║", tetris.score());
    println!("╚════════════════════════════════════════╝\n");

    // Draw grid
    println!("┌{}┐", "─".repeat(tetris.width() as usize * 2));
    
    for y in 0..tetris.height() {
        print!("│");
        for x in 0..tetris.width() {
            let pos = Pos(x, y);
            let cell = tetris.get(pos);
            print!("{}", get_cell_char(cell));
        }
        println!("│");
    }
    
    println!("└{}┘", "─".repeat(tetris.width() as usize * 2));

    if tetris.is_game_over() {
        println!("\n╔════════════════════════════════════════╗");
        println!("║           💀 GAME OVER! 💀             ║");
        println!("║      Final Score: {:<19} ║", tetris.score());
        println!("╚════════════════════════════════════════╝");
    } else {
        println!("\n📋 Controls:");
        println!("  a/d - Move Left/Right");
        println!("  w   - Rotate");
        println!("  s   - Soft Drop");
        println!("  x   - Hard Drop");
        println!("  r   - Restart");
        println!("  q   - Quit");
    }
}

fn main() {
    println!("Starting Tetris...");
    println!("Note: This is a simple terminal version.");
    println!("For better graphics, use the SDL2 version on Windows!");
    println!("\nPress Enter to start...");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut tetris = Tetris::new(10, 20);
    let mut last_tick = Instant::now();
    let tick_duration = Duration::from_millis(500);

    // Enable raw mode would be better but requires external crate
    // For now, this is a simplified version
    
    loop {
        // Auto tick
        if last_tick.elapsed() >= tick_duration {
            tetris.tick();
            last_tick = Instant::now();
        }

        draw_game(&tetris);

        // Simple input (line-based, not ideal but works without dependencies)
        println!("\nEnter command (a/d/w/s/x/r/q): ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd = input.trim().to_lowercase();
                match cmd.as_str() {
                    "a" => tetris.shift(Direction::Left),
                    "d" => tetris.shift(Direction::Right),
                    "w" => tetris.rotate(),
                    "s" => tetris.tick(),
                    "x" => tetris.drop(),
                    "r" => {
                        tetris.reset();
                        last_tick = Instant::now();
                    }
                    "q" => {
                        clear_screen();
                        println!("Thanks for playing! Final score: {}", tetris.score());
                        break;
                    }
                    "" => {
                        // Just redraw
                    }
                    _ => {
                        println!("Invalid command!");
                        std::thread::sleep(Duration::from_millis(500));
                    }
                }
            }
            Err(_) => break,
        }

        if tetris.is_game_over() {
            println!("\nPress 'r' to restart or 'q' to quit: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let cmd = input.trim().to_lowercase();
            if cmd == "r" {
                tetris.reset();
                last_tick = Instant::now();
            } else {
                break;
            }
        }
    }

    clear_screen();
    println!("👋 Goodbye!");
}
