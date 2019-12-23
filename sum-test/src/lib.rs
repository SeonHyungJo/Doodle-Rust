extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    using_web_sys();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = time)]
    fn log_time(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = timeEnd)]
    fn log_timeEnd(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

fn using_web_sys() {
    log_time("time");

    let max = 1000000; // 1경
    let mut check = 0;
    let mut done = false;
    
    while !done {
        check = check + 1;

        if check == max {
            done = true;
        }
    }

    log_u32(check);
    log_timeEnd("time");
}