<<<<<<< HEAD
use std::f64::consts::PI;

use crate::{
    taiko::difficulty::{
        object::{TaikoDifficultyObject, TaikoDifficultyObjects},
        rhythm::data::same_rhythm_hit_object_grouping::SameRhythmHitObjectGrouping,
    },
    util::{
        difficulty::{bell_curve, logistic},
        sync::RefCount,
    },
};

use super::stamina::StaminaEvaluator;

define_skill! {
    #[derive(Clone)]
    pub struct Rhythm: StrainDecaySkill => TaikoDifficultyObjects[TaikoDifficultyObject] {
        great_hit_window: f64,
    }
}

impl Rhythm {
    const SKILL_MULTIPLIER: f64 = 1.0;
    const STRAIN_DECAY_BASE: f64 = 0.4;

    fn strain_value_of(
        &mut self,
        curr: &TaikoDifficultyObject,
        objects: &TaikoDifficultyObjects,
    ) -> f64 {
        let mut difficulty = RhythmEvaluator::evaluate_diff_of(curr, self.great_hit_window);

        // * To prevent abuse of exceedingly long intervals between awkward rhythms, we penalise its difficulty.
        let stamina_difficulty = StaminaEvaluator::evaluate_diff_of(curr, objects) - 0.5; // * Remove base strain
        difficulty *= logistic(stamina_difficulty, 1.0 / 15.0, 50.0, None);

        difficulty
    }
}

struct RhythmEvaluator;

impl RhythmEvaluator {
    fn evaluate_diff_of(hit_object: &TaikoDifficultyObject, hit_window: f64) -> f64 {
        let rhythm_data = &hit_object.rhythm_data;
        let mut difficulty = 0.0;

        let mut same_rhythm = 0.0;
        let mut same_pattern = 0.0;
        let mut interval_penalty = 0.0;

        // * Difficulty for SameRhythmGroupedHitObjects
        if let Some(ref same_rhythm_grouped) = rhythm_data.same_rhythm_grouped_hit_objects {
            if same_rhythm_grouped
                .get()
                .first_hit_object()
                .is_some_and(|h| &*h.get() == hit_object)
            {
                same_rhythm += 10.0 * Self::evaluate_diff_of_(same_rhythm_grouped, hit_window);
                interval_penalty =
                    Self::repeated_interval_penalty(same_rhythm_grouped, hit_window, None);
            }
        }

        // * Difficulty for SamePatternsGroupedHitObjects
        if let Some(ref same_pattern_grouped) = rhythm_data.same_patterns_grouped_hit_objects {
            if same_pattern_grouped
                .get()
                .first_hit_object()
                .is_some_and(|h| &*h.get() == hit_object)
            {
                same_pattern += 1.15
                    * Self::ratio_difficulty(same_pattern_grouped.get().interval_ratio(), None);
            }
        }

        difficulty += f64::max(same_rhythm, same_pattern) * interval_penalty;

        difficulty
    }

    fn evaluate_diff_of_(
        same_rhythm_grouped_hit_objects: &RefCount<SameRhythmHitObjectGrouping>,
        hit_window: f64,
    ) -> f64 {
        let mut interval_diff = Self::ratio_difficulty(
            same_rhythm_grouped_hit_objects
                .get()
                .hit_object_interval_ratio,
            None,
        );
        let prev_interval = same_rhythm_grouped_hit_objects
            .get()
            .upgraded_previous()
            .and_then(|h| h.get().hit_object_interval);

        interval_diff *=
            Self::repeated_interval_penalty(same_rhythm_grouped_hit_objects, hit_window, None);

        let borrowed = same_rhythm_grouped_hit_objects.get();
        let duration = borrowed.duration();

        // * If a previous interval exists and there are multiple hit objects in the sequence:
        if let Some(prev_interval) = prev_interval.filter(|_| borrowed.hit_objects.len() > 1) {
            if let Some(duration) = duration {
                let expected_duration_from_prev = prev_interval * borrowed.hit_objects.len() as f64;
                let duration_diff = duration - expected_duration_from_prev;

                if duration_diff > 0.0 {
                    interval_diff *= logistic(duration_diff / hit_window, 0.7, 1.0, Some(1.0));
                }
            }
        }

        // Penalise patterns that can be hit within a single hit window.
        if let Some(duration) = duration {
            interval_diff *= logistic(duration / hit_window, 0.6, 1.0, Some(1.0));
        }

        f64::powf(interval_diff, 0.75)
    }

