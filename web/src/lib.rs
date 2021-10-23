use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use web_sys::console;

use serde_json::to_string;
use slicing::geometry::Plane;
use slicing::threemf::model::Model;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(s: usize);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn load_model(data: &[u8]) -> Result<String, JsValue> {
    let model = Model::from_raw_data(data).unwrap();

    let json_verts = serde_json::to_string(&model.objects[0].mesh.vertices).unwrap();
    let json_tris = serde_json::to_string(&model.objects[0].mesh.triangles).unwrap();
    log(format!("{},{}", json_verts, json_tris).as_str());

    Ok(format!(
        r#"{{"name": "{}", "verts": {}, "tris": {}}}"#,
        model.name(),
        json_verts,
        json_tris
    ))
}

#[wasm_bindgen]
pub fn slice_model(data: &[u8]) -> Result<String, JsValue> {
    log_usize(data.len());

    let model = Model::from_raw_data(data).unwrap();

    log(model.name());
    log(format!("\tObjects: {}", model.num_objects()).as_str());
    log(format!("\tTris: {}", model.num_triangles()).as_str());
    log(format!("\tVerts: {}", model.num_vertices()).as_str());

    log("\nSlicing Results");
    let cuts = slicing::slice_model(model).unwrap();
    log(format!("Number of Cuts: {}", cuts.len()).as_str());
    for cut in cuts.iter() {
        log(format!("{}", cut.normal).as_str());
    }

    let json = serde_json::to_string(&cuts).unwrap();
    log(json.as_str());

    Ok(json)
}

#[wasm_bindgen]
pub fn get_centroid(data: &[u8]) -> Result<String, JsValue> {
    let model = Model::from_raw_data(data).unwrap();

    let centroid = model.objects[0].mesh.centroid();
    let json = serde_json::to_string(&centroid).unwrap();

    log(format!("Centroid: {}", json.as_str()).as_str());

    Ok(json)
}
