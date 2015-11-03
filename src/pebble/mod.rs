use types;

pub mod raw;

pub trait Layer {
    fn add_child(&self, layer: *mut types::Layer) {
        raw::layer_add_child(self.get_layer(), layer); 
    }
    fn get_layer(&self) -> *mut types::Layer;
}

pub struct Bitmap {
    pub bitmap: *mut types::GBitmap
}

impl Bitmap {
    pub fn new(resource_id: u32) -> Bitmap {
        Bitmap {
            bitmap: raw::gbitmap_create_with_resource(resource_id)
        }
    }
}

pub struct BitmapLayer {
    bitmap_layer: *mut types::BitmapLayer
}

impl BitmapLayer {
    pub fn new(bounds: types::GRect) -> BitmapLayer {
        BitmapLayer {
            bitmap_layer: raw::bitmap_layer_create(bounds)
        }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        raw::bitmap_layer_set_bitmap(self.bitmap_layer, bitmap.bitmap); 
    }

    pub fn set_compositing_mode(&self, mode: types::GCompOp) {
        raw::bitmap_layer_set_compositing_mode(self.bitmap_layer, mode);
    }
}

impl Layer for BitmapLayer {
    //should this be an option?
    //if bitmap_layer is null, pebble sdk has a problem
    fn get_layer(&self) -> *mut types::Layer {
        raw::bitmap_layer_get_layer(self.bitmap_layer)
    }
}

pub struct TextLayer {
    text_layer: *mut types::TextLayer
}

impl TextLayer {
    pub fn new(bounds: types::GRect) -> TextLayer {
        TextLayer {
            text_layer: raw::text_layer_create(bounds)
        }
    }

    pub fn new_from(raw: *mut types::TextLayer) -> TextLayer {
        TextLayer {
            text_layer: raw
        }
    }

    pub fn set_text(&self, text: &str) {
        raw::text_layer_set_text(self.text_layer, text);
    }

    pub fn get_raw(&self) -> *mut types::TextLayer {
        self.text_layer
    }
}

impl Layer for TextLayer {
    fn get_layer(&self) -> *mut types::Layer {
        raw::text_layer_get_layer(self.text_layer)
    }
}
