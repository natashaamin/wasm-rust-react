use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
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

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello {}", name));
}

#[wasm_bindgen]
pub fn take_string_by_value(x: String) -> String {
    x
}