<<<<<<< HEAD
use crate::util::float_ext::FloatExt;

=======
>>>>>>> 42db299 (meow)
/// Effect-related info about this control point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EffectPoint {
    pub time: f64,
    pub kiai: bool,
<<<<<<< HEAD
    pub scroll_speed: f64,
=======
>>>>>>> 42db299 (meow)
}

impl EffectPoint {
    pub const DEFAULT_KIAI: bool = rosu_map::section::timing_points::EffectPoint::DEFAULT_KIAI;
<<<<<<< HEAD
    pub const DEFAULT_SCROLL_SPEED: f64 =
        rosu_map::section::timing_points::EffectPoint::DEFAULT_SCROLL_SPEED;

    pub const fn new(time: f64, kiai: bool) -> Self {
        Self {
            time,
            kiai,
            scroll_speed: Self::DEFAULT_SCROLL_SPEED,
        }
    }

    pub fn is_redundant(&self, existing: &Self) -> bool {
        self.kiai == existing.kiai && FloatExt::eq(self.scroll_speed, existing.scroll_speed)
=======

    pub const fn new(time: f64, kiai: bool) -> Self {
        Self { time, kiai }
    }

    pub const fn is_redundant(&self, existing: &Self) -> bool {
        self.kiai == existing.kiai
>>>>>>> 42db299 (meow)
    }
}

impl Default for EffectPoint {
    fn default() -> Self {
        Self {
            time: 0.0,
            kiai: Self::DEFAULT_KIAI,
<<<<<<< HEAD
            scroll_speed: Self::DEFAULT_SCROLL_SPEED,
=======
>>>>>>> 42db299 (meow)
        }
    }
}

pub fn effect_point_at(points: &[EffectPoint], time: f64) -> Option<&EffectPoint> {
    points
        .binary_search_by(|probe| probe.time.total_cmp(&time))
        .map_or_else(|i| i.checked_sub(1), Some)
        .map(|i| &points[i])
}
