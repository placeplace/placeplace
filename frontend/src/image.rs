use std::{collections::HashMap, io::Cursor};

use js_sys::Uint8Array;
use png::ColorType;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCache, RequestInit, Response};

pub async fn load_image_cells(
    url: &str,
    lookup: &HashMap<[u8; 3], usize>,
) -> (usize, usize, Vec<u8>) {
    let window = web_sys::window().unwrap();

    // Fetch the image
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.cache(RequestCache::Reload);
    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let response: Response = response_value.dyn_into().unwrap();

    let buffer_value = JsFuture::from(response.array_buffer().unwrap())
        .await
        .unwrap();
    let array = Uint8Array::new(&buffer_value);

    let mut image_data = vec![0; array.length() as usize];
    array.copy_to(&mut image_data[..]);

    // Load the image
    let decoder = png::Decoder::new(Cursor::new(image_data));
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    let log_text = format!("Decoding PNG, color type: {:?}", info.color_type);
    web_sys::console::log_1(&log_text.into());

    assert_eq!(info.color_type, ColorType::Rgba);

    // Decode the image
    let mut cells = vec![0u8; (info.width * info.height) as usize];
    let (width, height) = (info.width as usize, info.height as usize);
    for (i, cell) in cells.iter_mut().enumerate() {
        let x = i as u32 % width as u32;
        let y = i as u32 / width as u32;

        let start_byte = i * 4;
        let pixel = &bytes[start_byte..start_byte + 4];
        let color = [pixel[0], pixel[1], pixel[2]];

        if let Some(color_index) = lookup.get(&color) {
            *cell = *color_index as u8;
        } else {
            let log_text = format!("Invalid pixel: ({}, {}) -> {:?}", x, y, color);
            web_sys::console::log_1(&log_text.into());

            *cell = 0;
        }
    }

    (width, height, cells)
}