    fn repeated_interval_penalty(
        same_rhythm_grouped_hit_objects: &RefCount<SameRhythmHitObjectGrouping>,
        hit_window: f64,
        threshold: Option<f64>,
    ) -> f64 {
        let threshold = threshold.unwrap_or(0.1);

        let same_interval =
            |start_object: RefCount<SameRhythmHitObjectGrouping>, interval_count: usize| -> f64 {
                let mut intervals = Vec::new();
                let mut curr_object = Some(start_object);

                let mut i = 0;

                while let Some(curr) = curr_object.filter(|_| i < interval_count) {
                    let curr = curr.get();

                    if let Some(interval) = curr.hit_object_interval {
                        intervals.push(interval);
                    }

                    curr_object = curr.upgraded_previous();
                    i += 1;
                }

                if intervals.len() < interval_count {
                    return 1.0; // * No penalty if there aren't enough valid intervals.
                }

                for i in 0..intervals.len() {
                    for j in i + 1..intervals.len() {
                        let ratio = intervals[i] / intervals[j];

                        // * If any two intervals are similar, apply a penalty.
                        if f64::abs(1.0 - ratio) <= threshold {
                            return 0.8;
                        }
                    }
                }

                // * No penalty if all intervals are different.
                1.0
            };

        let long_interval_penalty =
            same_interval(RefCount::clone(same_rhythm_grouped_hit_objects), 3);

        let short_interval_penalty = if same_rhythm_grouped_hit_objects.get().hit_objects.len() < 6
        {
            same_interval(RefCount::clone(same_rhythm_grouped_hit_objects), 4)
        } else {
            // * Returns a non-penalty if there are 6 or more notes within an interval.
            1.0
        };

        // * The duration penalty is based on hit object duration relative to hitWindow.
        let duration_penalty = same_rhythm_grouped_hit_objects
            .get()
            .duration()
            .map_or(0.5, |duration| {
                f64::max(1.0 - duration * 2.0 / hit_window, 0.5)
            });

        f64::min(long_interval_penalty, short_interval_penalty) * duration_penalty
    }

    fn ratio_difficulty(mut ratio: f64, terms: Option<i32>) -> f64 {
        let terms = terms.unwrap_or(8);
        let mut difficulty = 0.0;

        // * Validate the ratio by ensuring it is a normal number in cases where maps breach regular mapping conditions.
        ratio = if ratio.is_normal() { ratio } else { 0.0 };

        for i in 1..=terms {
            difficulty += Self::term_penalty(ratio, i, 4.0, 1.0);
        }

        difficulty += f64::from(terms) / (1.0 + ratio);

        // * Give bonus to near-1 ratios
        difficulty += bell_curve(ratio, 1.0, 0.5, None);

        // * Penalize ratios that are VERY near 1
        difficulty -= bell_curve(ratio, 1.0, 0.3, None);

        difficulty = f64::max(difficulty, 0.0);
        difficulty /= f64::sqrt(8.0);

        difficulty
    }

