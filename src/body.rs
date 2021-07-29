use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{color::Color, log, vector2::Vector2, HEIGHT, WIDTH};

#[derive(Copy, Clone)]
pub struct BodyState {
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub color: Color,
}

#[derive(Copy, Clone)]

pub struct Body {
    state: BodyState,
}

impl PartialEq for Body {
    fn ne(&self, other: &Self) -> bool {
        other.state.pos.x != self.state.pos.x
            && other.state.vel.x != self.state.vel.x
            && other.state.acc.x != self.state.acc.x
    }

    fn eq(&self, other: &Self) -> bool {
        other.state.pos.x == self.state.pos.x
            && other.state.vel.x == self.state.vel.x
            && other.state.acc.x == self.state.acc.x
    }
}

impl Body {
    pub fn new(pos: Vector2) -> Self {
        Self {
            state: BodyState {
                pos,
                vel: Vector2 { x: 0.0, y: 0.0 },
                acc: Vector2 { x: 0.0, y: 0.0 },
                color: Color::new(0, 0, 0),
            },
        }
    }

    pub fn edges(&mut self) {
        let mut strikesx = false;
        let mut strikesy = false;

        if self.state.pos.x < 0.0 {
            self.state.vel.x *= -1.0;
            self.state.pos.x = 0.0;
            strikesx = true;
        }

        if self.state.pos.x > WIDTH - 10.0 {
            self.state.vel.x *= -1.0;
            self.state.pos.x = WIDTH - 10.0;
            strikesx = true;
        }

        if self.state.pos.y < 0.0 {
            self.state.vel.y *= -1.0;
            self.state.pos.y = 0.0;
            strikesy = true;
        }

        if self.state.pos.y > HEIGHT - 10.0 {
            self.state.vel.y *= -1.0;
            self.state.pos.y = HEIGHT - 10.0;
            strikesy = true;
        }

        if strikesx {
            self.state.vel.x *= 0.9;
        }

        if strikesy {
            self.state.vel.y *= 0.9;
        }
    }

    pub fn update(&mut self) {
        self.edges();
        self.state.acc.add(Vector2 { x: 0.0, y: 0.1 });

        self.state.pos.add(self.state.vel);
        self.state.vel.add(self.state.acc);
        self.state.acc.mult_f64(0.0);
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.state.acc.add(force);
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let color = &format!("#{}", self.state.color.get_hex());

        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.fill_rect(self.state.pos.x, self.state.pos.y, 10.0, 10.0);
    }

    pub fn get_state(&self) -> BodyState {
        self.state
    }

    pub fn get_state_mut(&mut self) -> &mut BodyState {
        &mut self.state
    }

    pub fn spring_force(&mut self, other: &Body) -> Vector2 {
        let rest_length = 2.0;
        let dx = self.state.pos.x - other.state.pos.x;
        let dy = self.state.pos.y - other.state.pos.y;

        let elasticity = -0.001;

        Vector2 {
            x: (dx - rest_length) * elasticity,
            y: (dy - rest_length) * elasticity,
        }
    }
}
