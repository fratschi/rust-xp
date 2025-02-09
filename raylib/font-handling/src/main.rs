use raylib::{color::Color, ffi::{self, rlTextureFilter, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle}, text::Font};

const FONT : &str = "font/DMMono-Medium.ttf";
const FONT_SIZE: i32 = 200;
const LABEL_SIZE: i32 = 20;
const FPS : u32 = 60;
const DEFAULT_KEY: char= 'a';
const TEXT_COLOR : Color = Color::WHITE;
struct Raylib {
    rl: raylib::RaylibHandle,
    rt: raylib::RaylibThread,
}

impl Raylib {
    pub fn new() -> Self {
        let (rl, rt) = raylib::init()
            .size(1024, 768 )
            .title("Font Handling")
            .build();   
        Raylib {rl,rt}
    }   
}

struct FontSet {
    font_none: Font,
    font_bilinear: Font,
    font_trilinear: Font,
    font_anisotropic_4X: Font,
    font_anisotropic_8X: Font,
    font_anisotropic_16X: Font,
}

fn main() {
    let mut r = Raylib::new();

    r.rl.set_target_fps(FPS);

    let font_set = FontSet{
        font_none: r.rl.load_font_ex(&r.rt,FONT, FONT_SIZE, None).unwrap(),
        font_bilinear: r.rl.load_font_ex(&r.rt,FONT, FONT_SIZE, None).unwrap(),
        font_trilinear: r.rl.load_font_ex(&r.rt,FONT, FONT_SIZE, None).unwrap(),
        font_anisotropic_4X: r.rl.load_font_ex(&r.rt,FONT, FONT_SIZE, None).unwrap(),
        font_anisotropic_8X: r.rl.load_font_ex(&r.rt,FONT, FONT_SIZE, None).unwrap(),
        font_anisotropic_16X: r.rl.load_font_ex(&r.rt,FONT, FONT_SIZE, None).unwrap(),
    };
   
    unsafe {
        // Don't know how to do this safely for now
        ffi::SetTextureFilter(font_set.font_bilinear.texture, rlTextureFilter::RL_TEXTURE_FILTER_BILINEAR as i32);
        ffi::SetTextureFilter(font_set.font_trilinear.texture, rlTextureFilter::RL_TEXTURE_FILTER_TRILINEAR as i32);
        ffi::SetTextureFilter(font_set.font_anisotropic_4X.texture, rlTextureFilter::RL_TEXTURE_FILTER_ANISOTROPIC_4X as i32);
        ffi::SetTextureFilter(font_set.font_anisotropic_8X.texture, rlTextureFilter::RL_TEXTURE_FILTER_ANISOTROPIC_8X as i32);
        ffi::SetTextureFilter(font_set.font_anisotropic_16X.texture, rlTextureFilter::RL_TEXTURE_FILTER_ANISOTROPIC_16X as i32);
    }
     
    let mut key: i32 = DEFAULT_KEY as i32;    

    while !r.rl.window_should_close() {
        let mut d = r.rl.begin_drawing(&r.rt);
        d.clear_background(Color::BLACK);
        
        unsafe {
            // Can't get the key press in safe rust here
            let k =  ffi::GetCharPressed();
            if k > 0 {
                key = k;
            }
        }

        draw(&font_set, &mut d, 100.0, 100.0, key);
        draw(&font_set, &mut d, 200.0, 200.0, key);
        draw(&font_set, &mut d, 350.0, 300.0, key);       


        // Horrible layouted labels
        d.draw_text("100px", 10, 140, LABEL_SIZE, TEXT_COLOR);
        d.draw_text("200px", 10, 310, LABEL_SIZE, TEXT_COLOR);
        d.draw_text("300px", 10, 520, LABEL_SIZE, TEXT_COLOR);

        d.draw_text("No Filter", 100, 700, LABEL_SIZE, TEXT_COLOR);
        d.draw_text("Bilinear", 260, 730, LABEL_SIZE, TEXT_COLOR);
        d.draw_text("Trilinear", 400, 700, LABEL_SIZE, TEXT_COLOR);

        d.draw_text("Anisotropic_4X", 530, 730, LABEL_SIZE, TEXT_COLOR);
        d.draw_text("Anisotropic_8X", 680, 700, LABEL_SIZE, TEXT_COLOR);
        d.draw_text("Anisotropic_16X", 830, 730, LABEL_SIZE, TEXT_COLOR);

    }
}

fn draw(fs: &FontSet, d: &mut RaylibDrawHandle, y: f32, s: f32, c: i32) {
    let str = &char::from_u32(c as u32).unwrap_or(DEFAULT_KEY).to_string();
    // TODO use proper table grid
    d.draw_text_ex(&fs.font_none, str, Vector2{x:150.0-s*0.25, y:y}, s, 0.0, TEXT_COLOR);
    d.draw_text_ex(&fs.font_bilinear, str, Vector2{x:300.0-s*0.25, y:y}, s, 0.0, TEXT_COLOR);
    d.draw_text_ex(&fs.font_trilinear, str, Vector2{x:450.0-s*0.25, y:y}, s, 0.0, TEXT_COLOR);
    d.draw_text_ex(&fs.font_anisotropic_4X, str, Vector2{x:600.0-s*0.25, y:y}, s, 0.0, TEXT_COLOR);
    d.draw_text_ex(&fs.font_anisotropic_8X, str, Vector2{x:750.0-s*0.25, y:y}, s, 0.0, TEXT_COLOR);
    d.draw_text_ex(&fs.font_anisotropic_16X, str, Vector2{x:900.0-s*0.25, y:y}, s, 0.0, TEXT_COLOR);
}