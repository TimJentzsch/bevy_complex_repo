#![no_std]
#![no_main]

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn suppress_panics(_: &core::panic::PanicInfo<'_>) -> ! {
    alert("Oh No!");
    loop {}
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub extern "C" fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, || {
            alert("Starting Up...");
        })
        .run();
}
