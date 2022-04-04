#![allow(clippy::unused_unit)]

mod image;
mod palette;

use std::collections::HashMap;

use once_cell::sync::OnceCell;
use palette::PaletteColor;
use rand::{prelude::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, MouseEvent, Request, RequestCache,
    RequestInit, Response,
};

use crate::{image::load_image_cells, palette::create_palette};

const PIXEL_SIZE: i32 = 5;

#[wasm_bindgen(start)]
pub async fn main() {
    let window = web_sys::window().unwrap();

    // Fetch configuration
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.cache(RequestCache::Reload);
    let request = Request::new_with_str_and_init("/placeplace.json", &opts).unwrap();

    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let response: Response = response_value.dyn_into().unwrap();

    let json = JsFuture::from(response.json().unwrap()).await.unwrap();
    let configuration: Configuration = json.into_serde().unwrap();

    let info = format!("{:?}", configuration);
    web_sys::console::log_1(&info.into());

    // Store the configuration globally
    let colors = create_palette();

    let mut lookup = HashMap::new();
    for (i, color) in colors.iter().enumerate() {
        lookup.insert(color.color, i);
    }

    let (width, height, cells) = load_image_cells("/placeplace.png", &lookup).await;
    let (_, _, mut current_cells) = load_image_cells("/placeplace_current.png", &lookup).await;

    if cells.len() != current_cells.len() {
        // Will never match, to fall back to random
        current_cells = vec![0xF0; cells.len()];
    }

    let data = GlobalData {
        config: configuration,
        colors,
        width,
        height,
        cells,
        current_cells,
    };
    GLOBAL.set(data).unwrap();

    // Different behavior depending on if we're set to ready
    if GLOBAL.get().unwrap().config.ready {
        init_ready();
    } else {
        // Display not-ready
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let label = document.get_element_by_id("pp-label-assigned").unwrap();
        label.set_inner_html("Nothing yet! Follow the RubberRoss Twitch stream for instructions.");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    ready: bool,
    offset_x: usize,
    offset_y: usize,
    dither: DitherMode,
}

#[derive(Serialize, Deserialize, Debug)]
enum DitherMode {
    None,
    Even,
    Odd,
}

fn init_ready() {
    let g = GLOBAL.get().unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Add button click callback
    let button = document
        .get_element_by_id("pp-button-new")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let click_handler = Closure::wrap(Box::new(move || {
        pick_new_pixel();
    }) as Box<dyn FnMut()>);
    button.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();

    // Add canvas click callback
    let canvas = document
        .get_element_by_id("pp-canvas")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let click_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        pick_canvas_pixel(event);
    }) as Box<dyn FnMut(_)>);
    canvas.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();

    // Initialize overview data
    let topleft_text = format!("{}, {}", g.config.offset_x, g.config.offset_y);
    let label = document.get_element_by_id("pp-label-topleft").unwrap();
    label.set_inner_html(&topleft_text);

    // Pick pixel automatically
    pick_new_pixel();
}

static GLOBAL: OnceCell<GlobalData> = OnceCell::new();

#[derive(Debug)]
struct GlobalData {
    config: Configuration,
    colors: Vec<PaletteColor>,
    width: usize,
    height: usize,
    cells: Vec<u8>,
    current_cells: Vec<u8>,
}

fn pick_new_pixel() {
    let g = GLOBAL.get().unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        // Pick the random pixel
        let index = next_index(g, &mut rng);

        // If the pixel doesn't match, target that
        if g.current_cells[index] != g.cells[index] {
            set_active_pixel(index);
            return;
        }
    }

    // Fall back to just any random pixel
    let index = next_index(g, &mut rng);
    set_active_pixel(index);
}

fn next_index(g: &GlobalData, rng: &mut ThreadRng) -> usize {
    match g.config.dither {
        DitherMode::None => rng.gen_range(1..(g.cells.len())),
        DitherMode::Even => rng.gen_range(1..(g.cells.len() / 2)) * 2,
        DitherMode::Odd => rng.gen_range(1..(g.cells.len() / 2)) * 2 + 1,
    }
}

