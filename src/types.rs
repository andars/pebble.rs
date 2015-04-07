#[repr(C)]
pub struct Window;

#[repr(C)]
pub struct Layer;

#[repr(C)]
pub struct TextLayer;

#[repr(C)]
pub struct ClickRecognizer;

#[repr(C)]
pub struct GPoint {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
pub struct GSize {
    pub w: u16,
    pub h: u16,
}

#[repr(C)]
pub struct GRect {
    pub origin: GPoint,
    pub size: GSize,
}

#[repr(C)]
pub struct WindowHandlers {
    pub load: extern fn(*mut Window),
    pub appear: extern fn(*mut Window),
    pub disappear: extern fn(*mut Window),
    pub unload: extern fn(*mut Window),
}

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}
