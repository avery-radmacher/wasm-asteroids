use crate::game::{Asteroid, Bullet, Explosion, Game, InputIndex};
use crate::math::Vec2D;
use std::fmt::Write;

fn draw(buf: &mut String, line: bool, point: &Vec2D) {
    write!(
        buf,
        "{}{:.2} {:.2} ",
        if line { 'L' } else { 'M' },
        point.x,
        point.y
    )
    .expect("could not write string");
}

fn draw_points(buf: &mut String, points: &Vec<Vec2D>) {
    if points.is_empty() {
        return;
    }
    draw(buf, false, &points[0]);
    for &point in &points[1..] {
        draw(buf, true, &point);
    }
}

fn calculate_wrap(points: &Vec<Vec2D>, field_size: &Vec2D, x: bool) -> f64 {
    let field_size = if x { field_size.x } else { field_size.y };
    points
        .iter()
        .map(|point| if x { point.x } else { point.y })
        .filter_map(|x| {
            if x >= field_size {
                Some(-1.0)
            } else if x < 0.0 {
                Some(1.0)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap_or_default()
}

fn translate(points: &Vec<Vec2D>, translation: &Vec2D) -> Vec<Vec2D> {
    points.iter().map(|&point| point + *translation).collect()
}

fn draw_points_wrapping(buf: &mut String, points: &Vec<Vec2D>, field_size: &Vec2D) {
    let x_wrap = calculate_wrap(&points, &field_size, true);
    let y_wrap = calculate_wrap(&points, &field_size, false);
    if x_wrap != 0.0 {
        if y_wrap != 0.0 {
            let translated_points = translate(
                &points,
                &Vec2D {
                    x: field_size.x * x_wrap,
                    y: field_size.y * y_wrap,
                },
            );
            draw_points(buf, &translated_points);
        }
        let translated_points = translate(
            &points,
            &Vec2D {
                x: field_size.x * x_wrap,
                y: 0.0,
            },
        );
        draw_points(buf, &translated_points);
    }
    if y_wrap != 0.0 {
        let translated_points = translate(
            &points,
            &Vec2D {
                x: 0.0,
                y: field_size.y * y_wrap,
            },
        );
        draw_points(buf, &translated_points);
    }
    draw_points(buf, points);
}

fn draw_object(
    buf: &mut String,
    points: &Vec<Vec2D>,
    scale: f64,
    rotation: f64,
    offset: &Vec2D,
    field_size: &Vec2D,
) {
    draw_points_wrapping(
        buf,
        &points
            .iter()
            .map(|p| p.scale(scale).rotate(rotation) + *offset)
            .collect(),
        field_size,
    );
}

const SHIP_POINTS: &[Vec2D] = &[
    Vec2D { x: 10.0, y: 0.0 },
    Vec2D { x: -10.0, y: -5.0 },
    Vec2D { x: -8.0, y: -2.5 },
    Vec2D { x: -8.0, y: 2.5 },
    Vec2D { x: -10.0, y: 5.0 },
    Vec2D { x: 10.0, y: 0.0 },
];

const FLARE: &[Vec2D] = &[
    Vec2D { x: -8.0, y: 1.5 },
    Vec2D { x: -12.0, y: 0.0 },
    Vec2D { x: -8.0, y: -1.5 },
];

fn render_ship(buf: &mut String, game: &Game) {
    let ship = &game.ship;
    if ship.dead {
        return;
    }
    draw_object(
        buf,
        &SHIP_POINTS.to_vec(),
        2.0,
        ship.angle,
        &ship.pos,
        &game.config.field_size,
    );
    let inputs = &game.inputs;
    if inputs.is_down(InputIndex::Forward) || inputs.is_down(InputIndex::Backward) {
        draw_object(
            buf,
            &FLARE.to_vec(),
            2.0,
            ship.angle,
            &ship.pos,
            &game.config.field_size,
        );
    }
}

fn render_bullet(buf: &mut String, bullet: &Bullet, field_size: &Vec2D) {
    let tail = bullet.pos + bullet.speed.normalize().scale(5.0);
    draw_points_wrapping(buf, &vec![bullet.pos, tail], field_size);
}

fn render_asteroid(buf: &mut String, asteroid: &Asteroid, field_size: &Vec2D) {
    let cnt = asteroid.style;
    let angle = std::f64::consts::TAU / (cnt as f64);
    let asteroid_points = vec![Vec2D::one(); cnt + 1]
        .iter()
        .enumerate()
        .map(|(i, v)| v.rotate(angle * (i as f64)))
        .collect();
    draw_object(
        buf,
        &asteroid_points,
        asteroid.size,
        asteroid.angle,
        &asteroid.pos,
        field_size,
    );
}

fn render_lives(buf: &mut String, lives: u64, field_size: &Vec2D) {
    const LIFE_STEP: f64 = 40.0;
    const UP_ANGLE: f64 = std::f64::consts::PI * -0.5;
    for l in 0..lives {
        let y = 50.0;
        let x = ((l + 1) as f64) * LIFE_STEP;
        draw_object(
            buf,
            &SHIP_POINTS.to_vec(),
            2.0,
            UP_ANGLE,
            &Vec2D { x, y },
            field_size,
        );
    }
}

fn render_explosion(buf: &mut String, explosion: &Explosion, tick: u64, field_size: &Vec2D) {
    const EXPLOSION_RADIUS: f64 = 30.0;
    const EXPLOSION_PARTICLES: usize = 11;
    const EXPLOSION_PARTICLE_LENGTH: f64 = 10.0;
    let explosion_da = std::f64::consts::PI * 2.0 / (EXPLOSION_PARTICLES as f64);
    let state = ((tick - explosion.start_tick) as f64)
        / ((explosion.lifetime - explosion.start_tick) as f64);

    for i in 0..EXPLOSION_PARTICLES {
        let a = explosion_da * (i as f64);
        let dir = Vec2D::one().rotate(a);
        let start = dir.scale(state * EXPLOSION_RADIUS) + explosion.pos;
        let end = dir.scale(state * EXPLOSION_RADIUS + EXPLOSION_PARTICLE_LENGTH * (1.0 + state))
            + explosion.pos;
        draw_points_wrapping(buf, &vec![start, end], field_size);
    }
}

const VECTOR_DIGITS: &[&[Vec2D]] = &[
    // 0
    &[
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
    ],
    // 1
    &[
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 1.0, y: 3.0 },
    ],
    // 2
    &[
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 0.0, y: 3.0 },
        Vec2D { x: 2.0, y: 3.0 },
    ],
    // 3
    &[
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 1.0, y: 1.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 0.0, y: 3.0 },
    ],
    // 4
    &[
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 2.0, y: 3.0 },
    ],
    // 5
    &[
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 1.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 0.0, y: 3.0 },
    ],
    // 6
    &[
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 1.0 },
        Vec2D { x: 0.0, y: 1.0 },
    ],
    // 7
    &[
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 1.0, y: 3.0 },
    ],
    // 8
    &[
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 0.0, y: 3.0 },
        Vec2D { x: 2.0, y: 3.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 0.0, y: 0.0 },
    ],
    // 9
    &[
        Vec2D { x: 0.0, y: 3.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 2.0 },
        Vec2D { x: 2.0, y: 2.0 },
    ],
];

