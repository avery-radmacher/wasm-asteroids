use crate::game::{Asteroid, Bullet, Explosion, Game, InputIndex};
use crate::math::Vec2D;
use std::fmt::Write;

fn draw(buf: &mut String, line: bool, point: Vec2D) {
    write!(
        buf,
        "{}{:.2} {:.2} ",
        if line { 'L' } else { 'M' },
        point.x,
        point.y
    )
    .expect("could not write string");
}

fn draw_points(buf: &mut String, points: Vec<Vec2D>) {
    if (points.is_empty()) {
        return;
    }
    draw(buf, false, points[0]);
    for &point in &points[1..] {
        draw(buf, true, point);
    }
}

fn draw_object(buf: &mut String, points: Vec<Vec2D>, scale: f64, rotation: f64, offset: Vec2D) {
    draw_points(
        buf,
        points
            .iter()
            .map(|p| p.scale(scale).rotate(rotation) + offset)
            .collect(),
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
    draw_object(buf, SHIP_POINTS.to_vec(), 2.0, ship.angle, ship.pos);
    let inputs = &game.inputs;
    if inputs.is_down(InputIndex::Forward) || inputs.is_down(InputIndex::Backward) {
        draw_object(buf, FLARE.to_vec(), 2.0, ship.angle, ship.pos);
    }
}

fn render_bullet(buf: &mut String, bullet: &Bullet) {
    let tail = bullet.pos + bullet.speed.normalize().scale(5.0);
    draw_points(buf, vec![bullet.pos, tail]);
}

fn render_asteroid(buf: &mut String, asteroid: &Asteroid) {
    let cnt = asteroid.style;
    let angle = std::f64::consts::TAU / (cnt as f64);
    let asteroid_points = vec![Vec2D::one(); cnt + 1]
        .iter()
        .enumerate()
        .map(|(i, v)| v.rotate(angle * (i as f64)))
        .collect();
    draw_object(
        buf,
        asteroid_points,
        asteroid.size,
        asteroid.angle,
        asteroid.pos,
    );
}

fn render_lives(buf: &mut String, lives: u64) {
    const LIFE_STEP: f64 = 40.0;
    const UP_ANGLE: f64 = std::f64::consts::PI * -0.5;
    for l in 0..lives {
        let y = -50.0;
        let x = ((l + 1) as f64) * LIFE_STEP;
        for (i, p) in SHIP_POINTS.iter().enumerate() {
            let p_c = p.scale(2.0).rotate(UP_ANGLE) + Vec2D { x, y };
            draw(buf, i != 0, p_c);
        }
    }
}

fn render_explosion(buf: &mut String, explosion: &Explosion, tick: u64) {
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
        draw_points(buf, vec![start, end]);
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

fn render_score(buf: &mut String, mut score: u64) {
    let mut digits = Vec::new();
    while score > 0 {
        digits.push(score % 10);
        score /= 10;
    }
    if digits.is_empty() {
        digits.push(0);
    }
    const DIGIT_SCALE: f64 = 10.0;
    const DIGIT_STEP: f64 = -30.0;
    const DIGIT_RIGHTMOST: f64 = 1200.0;
    for (idx, d) in digits.iter().enumerate() {
        let digit = VECTOR_DIGITS[*d as usize];
        for (i, p) in digit.iter().enumerate() {
            let p = Vec2D { x: p.x, y: p.y }.scale(DIGIT_SCALE)
                + Vec2D {
                    x: DIGIT_RIGHTMOST + (idx as f64) * DIGIT_STEP,
                    y: -DIGIT_SCALE * 5.0,
                };
            draw(buf, i != 0, p);
        }
    }
}

pub fn render_game(buf: &mut String, game: &Game) {
    render_lives(buf, game.lives);
    render_ship(buf, game);
    for bullet in game.bullets.iter() {
        render_bullet(buf, bullet);
    }
    for asteroid in game.asteroids.iter() {
        render_asteroid(buf, asteroid);
    }
    for explosion in game.explosions.iter() {
        render_explosion(buf, explosion, game.tick);
    }
    render_score(buf, game.score);
}
