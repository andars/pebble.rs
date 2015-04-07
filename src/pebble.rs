use types::*;

pub fn app_event_loop() {
    unsafe {
        ::external::app_event_loop();
    }
}

pub fn window_create() -> *mut Window {
    unsafe {
        ::external::window_create()
    }
}

pub fn window_destroy(window: *mut Window) {
    unsafe {
        ::external::window_destroy(window);
    }
}

pub fn window_set_click_config_provider<T>(window: *mut Window, func: extern fn(*mut T)) {
    unsafe {
        ::external::window_set_click_config_provider(window, ::core::intrinsics::transmute(func));
    }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut Window, func: extern fn(*mut T), ctx: *mut TextLayer) {
    unsafe {
        ::external::window_set_click_config_provider_with_context(window,
                                                                  ::core::intrinsics::transmute(func),
                                                                  ::core::intrinsics::transmute(ctx));
    }
}

pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers) {
    unsafe {
        ::external::window_set_window_handlers(window, handlers);
    }
}

pub fn window_stack_push(window: *mut Window, animate: u8) {
    unsafe {
        ::external::window_stack_push(window, animate);
    }
}

pub fn window_get_root_layer(window: *mut Window) -> *mut Layer {
    unsafe {
        ::external::window_get_root_layer(window)
    }
}

pub fn window_single_click_subscribe<T>(button: u8, func: extern fn(*mut ClickRecognizer, *mut T)) {
    unsafe {
        ::external::window_single_click_subscribe(button, ::core::intrinsics::transmute(func));
    }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
    unsafe {
        ::external::layer_get_bounds(layer)
    }
}

pub fn layer_add_child(layer: *mut Layer, child: *mut Layer) {
    unsafe {
        ::external::layer_add_child(layer, child);
    }
}

pub fn text_layer_create(bounds: GRect) -> *mut TextLayer { 
    unsafe {
        ::external::text_layer_create(bounds)
    }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &str) {
    unsafe {
        ::external::text_layer_set_text(layer, text);
    }
}
pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer {
    unsafe {
        ::external::text_layer_get_layer(layer)
    }
}

