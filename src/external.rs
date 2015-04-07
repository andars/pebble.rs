use types::*;


extern {
    pub fn app_event_loop();
    pub fn window_create() -> *mut Window;
    pub fn window_destroy(window: *mut Window);

    pub fn window_set_click_config_provider(window: *mut Window, func: extern fn(*mut c_void));
    pub fn window_set_click_config_provider_with_context(window: *mut Window, func: extern fn(*mut u8), ctx: *mut u8);
    pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);

    pub fn window_single_click_subscribe(button: u8, func: extern fn(*mut ClickRecognizer, *mut u8));

    pub fn window_stack_push(window: *mut Window, animate: u8);

    pub fn window_get_root_layer(window: *mut Window) -> *mut Layer;

    pub fn layer_get_bounds(layer: *mut Layer) -> GRect;
    pub fn layer_add_child(layer: *mut Layer, child: *mut Layer);

    pub fn text_layer_create(bounds: GRect) -> *mut TextLayer;
    pub fn text_layer_set_text(layer: *mut TextLayer, text: &str);
    pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer;
}
