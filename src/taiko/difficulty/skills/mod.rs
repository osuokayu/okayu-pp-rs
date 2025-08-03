<<<<<<< HEAD
use reading::Reading;

use self::{color::Color, rhythm::Rhythm, stamina::Stamina};

pub mod color;
pub mod reading;
=======
use self::{color::Color, rhythm::Rhythm, stamina::Stamina};

pub mod color;
>>>>>>> 42db299 (meow)
pub mod rhythm;
pub mod stamina;

#[derive(Clone)]
pub struct TaikoSkills {
    pub rhythm: Rhythm,
<<<<<<< HEAD
    pub reading: Reading,
=======
>>>>>>> 42db299 (meow)
    pub color: Color,
    pub stamina: Stamina,
    pub single_color_stamina: Stamina,
}

impl TaikoSkills {
<<<<<<< HEAD
    pub fn new(great_hit_window: f64, is_convert: bool) -> Self {
        Self {
            rhythm: Rhythm::new(great_hit_window),
            reading: Reading::new(),
            color: Color::new(),
            stamina: Stamina::new(false, is_convert),
            single_color_stamina: Stamina::new(true, is_convert),
=======
    pub fn new() -> Self {
        Self {
            rhythm: Rhythm::default(),
            color: Color::default(),
            stamina: Stamina::new(false),
            single_color_stamina: Stamina::new(true),
>>>>>>> 42db299 (meow)
        }
    }
}
