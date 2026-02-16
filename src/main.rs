use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use tetris::engine::position::Pos;
use tetris::interface::tetris::{Direction, Tetris};

const CELL_SIZE: u32 = 30;
const GRID_PADDING: i32 = 10;

fn get_color_for_type(typ: &str) -> Color {
    match typ {
        "I" => Color::RGB(0, 212, 255),    // Ciano
        "O" => Color::RGB(255, 215, 0),    // Amarelo
        "T" => Color::RGB(168, 85, 247),   // Roxo
        "S" => Color::RGB(16, 185, 129),   // Verde
        "Z" => Color::RGB(239, 68, 68),    // Vermelho
        "J" => Color::RGB(59, 130, 246),   // Azul
        "L" => Color::RGB(249, 115, 22),   // Laranja
        _ => Color::RGB(15, 52, 96),       // Vazio (azul escuro)
    }
}

fn draw_cell(canvas: &mut WindowCanvas, pos: Pos, cell_type: &str, grid_offset_x: i32, grid_offset_y: i32) {
    let x = grid_offset_x + pos.0 * CELL_SIZE as i32;
    let y = grid_offset_y + pos.1 * CELL_SIZE as i32;
    
    let color = get_color_for_type(cell_type);
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x + 1, y + 1, CELL_SIZE - 2, CELL_SIZE - 2)).unwrap();
    
    // Borda mais clara para células preenchidas
    if cell_type != "empty" {
        let lighter = Color::RGB(
            color.r.saturating_add(40),
            color.g.saturating_add(40),
            color.b.saturating_add(40),
        );
        canvas.set_draw_color(lighter);
        canvas.draw_rect(Rect::new(x + 1, y + 1, CELL_SIZE - 2, CELL_SIZE - 2)).unwrap();
    }
}

fn draw_text(canvas: &mut WindowCanvas, text: &str, x: i32, y: i32, _size: u32) {
    // Simulação simples de texto com retângulos (para demonstração)
    // Em produção, você usaria SDL2_ttf
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(Rect::new(x, y, text.len() as u32 * 8, 12)).unwrap();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let grid_width = 10;
    let grid_height = 20;
    let info_panel_width = 250;
    
    let window_width = (grid_width * CELL_SIZE + 2 * GRID_PADDING as u32 + info_panel_width) as u32;
    let window_height = (grid_height * CELL_SIZE + 2 * GRID_PADDING as u32) as u32;

    let window = video_subsystem
        .window("🎮 Tetris Game", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut tetris = Tetris::new(grid_width, grid_height);
    let mut frame_count = 0;
    let frames_per_tick = 30; // Tick a cada 30 frames (~500ms a 60fps)

    'running: loop {
        // Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Left => tetris.shift(Direction::Left),
                    Keycode::Right => tetris.shift(Direction::Right),
                    Keycode::Down => tetris.tick(),
                    Keycode::Up | Keycode::W => tetris.rotate(),
                    Keycode::Space => tetris.drop(),
                    Keycode::R => tetris.reset(),
                    _ => {}
                },
                _ => {}
            }
        }

        // Auto-tick
        frame_count += 1;
        if frame_count >= frames_per_tick {
            tetris.tick();
            frame_count = 0;
        }

        // Clear canvas
        canvas.set_draw_color(Color::RGB(17, 17, 30));
        canvas.clear();

        let grid_offset_x = GRID_PADDING;
        let grid_offset_y = GRID_PADDING;

        // Draw grid background
        canvas.set_draw_color(Color::RGB(26, 26, 46));
        canvas.fill_rect(Rect::new(
            grid_offset_x - 5,
            grid_offset_y - 5,
            (grid_width * CELL_SIZE) as u32 + 10,
            (grid_height * CELL_SIZE) as u32 + 10,
        ))?;

        // Draw cells
        for pos in tetris.iter_position() {
            let cell_type = tetris.get(pos).unwrap_or("empty");
            draw_cell(&mut canvas, pos, cell_type, grid_offset_x, grid_offset_y);
        }

        // Draw info panel
        let info_x = (grid_width * CELL_SIZE) as i32 + GRID_PADDING + 30;
        let info_y = GRID_PADDING;

        // Title
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_text(&mut canvas, "TETRIS", info_x, info_y, 32);

        // Score
        let score_text = format!("Score: {}", tetris.score());
        canvas.set_draw_color(Color::RGB(100, 200, 255));
        canvas.fill_rect(Rect::new(info_x, info_y + 60, 200, 60))?;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_text(&mut canvas, &score_text, info_x + 10, info_y + 80, 24);

        // Status
        let status = if tetris.is_game_over() {
            "GAME OVER!"
        } else {
            "Playing..."
        };
        let status_color = if tetris.is_game_over() {
            Color::RGB(239, 68, 68)
        } else {
            Color::RGB(16, 185, 129)
        };
        canvas.set_draw_color(status_color);
        canvas.fill_rect(Rect::new(info_x, info_y + 140, 200, 40))?;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw_text(&mut canvas, status, info_x + 10, info_y + 150, 16);

        // Controls info
        let controls_y = info_y + 220;
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        draw_text(&mut canvas, "Controls:", info_x, controls_y, 18);
        draw_text(&mut canvas, "< > : Move", info_x, controls_y + 30, 14);
        draw_text(&mut canvas, "^ / W : Rotate", info_x, controls_y + 50, 14);
        draw_text(&mut canvas, "v : Soft Drop", info_x, controls_y + 70, 14);
        draw_text(&mut canvas, "Space : Hard Drop", info_x, controls_y + 90, 14);
        draw_text(&mut canvas, "R : Restart", info_x, controls_y + 110, 14);
        draw_text(&mut canvas, "ESC : Quit", info_x, controls_y + 130, 14);

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
