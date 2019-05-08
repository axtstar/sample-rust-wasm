mod utils;

extern crate image;
//extern crate rand;

use js_sys::*;
use rand::prelude::*;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

//use image::{RgbImage};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

///////////////////////////////////////////////////////////////////↓
// js's `alert`
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// calling js's `alert`
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
///////////////////////////////////////////////////////////////////↑


///////////////////////////////////////////////////////////////////↓
// startup
fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}

// And finally, we don't even have to define the `log` function ourselves! The
// `web_sys` crate already has it defined for us.

fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}
///////////////////////////////////////////////////////////////////↑


///////////////////////////////////////////////////////////////////↓
// Called when the wasm module is instantiated
pub fn main() -> Result<(), JsValue> {
    //run();
    //draw();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

// add automatically the below function
#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
///////////////////////////////////////////////////////////////////↑

#[wasm_bindgen]
pub fn hello_p() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-2) + fib(n-1),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("start");
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    console_log!("canvas width={}", canvas.width());
    console_log!("canvas height={}", canvas.height());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context
        .fill_rect(0.0, 0.0, 10.0, 10.0);
        //.unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
    console_log!("end");
}

#[wasm_bindgen]
pub struct Screen {
    bytes: Vec<u8>,
    #[wasm_bindgen(readonly)]
    pub width: usize,
    #[wasm_bindgen(readonly)]
    pub height: usize,
}

fn create_buffer(width: usize, height: usize) -> Vec<u8> {
    let size = 4 * width * height;
    let mut bytes = Vec::with_capacity(size);
    bytes.resize(size, 0);
    bytes
}


#[wasm_bindgen]
impl Screen {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            bytes: create_buffer(width, height),
            width,
            height,
        }
    }

    pub fn pointer(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.bytes = create_buffer(width, height);
        self.width = width;
        self.height = height;
    }
}

fn repeat(a: f64, b: f64, limit: usize) -> usize {
    let mut x0 = 0.;
    let mut y0 = 0.;
    for k in 0..limit {
        let x = x0 * x0 - y0 * y0 + a;
        let y = 2. * x0 * y0 + b;
        if x * x + y * y >= 4. {
            return k;
        }
        x0 = x;
        y0 = y;
    }
    return limit;
}

#[wasm_bindgen]
pub fn mandelbrot(screen: &mut Screen, x0: f64, y0: f64, d: f64, limit: usize) {
    let mut y = y0;
    for i in 0..screen.height {
        let mut x = x0;
        for j in 0..screen.width {
            let k = repeat(x, y, limit);
            let v = (k * 255 / limit) as u8;
            let offset = 4 * (screen.width * i + j);
            screen.bytes[offset] = v;
            screen.bytes[offset + 1] = v;
            screen.bytes[offset + 2] = v;
            screen.bytes[offset + 3] = 255;
            x += d;
        }
        y += d;
    }
}

fn getRandom(x:usize, a:usize,b:usize,c:usize) -> u8 {
    let y = a * x^3 + b * x^2 + c * x + 15;
    y as u8
}

#[wasm_bindgen]
pub fn to_transparent(screen: & mut Screen, bytes: & mut [u8]) {
    let mut color = 0;

    for i in 0..screen.height {
        for j in 0..screen.width {
            let offset = 4 * (screen.width * i + j);

            bytes[offset] = bytes[offset];
            bytes[offset + 1] = bytes[offset + 1];
            bytes[offset + 2] = bytes[offset + 2];
            bytes[offset + 3] = 25;//透明度を上げる

            color += 1;
        }
    }
}
