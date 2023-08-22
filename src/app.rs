use crate::utils::has_adjacent;
use crate::Chronicle;
use nannou::prelude::rgb::Srgb;
use nannou::prelude::{Point2, GREEN, RED, WHITE};

const NODE_SIZE: f32 = 25.0;
const EDGE_WIDTH: f32 = 2.0;

pub struct App {
    chronicle: Chronicle,
    delay: usize,
}

impl App {
    pub fn new(chronicle: Chronicle, delay: usize) -> Self {
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
        let n = (app.time / (self.delay as f32 / 1000.0)).floor() as usize;
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

        let draw = app.draw();

        // ---------- Draw Edges ----------
        for (u, v) in &self.chronicle.edges {
            let u_pos = self.chronicle.position(u);
            let v_pos = self.chronicle.position(v);

            let mut color = WHITE;

            // If this edge is in the current state
            if has_adjacent(&state.route, u, v) {
                color = RED; // currently searching
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
                .color(WHITE)
                .width(NODE_SIZE)
                .height(NODE_SIZE)
                .x_y(x(pos.0), y(pos.1));
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
