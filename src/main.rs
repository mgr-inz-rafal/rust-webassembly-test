extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;

pub struct View {
    canvas: CanvasElement,
    context: CanvasRenderingContext2d,
    dimension: (u32, u32),
}

impl View {
    pub fn new(width: u32, height: u32) -> View {
        let canvas: CanvasElement = document()
            .query_selector("#can")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        View {
            canvas,
            context,
            dimension: (width, height),
        }
    }

    pub fn paint(&self) {
        self.context.set_fill_style_color("blue");
        self.context.fill_rect(100.0, 100.0, 200.0, 200.0);
    }

    fn clear(&self) {
        self.context.set_fill_style_color("white");
        self.context.fill_rect(
            0.0,
            0.0,
            f64::from(self.dimension.0),
            f64::from(self.dimension.1),
        );
    }
}

fn main() {
    stdweb::initialize();

    let v = View::new(800, 600);
    v.clear();
    v.paint();

    stdweb::event_loop();
}
