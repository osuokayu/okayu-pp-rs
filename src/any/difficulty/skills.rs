<<<<<<< HEAD
use crate::util::{float_ext::FloatExt, hint::unlikely, strains_vec::StrainsVec};

pub trait StrainSkill: Sized {
    type DifficultyObject<'a>;
    type DifficultyObjects<'a>: ?Sized;

    const DECAY_WEIGHT: f64 = 0.9;
    const SECTION_LENGTH: i32 = 400;

    fn process<'a>(
        &mut self,
        curr: &Self::DifficultyObject<'a>,
        objects: &Self::DifficultyObjects<'a>,
    );

    fn count_top_weighted_strains(&self, difficulty_value: f64) -> f64;

    fn save_current_peak(&mut self);

    fn start_new_section_from<'a>(
        &mut self,
        time: f64,
        curr: &Self::DifficultyObject<'a>,
        objects: &Self::DifficultyObjects<'a>,
    );

    fn into_current_strain_peaks(self) -> StrainsVec;

    fn get_current_strain_peaks(
        mut strain_peaks: StrainsVec,
        current_section_peak: f64,
    ) -> StrainsVec {
        strain_peaks.push(current_section_peak);
=======
use crate::util::strains_vec::StrainsVec;

pub fn strain_decay(ms: f64, strain_decay_base: f64) -> f64 {
    strain_decay_base.powf(ms / 1000.0)
}

/// Wrapper around a difficulty skill that carries a list of all difficulty
/// objects.
pub struct Skill<'a, S: ISkill> {
    pub inner: &'a mut S,
    pub diff_objects: &'a S::DifficultyObjects<'a>,
}

impl<'a, S: ISkill> Skill<'a, S> {
    pub fn new(skill: &'a mut S, diff_objects: &'a S::DifficultyObjects<'a>) -> Self {
        Self {
            inner: skill,
            diff_objects,
        }
    }
}

/// Trait required for [`Skill`].
pub trait ISkill {
    type DifficultyObjects<'a>: ?Sized;
}

#[derive(Clone)]
pub struct StrainSkill {
    pub curr_section_peak: f64,
    pub curr_section_end: f64,
    pub strain_peaks: StrainsVec,
}

impl Default for StrainSkill {
    fn default() -> Self {
        Self {
            curr_section_peak: 0.0,
            curr_section_end: 0.0,
            // mean=386.81 | median=279
            strain_peaks: StrainsVec::with_capacity(256),
        }
    }
}

impl StrainSkill {
    pub const DECAY_WEIGHT: f64 = 0.9;
    pub const SECTION_LEN: f64 = 400.0;

    pub fn save_curr_peak(&mut self) {
        self.strain_peaks.push(self.curr_section_peak);
    }

    pub fn start_new_section_from(&mut self, initial_strain: f64) {
        self.curr_section_peak = initial_strain;
    }

    pub fn get_curr_strain_peaks(self) -> StrainsVec {
        let mut strain_peaks = self.strain_peaks;
        strain_peaks.push(self.curr_section_peak);
>>>>>>> 42db299 (meow)

        strain_peaks
    }

<<<<<<< HEAD
    fn difficulty_value(current_strain_peaks: StrainsVec) -> f64;

    fn into_difficulty_value(self) -> f64;

    fn cloned_difficulty_value(&self) -> f64;
}

pub trait StrainDecaySkill: StrainSkill {
    fn calculate_initial_strain<'a>(
        &self,
        time: f64,
        curr: &Self::DifficultyObject<'a>,
        objects: &Self::DifficultyObjects<'a>,
    ) -> f64;

    fn strain_value_at<'a>(
        &mut self,
        curr: &Self::DifficultyObject<'a>,
        objects: &Self::DifficultyObjects<'a>,
    ) -> f64;

    fn strain_decay(ms: f64) -> f64;
}

pub fn count_top_weighted_strains(object_strains: &[f64], difficulty_value: f64) -> f64 {
    if unlikely(object_strains.is_empty()) {
        return 0.0;
    }

    // * What would the top strain be if all strain values were identical
    let consistent_top_strain = difficulty_value / 10.0;

    if unlikely(FloatExt::eq(consistent_top_strain, 0.0)) {
        return object_strains.len() as f64;
    }

    // * Use a weighted sum of all strains. Constants are arbitrary and give nice values
    object_strains
        .iter()
        .map(|s| 1.1 / (1.0 + f64::exp(-10.0 * (s / consistent_top_strain - 0.88))))
        .sum()
}

pub fn difficulty_value(current_strain_peaks: StrainsVec, decay_weight: f64) -> f64 {
    let mut difficulty = 0.0;
    let mut weight = 1.0;

    // * Sections with 0 strain are excluded to avoid worst-case time complexity of the following sort (e.g. /b/2351871).
    // * These sections will not contribute to the difficulty.
    let mut peaks = current_strain_peaks;
    peaks.retain_non_zero_and_sort();

    // SAFETY: we just removed all zeros
    let peaks = unsafe { peaks.transmute_into_vec() };

    // * Difficulty is the weighted sum of the highest strains from every section.
    // * We're sorting from highest to lowest strain.
    for strain in peaks {
        difficulty += strain * weight;
        weight *= decay_weight;
    }

    difficulty
}

pub fn strain_decay(ms: f64, strain_decay_base: f64) -> f64 {
    f64::powf(strain_decay_base, ms / 1000.0)
=======
    pub fn difficulty_value(self, decay_weight: f64) -> f64 {
        let mut difficulty = 0.0;
        let mut weight = 1.0;

        let mut peaks = self.get_curr_strain_peaks();

        for strain in peaks.sorted_non_zero_iter() {
            difficulty += strain * weight;
            weight *= decay_weight;
        }

        difficulty
    }
}

#[derive(Clone, Default)]
pub struct StrainDecaySkill {
    pub inner: StrainSkill,
    pub curr_strain: f64,
}

impl StrainDecaySkill {
    pub const DECAY_WEIGHT: f64 = StrainSkill::DECAY_WEIGHT;
    pub const SECTION_LEN: f64 = StrainSkill::SECTION_LEN;

    pub fn save_curr_peak(&mut self) {
        self.inner.save_curr_peak();
    }

    pub fn start_new_section_from(&mut self, initial_strain: f64) {
        self.inner.start_new_section_from(initial_strain);
    }

    pub fn get_curr_strain_peaks(self) -> StrainsVec {
        self.inner.get_curr_strain_peaks()
    }

    pub fn difficulty_value(self, decay_weight: f64) -> f64 {
        self.inner.difficulty_value(decay_weight)
    }
>>>>>>> 42db299 (meow)
}
