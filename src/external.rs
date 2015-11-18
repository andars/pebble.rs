use types::*;


extern {
    pub fn app_event_loop();
    pub fn window_create() -> *mut Window;
    pub fn window_destroy(window: *mut Window);

    pub fn window_set_click_config_provider(window: *mut Window, func: extern fn(*mut c_void));
    pub fn window_set_click_config_provider_with_context(window: *mut Window, func: extern fn(*mut u8), ctx: *mut u8);
    pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);
    pub fn window_set_background_color(window: *mut Window, color: GColor);
    pub fn window_set_user_data(window: *mut Window, data: *mut c_void);
    pub fn window_get_user_data(window: *mut Window) -> *mut c_void;

    pub fn window_stack_push(window: *mut Window, animate: u8);
    pub fn window_get_root_layer(window: *mut Window) -> *mut Layer;
    pub fn window_single_click_subscribe(button: u8, func: extern fn(*mut ClickRecognizer, *mut u8));

    pub fn layer_create(bounds: GRect) -> *mut Layer;
    pub fn layer_destroy(layer: *mut Layer);
    pub fn layer_get_frame(layer: *mut Layer) -> GRect;
    pub fn layer_get_bounds(layer: *mut Layer) -> GRect;
    pub fn layer_add_child(layer: *mut Layer, child: *mut Layer);
    pub fn layer_mark_dirty(layer: *mut Layer);
    pub fn layer_set_update_proc(layer: *mut Layer, func: extern fn(*mut Layer, *mut GContext));

    pub fn text_layer_create(bounds: GRect) -> *mut TextLayer;
    pub fn text_layer_set_text(layer: *mut TextLayer, text: *const c_char);
    pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer;

    pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap;
    pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer;
    pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap);
    pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp);
    pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer;

    pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor);
    pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16);

    pub fn clock_is_24h_style() -> u8;

    pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern fn(*mut TM, TimeUnits));

    pub fn time(t: *mut usize) -> usize;
    pub fn localtime(now: *const usize) -> *mut TM;

    pub fn app_log(level: u8, filename: *const c_char, line_num: u32, msg: *const c_char);
}
