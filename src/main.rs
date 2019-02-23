#[macro_use]
extern crate stdweb;

use stdweb::js;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;

struct Ball {
    pos: (f64, f64),
    radius: u32,
    acceleration: (f64, f64),
}

impl Ball {
    fn tick(&mut self) {
        self.pos.1 += self.acceleration.1;
        js! { console.log( "Updating ball pos to: ", @{self.pos.1} ) }
    }
}

struct View {
    context: CanvasRenderingContext2d,
    dimension: (u32, u32),
}

impl View {
    fn new(width: u32, height: u32) -> View {
        let canvas: CanvasElement = document()
            .query_selector("#can")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        View {
            context,
            dimension: (width, height),
        }
    }

    fn paint(&self, ball: &Ball) {
        self.context.set_fill_style_color("rgb(0,255,0)");

        self.context.begin_path();
        self.context.arc(
            ball.pos.0,
            ball.pos.1,
            f64::from(ball.radius),
            0.0,
            f64::from(2.0 * 3.14),
            false,
        );
        self.context.fill(stdweb::web::FillRule::default());
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
    const W: u32 = 800;
    const H: u32 = 600;

    stdweb::initialize();

    let ball = Ball {
        pos: (f64::from(W / 2), f64::from(H)),
        radius: 50,
        acceleration: (0.0, -1.0),
    };

    fn game_loop(mut ball: Ball, view: View) {
        stdweb::web::set_timeout(
            move || {
                ball.tick();
                view.clear();
                view.paint(&ball);
                game_loop(ball, view);
            },
            16,
        );
    }

    let v = View::new(H, W);

    game_loop(ball, v);

    stdweb::event_loop();
}