fn render_score(buf: &mut String, mut score: u64, field_size: &Vec2D) {
    let mut digits = Vec::new();
    while score > 0 {
        digits.push(score % 10);
        score /= 10;
    }
    if digits.is_empty() {
        digits.push(0);
    }
    const DIGIT_SCALE: f64 = 10.0;
    const DIGIT_STEP: f64 = -3.0;
    const DIGIT_RIGHTMOST: f64 = 120.0;
    for (idx, d) in digits.iter().enumerate() {
        let digit = VECTOR_DIGITS[*d as usize];
        let offset = Vec2D {
            x: DIGIT_RIGHTMOST + (idx as f64) * DIGIT_STEP,
            y: 5.0,
        };
        let points = digit
            .iter()
            .map(|&p| (p + offset).scale(DIGIT_SCALE))
            .collect();
        draw_points_wrapping(buf, &points, field_size);
    }
}

pub fn render_game(buf: &mut String, game: &Game) {
    let field_size = game.config.field_size;
    render_lives(buf, game.lives, &field_size);
    render_ship(buf, game);
    for bullet in game.bullets.iter() {
        render_bullet(buf, bullet, &field_size);
    }
    for asteroid in game.asteroids.iter() {
        render_asteroid(buf, asteroid, &field_size);
    }
    for explosion in game.explosions.iter() {
        render_explosion(buf, explosion, game.tick, &field_size);
    }
    render_score(buf, game.score, &field_size);
}
