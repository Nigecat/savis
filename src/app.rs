use crate::Chronicle;
use nannou::prelude::rgb::Srgb;

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

        // Current iteration
        let n = (app.time / (self.delay as f32 / 1000.0)).floor() as usize;
        let k = match n < self.chronicle.history.len() {
            true => n,
            false => self.chronicle.history.len() - 1,
        };
        let state = &self.chronicle.history[k];

        let draw = app.draw();
        // todo
    }
}
