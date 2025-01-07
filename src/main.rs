use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const FOV: f64 = std::f64::consts::FRAC_PI_3; // Угол обзора 60 градусов
const MAX_PLAYER_SPEED: f64 = 0.2; // Максимальна швидкість гравця
const MIN_PLAYER_SPEED: f64 = 0.05; // Мінімальна швидкість після зіткнення
const PLAYER_ACCELERATION: f64 = 0.02; // Прискорення гравця
const PLAYER_DECELERATION: f64 = 0.05; // Гальмування гравця
const ROTATION_SPEED: f64 = 0.1; // Швидкість повороту
const BULLET_SPEED: f64 = 5.0; // Швидкість кулі

fn main() -> Result<(), String> {
    // Ініціалізація SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Doom-like Engine", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Більша карта
    let map = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    // Позиція гравця
    let mut player_pos = (1.5, 1.5);
    let mut player_angle: f64 = 0.0;

    // Швидкість гравця
    let mut player_speed: f64 = 0.0;

    // Кулі
    let mut bullets: Vec<(f64, f64)> = Vec::new(); // (x, y) позиції куль на екрані

    'running: loop {
        // Обробка подій
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
                    if keycode == Keycode::Space {
                        // Додаємо кулю при натисканні пробілу
                        bullets.push((50.0, SCREEN_HEIGHT as f64 - 100.0)); // Початкова позиція кулі (зброя)
                    }
                }
                _ => {}
            }
        }

        // Управління
        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Прискорення гравця вперед/назад
        if keys.contains(&Keycode::W) {
            player_speed += PLAYER_ACCELERATION;
            if player_speed > MAX_PLAYER_SPEED {
                player_speed = MAX_PLAYER_SPEED;
            }
        } else if keys.contains(&Keycode::S) {
            player_speed -= PLAYER_ACCELERATION;
            if player_speed < -MAX_PLAYER_SPEED {
                player_speed = -MAX_PLAYER_SPEED;
            }
        } else {
            // Гальмування, якщо гравець не рухається
            if player_speed > 0.0 {
                player_speed -= PLAYER_DECELERATION;
                if player_speed < 0.0 {
                    player_speed = 0.0;
                }
            } else if player_speed < 0.0 {
                player_speed += PLAYER_DECELERATION;
                if player_speed > 0.0 {
                    player_speed = 0.0;
                }
            }
        }

        // Рух гравця
        let (new_x, new_y) = (
            player_pos.0 + player_angle.cos() * player_speed,
            player_pos.1 + player_angle.sin() * player_speed,
        );

        // Перевірка колізій
        if map[new_y as usize][new_x as usize] == 0 {
            player_pos = (new_x, new_y);
        } else {
            // Якщо гравець врізається в стіну, швидкість зменшується до мінімальної
            if player_speed > 0.0 {
                player_speed = MIN_PLAYER_SPEED;
            } else if player_speed < 0.0 {
                player_speed = -MIN_PLAYER_SPEED;
            }
        }

        // Поворот вліво/вправо
        if keys.contains(&Keycode::A) {
            player_angle -= ROTATION_SPEED;
        }
        if keys.contains(&Keycode::D) {
            player_angle += ROTATION_SPEED;
        }

        // Оновлення позиції куль
        for bullet in &mut bullets {
            // Напрямок кулі завжди в центр екрану
            let center_x = SCREEN_WIDTH as f64 / 2.0;
            let center_y = SCREEN_HEIGHT as f64 / 2.0;

            let dx = center_x - bullet.0;
            let dy = center_y - bullet.1;
            let length = (dx * dx + dy * dy).sqrt();

            // Нормалізуємо напрямок і рухаємо кулю
            if length > 0.0 {
                bullet.0 += (dx / length) * BULLET_SPEED;
                bullet.1 += (dy / length) * BULLET_SPEED;
            }
        }

        // Видалення куль, які досягли центру екрану
        bullets.retain(|bullet| {
            let center_x = SCREEN_WIDTH as f64 / 2.0;
            let center_y = SCREEN_HEIGHT as f64 / 2.0;
            let distance = (bullet.0 - center_x).hypot(bullet.1 - center_y);
            distance > 10.0 // Видаляємо кулю, якщо вона близько до центру
        });

        // Очистка екрану
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Raycasting
        for x in 0..SCREEN_WIDTH {
            let ray_angle = player_angle - FOV / 2.0 + (x as f64 / SCREEN_WIDTH as f64) * FOV;
            let (distance, _) = cast_ray(player_pos, ray_angle, &map);

            // Розрахунок висоти стіни
            let wall_height = (SCREEN_HEIGHT as f64 / (distance + 0.0001)) as u32;
            let wall_top = (SCREEN_HEIGHT / 2).saturating_sub(wall_height / 2);

            // Малювання стіни
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(x as i32, wall_top as i32, 1, wall_height))?;
        }

        // Малювання зброї (простий прямокутник зліва внизу)
        canvas.set_draw_color(Color::RGB(100, 100, 100)); // Сірий колір для зброї
        canvas.fill_rect(Rect::new(50, SCREEN_HEIGHT as i32 - 100, 50, 50))?;

        // Малювання куль
        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Червоний колір для куль
        for bullet in &bullets {
            canvas.fill_rect(Rect::new(bullet.0 as i32, bullet.1 as i32, 5, 5))?;
        }

        // Оновлення екрану
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
            return (f64::MAX, 0); // Нічого не знайдено
        }

        if map[y as usize][x as usize] != 0 {
            return ((x - pos.0).hypot(y - pos.1), map[y as usize][x as usize]);
        }
    }
}