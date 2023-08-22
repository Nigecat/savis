use crate::utils::has_adjacent;
use crate::Chronicle;
use nannou::prelude::rgb::Srgb;
use nannou::prelude::{Point2, BLACK, GREEN, LIGHTGRAY, PURPLE, RED, WHITE};

const NODE_SIZE: f32 = 25.0;
const EDGE_WIDTH: f32 = 2.0;

pub struct App {
    chronicle: Chronicle,
    delay: f32,
}

impl App {
    pub fn new(chronicle: Chronicle, delay: f32) -> Self {
        App { chronicle, delay }
    }
}

impl nannou::Model for App {
    fn view(&self, app: &nannou::App, frame: nannou::Frame) {
        frame.clear(Srgb {
            red: 40_u8,
            green: 43_u8,
            blue: 48_u8,
            standard: ::core::marker::PhantomData,
        });

        let (width, height) = (app.window_rect().w(), app.window_rect().h());
        let x = |x: f32| x * (width / 2.0);
        let y = |y: f32| y * (height / 2.0);
        // let translate = |x: f32, y: f32| (x * (width / 2.0), y * (height / 2.0));

        // todo should warn! about missing position data

        // Current iteration
        let n = (app.time / (self.delay / 1000.0)).floor() as usize;
        let k = match n < self.chronicle.history.len() {
            true => n,
            false => self.chronicle.history.len() - 1,
        };
        let state = &self.chronicle.history[k];

        let last_good = self.chronicle.history[..=k]
            .iter()
            .rev()
            .find(|state| state.used)
            .unwrap_or(state);

        if last_good.route != state.route {
            println!("{last_good:?} {state:?}");
        }

        let draw = app.draw();

        // ---------- Draw Stats ----------
        draw.text(&format!(
            "Iteration: {}/{}",
            k + 1,
            self.chronicle.history.len()
        ))
        .x_y(x(-0.7), y(0.9))
        .no_line_wrap()
        .font_size(20)
        .color(WHITE);

        draw.text(&format!("Temperature: {}", state.temperature))
            .x_y(x(-0.7), y(0.8))
            .no_line_wrap()
            .font_size(20)
            .color(WHITE);

        draw.text(&format!("Energy: {}", state.energy))
            .x_y(x(-0.7), y(0.7))
            .font_size(20)
            .color(WHITE);

        // ---------- Draw Edges ----------
        for (u, v) in &self.chronicle.edges {
            let u_pos = self.chronicle.position(u);
            let v_pos = self.chronicle.position(v);

            let mut color = LIGHTGRAY;

            // If this edge is in the current state
            if has_adjacent(&state.route, u, v) {
                color = PURPLE; // currently searching
            }

            // If the edge is in the last good state
            if has_adjacent(&last_good.route, u, v) {
                color = GREEN; // last known good
            }

            draw.line()
                .start(Point2::new(x(u_pos.0), y(u_pos.1)))
                .end(Point2::new(x(v_pos.0), y(v_pos.1)))
                .weight(EDGE_WIDTH)
                .color(color);
        }

        // ---------- Draw Nodes ----------
        // (this is done after drawing the edges to prevent the line from showing up on top of the node)
        for node in &self.chronicle.nodes {
            let pos = self.chronicle.position(node);

            draw.ellipse()
                .color(match node == &state.route[0] {
                    true => BLACK,
                    false => match node == &last_good.route[last_good.route.len() - 1] {
                        true => RED,
                        false => match state.route.contains(node) {
                            true => GREEN,
                            false => WHITE,
                        },
                    },
                })
                .width(NODE_SIZE)
                .height(NODE_SIZE)
                .x_y(x(pos.0), y(pos.1));
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
