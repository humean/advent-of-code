#[macro_use]
extern crate cfg_if;
use advent_of_code;
extern crate wasm_bindgen;
use serde_derive::Serialize;
use web_sys;

use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[derive(Serialize)]
pub struct ExportedSolution {
    pub part_one: String,
    pub part_two: String,
}

impl From<advent_of_code::Solution> for ExportedSolution {
    fn from(s: advent_of_code::Solution) -> Self {
        Self {
            part_one: s.part_one,
            part_two: s.part_two,
        }
    }
}

#[wasm_bindgen]
pub fn solve_day(year: u16, day: u8, input: String) -> Result<JsValue, JsValue> {
    let config = advent_of_code::Config::new(year, day, input);

    let solution =
        ExportedSolution::from(advent_of_code::solve_day(&config).map_err(|err| err.to_string())?);

    Ok(JsValue::from_serde(&solution).unwrap())
}
