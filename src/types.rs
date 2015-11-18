pub enum Window {}

pub enum Layer {}

pub enum TextLayer {}

pub enum ClickRecognizer {}

pub enum GBitmap {}

pub enum GContext {}

pub enum BitmapLayer {}

#[repr(C)]
pub struct TM {
    pub tm_sec: u32,
    pub tm_min: u32,
    pub tm_hour: u32,
    pub tm_mday: u32,
    pub tm_mon: u32,
    pub tm_year: u32,
    pub tm_wday: u32,
    pub tm_yday: u32,
    pub tm_isdst: u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPoint {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSize {
    pub w: u16,
    pub h: u16,
}

#[derive(Copy, Clone)]
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

#[repr(C)]
pub enum GCompOp {
    GCompOpAssign,
    GCompOpAssignInverted,
    GCompOpOr,
    GCompOpAnd,
    GCompOpClear,
    GCompOpSet
}

#[repr(C)]
pub enum GColor {
    GColorClear = -1,
    GColorBlack = 0,
    GColorWhite = 1
}

#[repr(C)]
pub enum TimeUnits {
    SECOND_UNIT=1,
    MINUTE_UNIT,
    HOUR_UNIT,
    DAY_UNIT,
    MONTH_UNIT,
    YEAR_UNIT
}

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

#[allow(non_camel_case_types)]
pub type c_char = u8;
