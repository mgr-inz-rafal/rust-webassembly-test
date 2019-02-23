#[macro_use]
extern crate stdweb;

use stdweb::js;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const GRAVITY_DRAG: f64 = 0.2;
const BALL_RADIUS: u32 = 50;
const FALL_OFFSCREEN: u32 = SCREEN_HEIGHT + BALL_RADIUS;

struct Ball {
    pos: (f64, f64),
    radius: u32,
    acceleration: (f64, f64),
    color_str: String,
}

impl Ball {
    fn tick(&mut self) -> bool {
        self.pos.1 += self.acceleration.1;
        //        js! { console.log( "Updating ball pos to: ", @{self.pos.1} ) }

        self.acceleration.1 += GRAVITY_DRAG;

        if self.pos.1 > f64::from(FALL_OFFSCREEN) {
            true
        } else {
            false
        }
    }
}

fn get_random_u8() -> u8 {
    let value: u8 = js! { return Math.floor(Math.random() * @{std::u8::MAX}) }
        .try_into()
        .unwrap();
    u8::from(value)
}

impl Default for Ball {
    fn default() -> Ball {
        Ball {
            pos: (f64::from(SCREEN_WIDTH / 2), f64::from(FALL_OFFSCREEN)),
            radius: BALL_RADIUS,
            acceleration: (0.0, -15.0),
            color_str: format!(
                "rgb({},{},{})",
                get_random_u8(),
                get_random_u8(),
                get_random_u8()
            ),
        }
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
        self.context.set_fill_style_color(&ball.color_str);

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
    stdweb::initialize();

    let ball = Ball::default();

    fn game_loop(mut ball: Ball, view: View) {
        stdweb::web::set_timeout(
            move || {
                if ball.tick() == true {
                    js! { console.log( "Bounce!" ) }
                    let old_color = ball.color_str;
                    ball = Ball::default();
                    ball.color_str = old_color;
                }
                view.clear();
                view.paint(&ball);
                game_loop(ball, view);
            },
            16,
        );
    }

    let v = View::new(SCREEN_HEIGHT, SCREEN_WIDTH);

    game_loop(ball, v);

    stdweb::event_loop();
}
