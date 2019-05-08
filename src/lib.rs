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

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Called when the wasm module is instantiated
pub fn main() -> Result<(), JsValue> {

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

#[wasm_bindgen]
pub fn to_transparent(screen: & mut Screen, bytes: & mut [u8]) {
    for i in 0..screen.height {
        for j in 0..screen.width {
            let offset = 4 * (screen.width * i + j);

            bytes[offset] = bytes[offset];
            bytes[offset + 1] = bytes[offset + 1];
            bytes[offset + 2] = bytes[offset + 2];
            bytes[offset + 3] = 25;//透明度を上げる
        }
    }
}
