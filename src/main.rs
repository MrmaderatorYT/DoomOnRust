use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const FOV: f64 = std::f64::consts::FRAC_PI_3; // Угол обзора 60 градусов

fn main() -> Result<(), String> {
    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Doom-like Engine", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Карта
    let map = vec![
        vec![1, 1, 1, 1],
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 1],
        vec![1, 1, 1, 1],
    ];

    // Позиция игрока
    let mut player_pos = (1.5, 1.5);
    let mut player_angle: f64 = 0.0;

    'running: loop {
        // Обработка событий
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'running;
                    }
                }
                _ => {}
            }
        }

        // Управление
        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        if keys.contains(&Keycode::W) {
            player_pos.0 += player_angle.cos() * 0.1;
            player_pos.1 += player_angle.sin() * 0.1;
        }
        if keys.contains(&Keycode::S) {
            player_pos.0 -= player_angle.cos() * 0.1;
            player_pos.1 -= player_angle.sin() * 0.1;
        }
        if keys.contains(&Keycode::A) {
            player_angle -= 0.1;
        }
        if keys.contains(&Keycode::D) {
            player_angle += 0.1;
        }

        // Очистка экрана
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Raycasting
        for x in 0..SCREEN_WIDTH {
            let ray_angle = player_angle - FOV / 2.0 + (x as f64 / SCREEN_WIDTH as f64) * FOV;
            let (distance, _) = cast_ray(player_pos, ray_angle, &map);

            // Рассчет высоты стены
            let wall_height = (SCREEN_HEIGHT as f64 / (distance + 0.0001)) as u32;
            let wall_top = (SCREEN_HEIGHT / 2).saturating_sub(wall_height / 2);
            let wall_bottom = (SCREEN_HEIGHT / 2) + (wall_height / 2);

            // Отрисовка стены
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(x as i32, wall_top as i32, 1, wall_height))?;
        }

        // Обновление экрана
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

/// Бросок луча
fn cast_ray(pos: (f64, f64), angle: f64, map: &Vec<Vec<i32>>) -> (f64, i32) {
    let (mut x, mut y) = pos;
    let (dx, dy) = (angle.cos() * 0.01, angle.sin() * 0.01);

    loop {
        x += dx;
        y += dy;

        if y < 0.0 || y >= map.len() as f64 || x < 0.0 || x >= map[0].len() as f64 {
            return (f64::MAX, 0); // Ничего не найдено
        }

        if map[y as usize][x as usize] != 0 {
            return ((x - pos.0).hypot(y - pos.1), map[y as usize][x as usize]);
        }
    }
}