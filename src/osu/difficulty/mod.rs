use std::{cmp, pin::Pin};

use rosu_map::section::general::GameMode;
<<<<<<< HEAD
use skills::{aim::Aim, flashlight::Flashlight, speed::Speed, strain::OsuStrainSkill};

use crate::{
    any::difficulty::{skills::StrainSkill, Difficulty},
=======
use skills::{
    flashlight::Flashlight,
    strain::{DifficultyValue, OsuStrainSkill, UsedOsuStrainSkills},
};

use crate::{
    any::difficulty::{skills::Skill, Difficulty},
>>>>>>> 42db299 (meow)
    model::{beatmap::BeatmapAttributes, mode::ConvertError, mods::GameMods},
    osu::{
        convert::convert_objects,
        difficulty::{object::OsuDifficultyObject, scaling_factor::ScalingFactor},
        object::OsuObject,
        performance::PERFORMANCE_BASE_MULTIPLIER,
    },
    Beatmap,
};

use self::skills::OsuSkills;

use super::attributes::OsuDifficultyAttributes;

pub mod gradual;
mod object;
pub mod scaling_factor;
pub mod skills;

const DIFFICULTY_MULTIPLIER: f64 = 0.0675;

const HD_FADE_IN_DURATION_MULTIPLIER: f64 = 0.4;
const HD_FADE_OUT_DURATION_MULTIPLIER: f64 = 0.3;

pub fn difficulty(
    difficulty: &Difficulty,
    map: &Beatmap,
) -> Result<OsuDifficultyAttributes, ConvertError> {
    let map = map.convert_ref(GameMode::Osu, difficulty.get_mods())?;

<<<<<<< HEAD
    let DifficultyValues { skills, mut attrs } = DifficultyValues::calculate(difficulty, &map);

    let mods = difficulty.get_mods();

    DifficultyValues::eval(&mut attrs, mods, &skills);
=======
    let DifficultyValues {
        skills:
            OsuSkills {
                aim,
                aim_no_sliders,
                speed,
                flashlight,
            },
        mut attrs,
    } = DifficultyValues::calculate(difficulty, &map);

    let aim_difficulty_value = aim.difficulty_value();
    let aim_no_sliders_difficulty_value = aim_no_sliders.difficulty_value();
    let speed_relevant_note_count = speed.relevant_note_count();
    let speed_difficulty_value = speed.difficulty_value();
    let flashlight_difficulty_value = flashlight.difficulty_value();

    let mods = difficulty.get_mods();

    DifficultyValues::eval(
        &mut attrs,
        mods,
        &aim_difficulty_value,
        &aim_no_sliders_difficulty_value,
        &speed_difficulty_value,
        speed_relevant_note_count,
        flashlight_difficulty_value,
    );
>>>>>>> 42db299 (meow)

    Ok(attrs)
}

pub struct OsuDifficultySetup {
    scaling_factor: ScalingFactor,
    map_attrs: BeatmapAttributes,
    attrs: OsuDifficultyAttributes,
    time_preempt: f64,
}

impl OsuDifficultySetup {
    pub fn new(difficulty: &Difficulty, map: &Beatmap) -> Self {
        let clock_rate = difficulty.get_clock_rate();
        let map_attrs = map.attributes().difficulty(difficulty).build();
        let scaling_factor = ScalingFactor::new(map_attrs.cs);

        let attrs = OsuDifficultyAttributes {
            ar: map_attrs.ar,
            hp: map_attrs.hp,
<<<<<<< HEAD
            great_hit_window: map_attrs.hit_windows.od_great,
            ok_hit_window: map_attrs.hit_windows.od_ok.unwrap_or(0.0),
            meh_hit_window: map_attrs.hit_windows.od_meh.unwrap_or(0.0),
=======
            od: map_attrs.od,
>>>>>>> 42db299 (meow)
            ..Default::default()
        };

        let time_preempt = f64::from((map_attrs.hit_windows.ar * clock_rate) as f32);

        Self {
            scaling_factor,
            map_attrs,
            attrs,
            time_preempt,
        }
    }
}

pub struct DifficultyValues {
    pub skills: OsuSkills,
    pub attrs: OsuDifficultyAttributes,
}

impl DifficultyValues {
    pub fn calculate(difficulty: &Difficulty, map: &Beatmap) -> Self {
        let mods = difficulty.get_mods();
        let take = difficulty.get_passed_objects();

        let OsuDifficultySetup {
            scaling_factor,
            map_attrs,
            mut attrs,
            time_preempt,
        } = OsuDifficultySetup::new(difficulty, map);

        let mut osu_objects = convert_objects(
            map,
            &scaling_factor,
            mods.reflection(),
            time_preempt,
            take,
            &mut attrs,
        );

        let osu_object_iter = osu_objects.iter_mut().map(Pin::new);

        let diff_objects =
            Self::create_difficulty_objects(difficulty, &scaling_factor, osu_object_iter);

        let mut skills = OsuSkills::new(mods, &scaling_factor, &map_attrs, time_preempt);

<<<<<<< HEAD
        // The first hit object has no difficulty object
        let take_diff_objects = cmp::min(map.hit_objects.len(), take).saturating_sub(1);

        for hit_object in diff_objects.iter().take(take_diff_objects) {
            skills.process(hit_object, &diff_objects);
=======
        {
            let mut aim = Skill::new(&mut skills.aim, &diff_objects);
            let mut aim_no_sliders = Skill::new(&mut skills.aim_no_sliders, &diff_objects);
            let mut speed = Skill::new(&mut skills.speed, &diff_objects);
            let mut flashlight = Skill::new(&mut skills.flashlight, &diff_objects);

            // The first hit object has no difficulty object
            let take_diff_objects = cmp::min(map.hit_objects.len(), take).saturating_sub(1);

            for hit_object in diff_objects.iter().take(take_diff_objects) {
                aim.process(hit_object);
                aim_no_sliders.process(hit_object);
                speed.process(hit_object);
                flashlight.process(hit_object);
            }
>>>>>>> 42db299 (meow)
        }

        Self { skills, attrs }
    }

    /// Process the difficulty values and store the results in `attrs`.
<<<<<<< HEAD
    pub fn eval(attrs: &mut OsuDifficultyAttributes, mods: &GameMods, skills: &OsuSkills) {
        let OsuSkills {
            aim,
            aim_no_sliders,
            speed,
            flashlight,
        } = skills;

        let aim_difficulty_value = aim.cloned_difficulty_value();

        let mut aim_rating = aim_difficulty_value.sqrt() * DIFFICULTY_MULTIPLIER;
        let aim_difficult_strain_count = aim.count_top_weighted_strains(aim_difficulty_value);

        let difficult_sliders = aim.get_difficult_sliders();

        let aim_rating_no_sliders =
            f64::sqrt(aim_no_sliders.cloned_difficulty_value()) * DIFFICULTY_MULTIPLIER;
=======
    pub fn eval(
        attrs: &mut OsuDifficultyAttributes,
        mods: &GameMods,
        aim: &UsedOsuStrainSkills<DifficultyValue>,
        aim_no_sliders: &UsedOsuStrainSkills<DifficultyValue>,
        speed: &UsedOsuStrainSkills<DifficultyValue>,
        speed_relevant_note_count: f64,
        flashlight_difficulty_value: f64,
    ) {
        let mut aim_rating = aim.difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER;
        let aim_rating_no_sliders =
            aim_no_sliders.difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER;
        let mut speed_rating = speed.difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER;
        let mut flashlight_rating = flashlight_difficulty_value.sqrt() * DIFFICULTY_MULTIPLIER;
>>>>>>> 42db299 (meow)

        let slider_factor = if aim_rating > 0.0 {
            aim_rating_no_sliders / aim_rating
        } else {
            1.0
        };

<<<<<<< HEAD
        let speed_difficulty_value = speed.cloned_difficulty_value();
        let mut speed_rating = f64::sqrt(speed_difficulty_value) * DIFFICULTY_MULTIPLIER;
        let speed_difficult_strain_count = speed.count_top_weighted_strains(speed_difficulty_value);

        let mut flashlight_rating =
            f64::sqrt(flashlight.cloned_difficulty_value()) * DIFFICULTY_MULTIPLIER;
=======
        let aim_difficult_strain_count = aim.count_difficult_strains();
        let speed_difficult_strain_count = speed.count_difficult_strains();
>>>>>>> 42db299 (meow)

        if mods.td() {
            aim_rating = aim_rating.powf(0.8);
            flashlight_rating = flashlight_rating.powf(0.8);
        }

        if mods.rx() {
            aim_rating *= 0.9;
            speed_rating = 0.0;
            flashlight_rating *= 0.7;
<<<<<<< HEAD
        } else if mods.ap() {
            speed_rating *= 0.5;
=======
        }

        if mods.ap() {
>>>>>>> 42db299 (meow)
            aim_rating = 0.0;
            flashlight_rating *= 0.4;
        }

<<<<<<< HEAD
        let base_aim_performance = Aim::difficulty_to_performance(aim_rating);
        let base_speed_performance = Speed::difficulty_to_performance(speed_rating);
=======
        let base_aim_performance = OsuStrainSkill::difficulty_to_performance(aim_rating);
        let base_speed_performance = OsuStrainSkill::difficulty_to_performance(speed_rating);
>>>>>>> 42db299 (meow)

        let base_flashlight_performance = if mods.fl() {
            Flashlight::difficulty_to_performance(flashlight_rating)
        } else {
            0.0
        };

        let base_performance = ((base_aim_performance).powf(1.1)
            + (base_speed_performance).powf(1.1)
            + (base_flashlight_performance).powf(1.1))
        .powf(1.0 / 1.1);

        let star_rating = if base_performance > 0.00001 {
            PERFORMANCE_BASE_MULTIPLIER.cbrt()
                * 0.027
                * ((100_000.0 / 2.0_f64.powf(1.0 / 1.1) * base_performance).cbrt() + 4.0)
        } else {
            0.0
        };

        attrs.aim = aim_rating;
<<<<<<< HEAD
        attrs.aim_difficult_slider_count = difficult_sliders;
=======
>>>>>>> 42db299 (meow)
        attrs.speed = speed_rating;
        attrs.flashlight = flashlight_rating;
        attrs.slider_factor = slider_factor;
        attrs.aim_difficult_strain_count = aim_difficult_strain_count;
        attrs.speed_difficult_strain_count = speed_difficult_strain_count;
        attrs.stars = star_rating;
<<<<<<< HEAD
        attrs.speed_note_count = speed.relevant_note_count();
=======
        attrs.speed_note_count = speed_relevant_note_count;
>>>>>>> 42db299 (meow)
    }

    pub fn create_difficulty_objects<'a>(
        difficulty: &Difficulty,
        scaling_factor: &ScalingFactor,
        osu_objects: impl ExactSizeIterator<Item = Pin<&'a mut OsuObject>>,
    ) -> Vec<OsuDifficultyObject<'a>> {
        let take = difficulty.get_passed_objects();
        let clock_rate = difficulty.get_clock_rate();

        let mut osu_objects_iter = osu_objects
            .map(|h| OsuDifficultyObject::compute_slider_cursor_pos(h, scaling_factor.radius))
            .map(Pin::into_ref);

        let Some(mut last) = osu_objects_iter.next().filter(|_| take > 0) else {
            return Vec::new();
        };

        let mut last_last = None;

        osu_objects_iter
            .enumerate()
            .map(|(idx, h)| {
                let diff_object = OsuDifficultyObject::new(
                    h.get_ref(),
                    last.get_ref(),
                    last_last.as_deref(),
                    clock_rate,
                    idx,
                    scaling_factor,
                );

                last_last = Some(last);
                last = h;

                diff_object
            })
            .collect()
    }
}
