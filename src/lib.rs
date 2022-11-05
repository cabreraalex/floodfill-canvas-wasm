use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

fn same_color(curr_pixel: [u8; 4], color: [u8; 4]) -> bool {
    if curr_pixel[0] == color[0] && curr_pixel[1] == color[1] && curr_pixel[2] == color[2] {
        return true;
    } else {
        return false;
    }
}

fn set_color(img_map: &mut Vec<Vec<[u8; 4]>>, curr_idx: [usize; 2], color: [u8; 4]) {
    let mut curr_pixel = img_map[curr_idx[0]][curr_idx[1]];
    curr_pixel[0] = color[0];
    curr_pixel[1] = color[1];
    curr_pixel[2] = color[2];
    curr_pixel[3] = 255;
    img_map[curr_idx[0]][curr_idx[1]] = curr_pixel;
}

#[wasm_bindgen]
pub fn floodfill(
    arr_js: js_sys::Uint8ClampedArray,
    start_js: js_sys::Array,
    width_js: js_sys::Number,
    height_js: js_sys::Number,
    color_js: js_sys::Array,
) -> js_sys::Uint8ClampedArray {
    console_error_panic_hook::set_once();

    let width: usize = width_js.as_f64().unwrap() as usize;
    let height: usize = height_js.as_f64().unwrap() as usize;
    let start: [usize; 2] = [
        start_js.get(0).as_f64().unwrap_or(0.0) as usize,
        start_js.get(1).as_f64().unwrap_or(0.0) as usize,
    ];
    let color: [u8; 4] = [
        color_js.get(0).as_f64().unwrap_or(0.0) as u8,
        color_js.get(1).as_f64().unwrap_or(0.0) as u8,
        color_js.get(2).as_f64().unwrap_or(0.0) as u8,
        255,
    ];

    let mut arr: Vec<u8> = vec![0; width * height * 4];
    arr_js.copy_to(&mut arr);

    let mut img_map: Vec<Vec<[u8; 4]>> = vec![vec![[0, 0, 0, 0]; height]; width];

    for r in 0..width {
        for c in 0..height {
            let idx = (r + (c * width)) * 4;
            img_map[r][c] = [arr[idx], arr[idx + 1], arr[idx + 2], arr[idx + 3]];
        }
    }

    let mut pixel_stack: Vec<[usize; 2]> = vec![start];
    let original_color = img_map[start[0]][start[1]].clone();
    while pixel_stack.len() != 0 {
        let curr_idx = pixel_stack.pop().unwrap();
        let curr_pixel = img_map[curr_idx[0] as usize][curr_idx[1] as usize];
        if same_color(curr_pixel, original_color) {
            set_color(&mut img_map, curr_idx, color);
            if curr_idx[0] != 0 {
                pixel_stack.push([curr_idx[0] - 1, curr_idx[1]]);
            }
            if curr_idx[1] != 0 {
                pixel_stack.push([curr_idx[0], curr_idx[1] - 1]);
            }
            if curr_idx[0] + 1 != width {
                pixel_stack.push([curr_idx[0] + 1, curr_idx[1]]);
            }
            if curr_idx[1] + 1 != height {
                pixel_stack.push([curr_idx[0], curr_idx[1] + 1]);
            }
        }
    }

    for r in 0..width {
        for c in 0..height {
            let idx = (r + (c * width)) * 4;
            let ent = img_map[r][c];
            arr[idx] = ent[0];
            arr[idx + 1] = ent[1];
            arr[idx + 2] = ent[2];
            arr[idx + 3] = ent[3];
        }
    }

    let return_arr = Uint8ClampedArray::new_with_length((width * height * 4) as u32);
    return_arr.copy_from(&arr[..]);
    return return_arr;
}
