use types::*;
use core::intrinsics;

//No idea why this doesn't work
use external;



pub fn app_event_loop() {
    unsafe {
        external::app_event_loop();
    }
}

pub fn window_create() -> *mut Window {
    unsafe {
        external::window_create()
    }
}

pub fn window_destroy(window: *mut Window) {
    unsafe {
        external::window_destroy(window);
    }
}

pub fn window_set_click_config_provider<T>(window: *mut Window, func: extern fn(*mut T)) {
    unsafe {
        external::window_set_click_config_provider(window, intrinsics::transmute(func));
    }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut Window, func: extern fn(*mut T), ctx: *mut TextLayer) {
    unsafe {
        external::window_set_click_config_provider_with_context(window,
                                                                intrinsics::transmute(func),
                                                                intrinsics::transmute(ctx));
    }
}

pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers) {
    unsafe {
        external::window_set_window_handlers(window, handlers);
    }
}

pub fn window_stack_push(window: *mut Window, animate: bool) {
    unsafe {
        if animate {
            external::window_stack_push(window, 1);
        } else {
            external::window_stack_push(window, 0);
        }
    }
}

pub fn window_get_root_layer(window: *mut Window) -> *mut Layer {
    unsafe {
        external::window_get_root_layer(window)
    }
}

pub fn window_single_click_subscribe<T>(button: u8, func: extern fn(*mut ClickRecognizer, *mut T)) {
    unsafe {
        external::window_single_click_subscribe(button, intrinsics::transmute(func));
    }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
    unsafe {
        external::layer_get_bounds(layer)
    }
}

pub fn layer_add_child(layer: *mut Layer, child: *mut Layer) {
    unsafe {
        external::layer_add_child(layer, child);
    }
}

pub fn text_layer_create(bounds: GRect) -> *mut TextLayer { 
    unsafe {
        external::text_layer_create(bounds)
    }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &str) {
    unsafe {
        external::text_layer_set_text(layer, text.as_ptr());
    }
}
pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer {
    unsafe {
        external::text_layer_get_layer(layer)
    }
}

pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap {
    unsafe {
        external::gbitmap_create_with_resource(id)
    }
}

pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer {
    unsafe {
        external::bitmap_layer_create(frame)
    }
}

pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap) {
    unsafe {
        external::bitmap_layer_set_bitmap(layer, bitmap);
    }
}

pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp) {
    unsafe {
        external::bitmap_layer_set_compositing_mode(layer, mode);
    }
}

pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer {
    unsafe {
        external::bitmap_layer_get_layer(layer)
    }
}
