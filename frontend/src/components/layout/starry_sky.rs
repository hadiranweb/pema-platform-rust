use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use gloo_render::{request_animation_frame, AnimationFrame};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

struct Star {
    x: f64,
    y: f64,
    size: f64,
    speed: f64,
}

impl Star {
    fn new(canvas_width: f64, canvas_height: f64) -> Self {
        Star {
            x: js_sys::Math::random() * canvas_width,
            y: js_sys::Math::random() * canvas_height,
            size: js_sys::Math::random() * 2.0 + 0.5,
            speed: js_sys::Math::random() * 0.5 + 0.1,
        }
    }

    fn update(&mut self, canvas_height: f64) {
        self.y += self.speed;
        if self.y > canvas_height {
            self.y = 0.0;
            self.x = js_sys::Math::random() * 800.0; // Assuming canvas width is 800 for simplicity
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.size / 2.0, 0.0, js_sys::Math::PI() * 2.0).unwrap();
        ctx.set_fill_style(&"white".into());
        ctx.fill();
    }
}

#[function_component(StarrySky)]
pub fn starry_sky() -> Html {
    let canvas_ref = use_node_ref();
    let animation_frame = use_ref(|| None);

    use_effect_with_deps(move |canvas_ref| {
        let canvas = canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("canvas_ref should be attached to a canvas element");

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();

        let num_stars = 100;
        let mut stars: Vec<Star> = (0..num_stars)
            .map(|_| Star::new(canvas.width() as f64, canvas.height() as f64))
            .collect();

        let stars_rc = Rc::new(RefCell::new(stars));
        let ctx_rc = Rc::new(ctx);
        let canvas_rc = Rc::new(canvas);

        let render_frame = { 
            let stars_rc = stars_rc.clone();
            let ctx_rc = ctx_rc.clone();
            let canvas_rc = canvas_rc.clone();

            let mut f: Option<AnimationFrame> = None;
            let f_rc = Rc::new(RefCell::new(f));
            let g = f_rc.clone();

            let closure = Closure::wrap(Box::new(move || {
                let canvas = canvas_rc.borrow();
                let ctx = ctx_rc.borrow();
                let mut stars = stars_rc.borrow_mut();

                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                ctx.set_fill_style(&"black".into());
                ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                for star in stars.iter_mut() {
                    star.update(canvas.height() as f64);
                    star.draw(&ctx);
                }

                *g.borrow_mut() = Some(request_animation_frame(g.borrow().as_ref().unwrap()));
            }) as Box<dyn FnMut()>);

            *f_rc.borrow_mut() = Some(request_animation_frame(closure.as_ref().unchecked_ref()));
            closure
        };

        animation_frame.set(Some(render_frame));

        || {
            // Cleanup: stop animation frame when component unmounts
            if let Some(frame) = animation_frame.borrow_mut().take() {
                drop(frame);
            }
        }
    }, canvas_ref.clone());

    html! {
        <canvas ref={canvas_ref} id="starry-sky" width="800" height="600"></canvas>
    }
}