    fn term_penalty(ratio: f64, denominator: i32, power: f64, multiplier: f64) -> f64 {
        -multiplier * f64::powf(f64::cos(f64::from(denominator) * PI * ratio), power)
=======
use std::cmp;

use crate::{
    any::difficulty::{
        object::IDifficultyObject,
        skills::{strain_decay, ISkill, Skill, StrainDecaySkill},
    },
    taiko::{
        difficulty::{
            object::{TaikoDifficultyObject, TaikoDifficultyObjects},
            rhythm::HitObjectRhythm,
        },
        object::HitType,
    },
    util::{float_ext::FloatExt, limited_queue::LimitedQueue, strains_vec::StrainsVec},
};

const SKILL_MULTIPLIER: f64 = 10.0;
const STRAIN_DECAY_BASE: f64 = 0.0;

const STRAIN_DECAY: f64 = 0.96;
const RHYTHM_HISTORY_MAX_LEN: usize = 8;

#[allow(clippy::struct_field_names)]
#[derive(Clone, Default)]
pub struct Rhythm {
    inner: StrainDecaySkill,
    rhythm_history: LimitedQueue<RhythmHistoryElement, RHYTHM_HISTORY_MAX_LEN>,
    curr_strain: f64,
    notes_since_rhythm_change: usize,
}

impl Rhythm {
    fn repetition_penalties(&mut self, hit_object: &TaikoDifficultyObject) -> f64 {
        let mut penalty = 1.0;

        self.rhythm_history
            .push(RhythmHistoryElement::new(hit_object));

        for most_recent_patterns_to_compare in
            2..=cmp::min(RHYTHM_HISTORY_MAX_LEN / 2, self.rhythm_history.len())
        {
            for start in (0..self.rhythm_history.len() - most_recent_patterns_to_compare).rev() {
                if !self.same_pattern(start, most_recent_patterns_to_compare) {
                    continue;
                }

                let notes_since = hit_object.idx - self.rhythm_history[start].idx;
                penalty *= Self::repetition_penalty(notes_since);

                break;
            }
        }

        penalty
    }

    fn same_pattern(&self, start: usize, most_recent_patterns_to_compare: usize) -> bool {
        let start = self.rhythm_history.iter().skip(start);

        let most_recent_patterns_to_compare = self
            .rhythm_history
            .iter()
            .skip(self.rhythm_history.len() - most_recent_patterns_to_compare);

        start
            .zip(most_recent_patterns_to_compare)
            .all(|(a, b)| a.rhythm == b.rhythm)
    }

    fn repetition_penalty(notes_since: usize) -> f64 {
        (0.032 * notes_since as f64).min(1.0)
    }

    fn pattern_len_penalty(pattern_len: usize) -> f64 {
        let pattern_len = pattern_len as f64;
        let short_pattern_penalty = (0.15 * pattern_len).min(1.0);
        let long_pattern_penalty = (2.5 - 0.15 * pattern_len).clamp(0.0, 1.0);

        short_pattern_penalty.min(long_pattern_penalty)
    }

    fn speed_penalty(&mut self, delta: f64) -> f64 {
        if delta < 80.0 {
            return 1.0;
        } else if delta < 210.0 {
            return (1.4 - 0.005 * delta).max(0.0);
        }

        self.reset_rhythm_and_strain();

        0.0
    }

    fn reset_rhythm_and_strain(&mut self) {
        self.curr_strain = 0.0;
        self.notes_since_rhythm_change = 0;
    }

    fn strain_value_of(&mut self, curr: &TaikoDifficultyObject) -> f64 {
        // * drum rolls and swells are exempt.
        if matches!(curr.base_hit_type, HitType::NonHit) {
            self.reset_rhythm_and_strain();

            return 0.0;
        }

        self.curr_strain *= STRAIN_DECAY;
        self.notes_since_rhythm_change += 1;

        // * rhythm difficulty zero (due to rhythm not changing) => no rhythm strain.
        if curr.rhythm.difficulty.eq(0.0) {
            return 0.0;
        }

        let mut obj_strain = curr.rhythm.difficulty;

        obj_strain *= self.repetition_penalties(curr);
        obj_strain *= Self::pattern_len_penalty(self.notes_since_rhythm_change);
        obj_strain *= self.speed_penalty(curr.delta_time);

        // * careful - needs to be done here since calls above read this value
        self.notes_since_rhythm_change = 0;

        self.curr_strain += obj_strain;

        self.curr_strain
    }

    const fn curr_strain(&self) -> f64 {
        self.inner.curr_strain
    }

    fn curr_strain_mut(&mut self) -> &mut f64 {
        &mut self.inner.curr_strain
    }

    fn strain_value_at(&mut self, curr: &TaikoDifficultyObject) -> f64 {
        *self.curr_strain_mut() *= strain_decay(curr.delta_time, STRAIN_DECAY_BASE);
        *self.curr_strain_mut() += self.strain_value_of(curr) * SKILL_MULTIPLIER;

        self.curr_strain()
    }

    pub fn get_curr_strain_peaks(self) -> StrainsVec {
        self.inner.get_curr_strain_peaks()
    }

    pub fn as_difficulty_value(&self) -> f64 {
        self.inner
            .clone()
            .difficulty_value(StrainDecaySkill::DECAY_WEIGHT)
    }
}

impl ISkill for Rhythm {
    type DifficultyObjects<'a> = TaikoDifficultyObjects;
}

impl Skill<'_, Rhythm> {
    fn calculate_initial_strain(&mut self, time: f64, curr: &TaikoDifficultyObject) -> f64 {
        let prev_start_time = curr
            .previous(0, &self.diff_objects.objects)
            .map_or(0.0, |prev| prev.get().start_time);

        self.inner.curr_strain() * strain_decay(time - prev_start_time, STRAIN_DECAY_BASE)
    }

    const fn curr_section_peak(&self) -> f64 {
        self.inner.inner.inner.curr_section_peak
    }

    fn curr_section_peak_mut(&mut self) -> &mut f64 {
        &mut self.inner.inner.inner.curr_section_peak
    }

    const fn curr_section_end(&self) -> f64 {
        self.inner.inner.inner.curr_section_end
    }

    fn curr_section_end_mut(&mut self) -> &mut f64 {
        &mut self.inner.inner.inner.curr_section_end
    }

    pub fn process(&mut self, curr: &TaikoDifficultyObject) {
        if curr.idx == 0 {
            *self.curr_section_end_mut() = (curr.start_time / StrainDecaySkill::SECTION_LEN).ceil()
                * StrainDecaySkill::SECTION_LEN;
        }

        while curr.start_time > self.curr_section_end() {
            self.inner.inner.save_curr_peak();
            let initial_strain = self.calculate_initial_strain(self.curr_section_end(), curr);
            self.inner.inner.start_new_section_from(initial_strain);
            *self.curr_section_end_mut() += StrainDecaySkill::SECTION_LEN;
        }

        let strain_value_at = self.inner.strain_value_at(curr);
        *self.curr_section_peak_mut() = strain_value_at.max(self.curr_section_peak());
    }
}

#[derive(Copy, Clone)]
struct RhythmHistoryElement {
    idx: usize,
    rhythm: &'static HitObjectRhythm,
}

impl RhythmHistoryElement {
    const fn new(difficulty_object: &TaikoDifficultyObject) -> Self {
        Self {
            idx: difficulty_object.idx,
            rhythm: difficulty_object.rhythm,
        }
    }
}

impl Default for RhythmHistoryElement {
    fn default() -> Self {
        Self {
            idx: 0,
            rhythm: HitObjectRhythm::static_ref(),
        }
>>>>>>> 42db299 (meow)
    }
}