fn pick_canvas_pixel(event: MouseEvent) {
    let g = GLOBAL.get().unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Get offset of the image in the canvas
    let canvas = document
        .get_element_by_id("pp-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let (offset_x, offset_y) = get_offset(canvas.width(), canvas.height());

    // Calculate pixel coordinates
    let pixel_x = (event.offset_x() - offset_x) / PIXEL_SIZE;
    let pixel_y = (event.offset_y() - offset_y) / PIXEL_SIZE;

    if pixel_x < 0 || pixel_y < 0 || pixel_x >= g.width as i32 || pixel_y >= g.height as i32 {
        return;
    }

    set_active_pixel(pixel_x as usize + (pixel_y as usize * g.width));
}

fn set_active_pixel(index: usize) {
    let g = GLOBAL.get().unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Resolve pixels
    let relative_x = index % g.width;
    let relative_y = index / g.width;
    let x = relative_x + g.config.offset_x;
    let y = relative_y + g.config.offset_y;

    let color = &g.colors[g.cells[index] as usize];
    let text = format!("Your pixel is <span class=\"font-semibold\">{}</span> at <span class=\"font-semibold\">{}</span>, <span class=\"font-semibold\">{}</span>!", color.name, x, y);
    let direct_link = format!("https://www.reddit.com/r/place/?cx={}&cy={}&px=11", x, y);
    let link_html = format!("<a target=\"_blank\" rel=\"noopener noreferrer\" href=\"{}\">Direct link to r/place location...</a>", direct_link);

    let color_str = color_to_rgb(color.color);

    // Initialize the page with the picked color
    let label_asigned = document.get_element_by_id("pp-label-assigned").unwrap();
    label_asigned.set_inner_html(&text);
    let label_link = document.get_element_by_id("pp-label-directlink").unwrap();
    label_link.set_inner_html(&link_html);

    let color_box = document
        .get_element_by_id("pp-color")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    color_box
        .style()
        .set_property("background-color", &color_str)
        .unwrap();

    // Redraw the canvas with the picked pixel
    redraw_canvas(relative_x as i32, relative_y as i32);
}

fn redraw_canvas(pixel_x: i32, pixel_y: i32) {
    let g = GLOBAL.get().unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Fetch canvas and context from the DOM
    let canvas = document
        .get_element_by_id("pp-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Get relative coordinates for centering the image
    let canvas_width = canvas.width() as u32;
    let canvas_height = canvas.height() as u32;
    let (offset_x, offset_y) = get_offset(canvas_width, canvas_height);

    // Clear the canvas before drawing the new content
    context.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

    // Draw the image centered on the screen
    for (i, cell) in g.cells.iter().enumerate() {
        let x = (i % g.width) as i32;
        let y = (i / g.width) as i32;

        let fill = color_to_rgb(g.colors[*cell as usize].color);
        context.set_fill_style(&fill.into());
        context.fill_rect(
            (offset_x + (x * PIXEL_SIZE)) as f64,
            (offset_y + (y * PIXEL_SIZE)) as f64,
            PIXEL_SIZE as f64,
            PIXEL_SIZE as f64,
        );
    }

    // Outline the picked pixel
    let pixel_canvax_x = offset_x + (pixel_x * PIXEL_SIZE);
    let pixel_canvax_y = offset_y + (pixel_y * PIXEL_SIZE);

    context.set_fill_style(&"rgba(255, 0, 0, 0.8)".into());

    // top
    context.fill_rect(
        (pixel_canvax_x - 2) as f64,
        (pixel_canvax_y - 2) as f64,
        (PIXEL_SIZE + 4) as f64,
        2.0,
    );
    // bottom
    context.fill_rect(
        (pixel_canvax_x - 2) as f64,
        (pixel_canvax_y + PIXEL_SIZE) as f64,
        (PIXEL_SIZE + 4) as f64,
        2.0,
    );
    // left
    context.fill_rect(
        (pixel_canvax_x - 2) as f64,
        (pixel_canvax_y) as f64,
        2.0,
        (PIXEL_SIZE) as f64,
    );
    // right
    context.fill_rect(
        (pixel_canvax_x + PIXEL_SIZE) as f64,
        (pixel_canvax_y) as f64,
        2.0,
        (PIXEL_SIZE) as f64,
    );
}

fn color_to_rgb(color: [u8; 3]) -> String {
    format!("rgb({}, {}, {})", color[0], color[1], color[2])
}

fn get_offset(canvas_width: u32, canvas_height: u32) -> (i32, i32) {
    let g = GLOBAL.get().unwrap();
    let image_width = g.width as i32 * PIXEL_SIZE;
    let image_height = g.height as i32 * PIXEL_SIZE;
    let offset_x = (canvas_width as i32 - image_width) / 2;
    let offset_y = (canvas_height as i32 - image_height) / 2;

    (offset_x, offset_y)
}
