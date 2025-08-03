use std::cmp;

use rosu_map::section::general::GameMode;

<<<<<<< HEAD
use self::calculator::ManiaPerformanceCalculator;

=======
>>>>>>> 42db299 (meow)
use crate::{
    any::{Difficulty, HitResultPriority, IntoModePerformance, IntoPerformance},
    model::{mode::ConvertError, mods::GameMods},
    osu::OsuPerformance,
    util::map_or_attrs::MapOrAttrs,
    Performance,
};

<<<<<<< HEAD
use super::{attributes::ManiaPerformanceAttributes, score_state::ManiaScoreState, Mania};

mod calculator;
=======
use super::{
    attributes::{ManiaDifficultyAttributes, ManiaPerformanceAttributes},
    score_state::ManiaScoreState,
    Mania,
};

>>>>>>> 42db299 (meow)
pub mod gradual;

/// Performance calculator on osu!mania maps.
#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct ManiaPerformance<'map> {
    map_or_attrs: MapOrAttrs<'map, Mania>,
    difficulty: Difficulty,
    n320: Option<u32>,
    n300: Option<u32>,
    n200: Option<u32>,
    n100: Option<u32>,
    n50: Option<u32>,
    misses: Option<u32>,
    acc: Option<f64>,
    hitresult_priority: HitResultPriority,
}

impl<'map> ManiaPerformance<'map> {
    /// Create a new performance calculator for osu!mania maps.
    ///
    /// The argument `map_or_attrs` must be either
    /// - previously calculated attributes ([`ManiaDifficultyAttributes`]
    ///   or [`ManiaPerformanceAttributes`])
    /// - a [`Beatmap`] (by reference or value)
    ///
    /// If a map is given, difficulty attributes will need to be calculated
    /// internally which is a costly operation. Hence, passing attributes
    /// should be prefered.
    ///
    /// However, when passing previously calculated attributes, make sure they
    /// have been calculated for the same map and [`Difficulty`] settings.
    /// Otherwise, the final attributes will be incorrect.
    ///
    /// [`Beatmap`]: crate::model::beatmap::Beatmap
<<<<<<< HEAD
    /// [`ManiaDifficultyAttributes`]: crate::mania::ManiaDifficultyAttributes
=======
>>>>>>> 42db299 (meow)
    pub fn new(map_or_attrs: impl IntoModePerformance<'map, Mania>) -> Self {
        map_or_attrs.into_performance()
    }

    /// Try to create a new performance calculator for osu!mania maps.
    ///
    /// Returns `None` if `map_or_attrs` does not belong to osu!mania i.e.
    /// a [`DifficultyAttributes`] or [`PerformanceAttributes`] of a different
    /// mode.
    ///
    /// See [`ManiaPerformance::new`] for more information.
    ///
    /// [`DifficultyAttributes`]: crate::any::DifficultyAttributes
    /// [`PerformanceAttributes`]: crate::any::PerformanceAttributes
    pub fn try_new(map_or_attrs: impl IntoPerformance<'map>) -> Option<Self> {
        if let Performance::Mania(calc) = map_or_attrs.into_performance() {
            Some(calc)
        } else {
            None
        }
    }

    /// Specify mods.
    ///
    /// Accepted types are
    /// - `u32`
    /// - [`rosu_mods::GameModsLegacy`]
    /// - [`rosu_mods::GameMods`]
    /// - [`rosu_mods::GameModsIntermode`]
    /// - [`&rosu_mods::GameModsIntermode`](rosu_mods::GameModsIntermode)
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub fn mods(mut self, mods: impl Into<GameMods>) -> Self {
        self.difficulty = self.difficulty.mods(mods);

        self
    }

    /// Use the specified settings of the given [`Difficulty`].
    pub fn difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;

        self
    }

    /// Amount of passed objects for partial plays, e.g. a fail.
    ///
    /// If you want to calculate the performance after every few objects,
    /// instead of using [`ManiaPerformance`] multiple times with different
    /// `passed_objects`, you should use [`ManiaGradualPerformance`].
    ///
    /// [`ManiaGradualPerformance`]: crate::mania::ManiaGradualPerformance
    pub fn passed_objects(mut self, passed_objects: u32) -> Self {
        self.difficulty = self.difficulty.passed_objects(passed_objects);

        self
    }

    /// Adjust the clock rate used in the calculation.
    ///
    /// If none is specified, it will take the clock rate based on the mods
    /// i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | 0.01    | 100     |
    pub fn clock_rate(mut self, clock_rate: f64) -> Self {
        self.difficulty = self.difficulty.clock_rate(clock_rate);

        self
    }

    /// Override a beatmap's set HP.
    ///
    /// `with_mods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn hp(mut self, hp: f32, with_mods: bool) -> Self {
        self.difficulty = self.difficulty.hp(hp, with_mods);

        self
    }

    /// Override a beatmap's set OD.
    ///
    /// `with_mods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn od(mut self, od: f32, with_mods: bool) -> Self {
        self.difficulty = self.difficulty.od(od, with_mods);

        self
    }

    /// Specify the accuracy of a play between `0.0` and `100.0`.
    /// This will be used to generate matching hitresults.
    pub fn accuracy(mut self, acc: f64) -> Self {
        self.acc = Some(acc.clamp(0.0, 100.0) / 100.0);

        self
    }

    /// Specify how hitresults should be generated.
    ///
    /// Defauls to [`HitResultPriority::BestCase`].
    pub const fn hitresult_priority(mut self, priority: HitResultPriority) -> Self {
        self.hitresult_priority = priority;

        self
    }

    /// Whether the calculated attributes belong to an osu!lazer or osu!stable
    /// score.
    ///
    /// Defaults to `true`.
    ///
<<<<<<< HEAD
    /// This affects internal hitresult generation because lazer (without `CL`
    /// mod) gives two hitresults per hold note whereas stable only gives one.
    /// It also affect accuracy calculation because stable makes no difference
    /// between perfect (n320) and great (n300) hitresults but lazer (without
    /// `CL` mod) rewards slightly more for perfect hitresults.
=======
    /// This affects internal hitresult generation because lazer gives two
    /// hitresults per hold note whereas stable only gives one.
>>>>>>> 42db299 (meow)
    pub fn lazer(mut self, lazer: bool) -> Self {
        self.difficulty = self.difficulty.lazer(lazer);

        self
    }

    /// Specify the amount of 320s of a play.
    pub const fn n320(mut self, n320: u32) -> Self {
        self.n320 = Some(n320);

        self
    }

    /// Specify the amount of 300s of a play.
    pub const fn n300(mut self, n300: u32) -> Self {
        self.n300 = Some(n300);

        self
    }

    /// Specify the amount of 200s of a play.
    pub const fn n200(mut self, n200: u32) -> Self {
        self.n200 = Some(n200);

        self
    }

    /// Specify the amount of 100s of a play.
    pub const fn n100(mut self, n100: u32) -> Self {
        self.n100 = Some(n100);

        self
    }

    /// Specify the amount of 50s of a play.
    pub const fn n50(mut self, n50: u32) -> Self {
        self.n50 = Some(n50);

        self
    }

    /// Specify the amount of misses of a play.
    pub const fn misses(mut self, n_misses: u32) -> Self {
        self.misses = Some(n_misses);

        self
    }

    /// Provide parameters through an [`ManiaScoreState`].
    #[allow(clippy::needless_pass_by_value)]
    pub const fn state(mut self, state: ManiaScoreState) -> Self {
        let ManiaScoreState {
            n320,
            n300,
            n200,
            n100,
            n50,
            misses,
        } = state;

        self.n320 = Some(n320);
        self.n300 = Some(n300);
        self.n200 = Some(n200);
        self.n100 = Some(n100);
        self.n50 = Some(n50);
        self.misses = Some(misses);

        self
    }

    /// Create the [`ManiaScoreState`] that will be used for performance calculation.
    #[allow(clippy::too_many_lines, clippy::similar_names)]
    pub fn generate_state(&mut self) -> Result<ManiaScoreState, ConvertError> {
        let attrs = match self.map_or_attrs {
            MapOrAttrs::Map(ref map) => {
                let attrs = self.difficulty.calculate_for_mode::<Mania>(map)?;

                self.map_or_attrs.insert_attrs(attrs)
            }
            MapOrAttrs::Attrs(ref attrs) => attrs,
        };

<<<<<<< HEAD
        let priority = self.hitresult_priority;
        let mut n_objects = cmp::min(self.difficulty.get_passed_objects() as u32, attrs.n_objects);
        let misses = self.misses.map_or(0, |n| cmp::min(n, n_objects));
        let classic = !self.difficulty.get_lazer() || self.difficulty.get_mods().cl();

        if !classic {
=======
        let mut n_objects = cmp::min(self.difficulty.get_passed_objects() as u32, attrs.n_objects);

        let priority = self.hitresult_priority;

        let misses = self.misses.map_or(0, |n| cmp::min(n, n_objects));

        if self.difficulty.get_lazer() {
>>>>>>> 42db299 (meow)
            n_objects += attrs.n_hold_notes;
        }

        let n_remaining = n_objects - misses;

<<<<<<< HEAD
        let min_remaining = |n: u32| cmp::min(n, n_remaining);

        let mut n320 = self.n320.map_or(0, min_remaining);
        let mut n300 = self.n300.map_or(0, min_remaining);
        let mut n200 = self.n200.map_or(0, min_remaining);
        let mut n100 = self.n100.map_or(0, min_remaining);
        let mut n50 = self.n50.map_or(0, min_remaining);

        let generate_fast = |acc: f64| {
            let target = i32::max(
                0,
                f64::round(acc * f64::from(if classic { 60 } else { 61 } * n_objects)) as i32,
            ) as u32;

            let mut remaining_hits = n_remaining;
            let mut delta = target - 10 * remaining_hits;

            let perfect_factor = if classic { 50 } else { 51 };

            if let Some(n320) = self.n320 {
                delta = delta.saturating_sub(n320 * perfect_factor);
                remaining_hits = remaining_hits.saturating_sub(n320);
            }

            if let Some(n300) = self.n300 {
                delta = delta.saturating_sub(n300 * 50);
                remaining_hits = remaining_hits.saturating_sub(n300);
            }

            if let Some(n200) = self.n200 {
                delta = delta.saturating_sub(n200 * 30);
                remaining_hits = remaining_hits.saturating_sub(n200);
            }

            if let Some(n100) = self.n100 {
                delta = delta.saturating_sub(n100 * 10);
                remaining_hits = remaining_hits.saturating_sub(n100);
            }

            if let Some(n50) = self.n50 {
                // should `delta` be adjusted here? unsure
                remaining_hits = remaining_hits.saturating_sub(n50);
            }

            let mut perfects = if let Some(n320) = self.n320 {
                n320
            } else {
                let perfects = u32::min(delta / perfect_factor, remaining_hits);
                delta = delta.saturating_sub(perfects * perfect_factor);
                remaining_hits = remaining_hits.saturating_sub(perfects);

                perfects
            };

            let mut greats = if let Some(n300) = self.n300 {
                n300
            } else {
                let greats = u32::min(delta / 50, remaining_hits);
                delta = delta.saturating_sub(greats * 50);
                remaining_hits = remaining_hits.saturating_sub(greats);

                greats
            };

            let mut goods = if let Some(n200) = self.n200 {
                n200
            } else {
                let goods = u32::min(delta / 30, remaining_hits);
                delta = delta.saturating_sub(goods * 30);
                remaining_hits = remaining_hits.saturating_sub(goods);

                goods
            };

            let mut oks = if let Some(n100) = self.n100 {
                n100
            } else {
                let oks = u32::min(delta / 10, remaining_hits);
                remaining_hits = remaining_hits.saturating_sub(oks);

                oks
            };

            let mehs = if let Some(mut n50) = self.n50 {
                if remaining_hits > 0 {
                    if self.n100.is_none() {
                        oks += remaining_hits;
                    } else if self.n200.is_none() {
                        goods += remaining_hits;
                    } else if self.n300.is_none() {
                        greats += remaining_hits;
                    } else if self.n320.is_none() {
                        perfects += remaining_hits;
                    } else {
                        n50 += remaining_hits;
                    }
                }

                n50
            } else {
                remaining_hits
            };

            ManiaScoreState {
                n320: perfects,
                n300: greats,
                n200: goods,
                n100: oks,
                n50: mehs,
                misses,
            }
        };

        let generate_slow = |acc: f64| {
            let target = acc * f64::from(if classic { 60 } else { 61 } * n_objects);

            let mut best = ManiaScoreState {
                n320,
                n300,
                n200,
                n100,
                n50: n_remaining.saturating_sub(n320 + n300 + n200 + n100),
                misses,
            };

            let mut best_dist = f64::INFINITY;

            let remaining = n_remaining.saturating_sub(n300 + n200 + n100 + n50);

            let mut min_n320 = cmp::min(
                (if classic {
                    ((target - f64::from(40 * n_remaining) + f64::from(20 * n100 + 30 * n50))
                        / 20.0)
                        - f64::from(n300)
                } else {
                    target - f64::from(60 * n_remaining)
                        + f64::from(20 * n200 + 40 * n100 + 50 * n50)
                })
                .floor() as u32,
                remaining,
            );

            let mut max_n320 = cmp::min(
                ((target - f64::from(10 * n_remaining + 50 * n300 + 30 * n200 + 10 * n100))
                    / if classic { 50.0 } else { 51.0 })
                .ceil() as u32,
                remaining,
            );

            if let Some(n320) = self.n320 {
                min_n320 = min_remaining(n320);
                max_n320 = min_remaining(n320);
            }

            for n320 in min_n320..=max_n320 {
                let remaining = n_remaining.saturating_sub(n320 + n200 + n100 + n50);

                let mut min_n300 = cmp::min(
                    (if classic && self.n320.is_none() {
                        // n320 and n300 have the same value so we
                        // generate them all via n320 and shift them
                        // afterwards if necessary
                        0.0
                    } else {
                        let n320_weight = if classic { 20 } else { 21 };

                        (target - f64::from(40 * n_remaining + n320_weight * n320)
                            + f64::from(20 * n100 + 30 * n50))
                            / 20.0
                    })
                    .floor() as u32,
                    remaining,
                );

                let mut max_n300 = cmp::min(
                    (if classic && self.n320.is_none() {
                        0.0
                    } else {
                        let n320_weight = if classic { 50 } else { 51 };

                        (target
                            - f64::from(
                                10 * n_remaining + n320_weight * n320 + 30 * n200 + 10 * n100,
                            ))
                            / 50.0
                    })
                    .ceil() as u32,
                    remaining,
                );

                if let Some(n300) = self.n300 {
                    min_n300 = min_remaining(n300);
                    max_n300 = min_remaining(n300);
                }

                for n300 in min_n300..=max_n300 {
                    let remaining = n_remaining.saturating_sub(n320 + n300 + n100 + n50);

                    let n320_weight = if classic { 50 } else { 51 };

                    let mut min_n200 = cmp::min(
                        ((target - f64::from(20 * n_remaining + n320_weight * n320 + 50 * n300)
                            + f64::from(10 * n50))
                            / 30.0)
                            .floor() as u32,
                        remaining,
                    );

                    let mut max_n200 = cmp::min(
                        ((target
                            - f64::from(
                                10 * n_remaining + n320_weight * n320 + 50 * n300 + 10 * n100,
                            ))
                            / 30.0)
                            .ceil() as u32,
                        remaining,
                    );

                    if let Some(n200) = self.n200 {
                        min_n200 = min_remaining(n200);
                        max_n200 = min_remaining(n200);
                    }

                    for n200 in min_n200..=max_n200 {
                        let n100s = if let Some(n100) = self.n100 {
                            [min_remaining(n100), min_remaining(n100)]
                        } else {
                            let remaining = n_remaining.saturating_sub(n320 + n300 + n200 + n50);

                            let n100_raw = if self.n50.is_some() {
                                let n320_weight = if classic { 41 } else { 42 };

                                target
                                    - f64::from(
                                        19 * n_remaining
                                            + n320_weight * n320
                                            + 41 * n300
                                            + 21 * n200,
                                    )
                                    + f64::from(9 * n50)
                            } else {
                                let n320_weight = if classic { 50 } else { 51 };

                                (target
                                    - f64::from(
                                        10 * n_remaining
                                            + n320_weight * n320
                                            + 50 * n300
                                            + 30 * n200,
                                    ))
                                    / 10.0
                            };

                            let min = cmp::min(n100_raw.floor() as u32, remaining);
                            let max = cmp::min(n100_raw.ceil() as u32, remaining);

                            [min, max]
                        };

                        for n100 in n100s {
                            let n50 = if let Some(n50) = self.n50 {
                                min_remaining(n50)
                            } else {
                                n_remaining.saturating_sub(n320 + n300 + n200 + n100)
                            };

                            let mut curr = ManiaScoreState {
                                n320,
                                n300,
                                n200,
                                n100,
                                n50,
                                misses,
                            };

                            if curr.total_hits() < n_objects {
                                let remaining = n_objects - curr.total_hits();

                                match (self.n50, self.n100, self.n200, self.n300, self.n320) {
                                    (None, ..) => curr.n50 += remaining,
                                    (_, None, ..) => curr.n100 += remaining,
                                    (_, _, None, ..) => curr.n200 += remaining,
                                    (.., None, _) => curr.n300 += remaining,
                                    (.., None) => curr.n320 += remaining,
                                    _ => curr.n50 += remaining,
                                }
                            }

                            let curr_acc = curr.accuracy(classic);
                            let curr_dist = (acc - curr_acc).abs();

                            if curr_dist < best_dist {
                                best_dist = curr_dist;
                                best = curr;
                            }
                        }
                    }
                }
            }

            // Only n320 have an increased effect on performance
            // calculation so we adjust them based on priority
            if classic && self.n320.is_none() {
                // The logic below only operates on n320 and not n300
                // so we shift them here and let the logic below do
                // its thing
                if self.n300.is_none() {
                    best.n320 += best.n300;
                    best.n300 = 0;
                }

                match priority {
                    HitResultPriority::BestCase | HitResultPriority::Fastest => {
                        if self.n100.is_none() && self.n200.is_none() {
                            let n = best.n200 / 2;
                            best.n320 += n;
                            best.n200 -= 2 * n;
                            best.n100 += n;
                        }

                        if self.n50.is_none() && self.n200.is_none() {
                            let n = best.n200 / 5;
                            best.n320 += n * 3;
                            best.n200 -= n * 5;
                            best.n50 += n * 2;
                        }

                        if self.n300.is_none() {
                            best.n320 += best.n300;
                            best.n300 = 0;
                        }
                    }
                    HitResultPriority::WorstCase => {
                        if self.n100.is_none() && self.n200.is_none() {
                            let n = cmp::min(best.n320, best.n100);
                            best.n320 -= n;
                            best.n200 += 2 * n;
                            best.n100 -= n;
                        }

                        if self.n50.is_none() && self.n200.is_none() {
                            let n = cmp::min(best.n320 / 3, best.n50 / 2);
                            best.n320 -= n * 3;
                            best.n200 += n * 5;
                            best.n50 -= n * 2;
                        }

                        if self.n300.is_none() {
                            best.n300 += best.n320;
                            best.n320 = 0;
                        }
                    }
                }
            }

            best
        };

        if let Some(acc) = self.acc {
=======
        let mut n320 = self.n320.map_or(0, |n| cmp::min(n, n_remaining));
        let mut n300 = self.n300.map_or(0, |n| cmp::min(n, n_remaining));
        let mut n200 = self.n200.map_or(0, |n| cmp::min(n, n_remaining));
        let mut n100 = self.n100.map_or(0, |n| cmp::min(n, n_remaining));
        let mut n50 = self.n50.map_or(0, |n| cmp::min(n, n_remaining));

        if let Some(acc) = self.acc {
            let target_total = acc * f64::from(6 * n_objects);

>>>>>>> 42db299 (meow)
            match (self.n320, self.n300, self.n200, self.n100, self.n50) {
                // All hitresults given
                (Some(_), Some(_), Some(_), Some(_), Some(_)) => {
                    let remaining =
                        n_objects.saturating_sub(n320 + n300 + n200 + n100 + n50 + misses);

                    match priority {
<<<<<<< HEAD
                        HitResultPriority::BestCase | HitResultPriority::Fastest => {
                            n320 += remaining;
                        }
=======
                        HitResultPriority::BestCase => n320 += remaining,
>>>>>>> 42db299 (meow)
                        HitResultPriority::WorstCase => n50 += remaining,
                    }
                }

                // All but one hitresults given
<<<<<<< HEAD
                (None, Some(_), Some(_), Some(_), Some(_)) => n320 = n_remaining,
                (Some(_), None, Some(_), Some(_), Some(_)) => n300 = n_remaining,
                (Some(_), Some(_), None, Some(_), Some(_)) => n200 = n_remaining,
                (Some(_), Some(_), Some(_), None, Some(_)) => n100 = n_remaining,
                (Some(_), Some(_), Some(_), Some(_), None) => n50 = n_remaining,

                // At least two hitresults are unknown
                _ => {
                    let best = match priority {
                        HitResultPriority::Fastest => generate_fast(acc),
                        _ => generate_slow(acc),
                    };

                    n320 = best.n320;
                    n300 = best.n300;
                    n200 = best.n200;
                    n100 = best.n100;
                    n50 = best.n50;
                }
            }
        } else {
            let remaining = n_remaining.saturating_sub(n320 + n300 + n200 + n100 + n50);

            match priority {
                HitResultPriority::BestCase | HitResultPriority::Fastest => {
=======
                (None, Some(_), Some(_), Some(_), Some(_)) => {
                    n320 = n_objects.saturating_sub(n300 + n200 + n100 + n50 + misses);
                }
                (Some(_), None, Some(_), Some(_), Some(_)) => {
                    n300 = n_objects.saturating_sub(n320 + n200 + n100 + n50 + misses);
                }
                (Some(_), Some(_), None, Some(_), Some(_)) => {
                    n200 = n_objects.saturating_sub(n320 + n300 + n100 + n50 + misses);
                }
                (Some(_), Some(_), Some(_), None, Some(_)) => {
                    n100 = n_objects.saturating_sub(n320 + n300 + n200 + n50 + misses);
                }
                (Some(_), Some(_), Some(_), Some(_), None) => {
                    n50 = n_objects.saturating_sub(n320 + n300 + n200 + n100 + misses);
                }

                // n200, n100, and n50 given
                (None, None, Some(_), Some(_), Some(_)) => {
                    let n_remaining =
                        n_objects.saturating_sub(n320 + n300 + n200 + n100 + n50 + misses);

                    match priority {
                        HitResultPriority::BestCase => n320 = n_remaining,
                        HitResultPriority::WorstCase => n300 = n_remaining,
                    }
                }

                // n100 and n50 given
                (.., None, Some(_), Some(_)) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n100 + n50 + misses);

                    let raw_n3x0 = (target_total - f64::from(4 * n_remaining)
                        + f64::from(2 * n100 + 3 * n50))
                        / 2.0;
                    let min_n3x0 = cmp::min(
                        raw_n3x0.floor() as u32,
                        n_remaining.saturating_sub(n100 + n50),
                    );
                    let max_n3x0 = cmp::min(
                        raw_n3x0.ceil() as u32,
                        n_remaining.saturating_sub(n100 + n50),
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (n320 + n300, n320 + n300),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let new200 = n_remaining.saturating_sub(new3x0 + n100 + n50);
                        let curr_dist =
                            (acc - accuracy(new3x0, 0, new200, n100, n50, misses)).abs();

                        if curr_dist < best_dist {
                            best_dist = curr_dist;
                            n3x0 = new3x0;
                            n200 = new200;
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }
                }

                // n200 and n50 given
                (.., Some(_), None, Some(_)) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n200 + n50 + misses);

                    let raw_n3x0 = (target_total - f64::from(2 * (n_remaining + n200) - n50)) / 4.0;
                    let min_n3x0 = cmp::min(
                        raw_n3x0.floor() as u32,
                        n_remaining.saturating_sub(n200 + n50),
                    );
                    let max_n3x0 = cmp::min(
                        raw_n3x0.ceil() as u32,
                        n_remaining.saturating_sub(n200 + n50),
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (n320 + n300, n320 + n300),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let new100 = n_remaining.saturating_sub(new3x0 + n200 + n50);
                        let curr_dist =
                            (acc - accuracy(new3x0, 0, n200, new100, n50, misses)).abs();

                        if curr_dist < best_dist {
                            best_dist = curr_dist;
                            n3x0 = new3x0;
                            n100 = new100;
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }
                }

                // n200 and n100 given
                (.., Some(_), Some(_), None) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n200 + n100 + misses);

                    let raw_n3x0 = (target_total - f64::from(n_remaining + 3 * n200 + n100)) / 5.0;
                    let min_n3x0 = cmp::min(
                        raw_n3x0.floor() as u32,
                        n_remaining.saturating_sub(n200 + n100),
                    );
                    let max_n3x0 = cmp::min(
                        raw_n3x0.ceil() as u32,
                        n_remaining.saturating_sub(n200 + n100),
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (n320 + n300, n320 + n300),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let new50 = n_remaining.saturating_sub(new3x0 + n200 + n100);
                        let curr_dist =
                            (acc - accuracy(new3x0, 0, n200, n100, new50, misses)).abs();

                        if curr_dist < best_dist {
                            best_dist = curr_dist;
                            n3x0 = new3x0;
                            n50 = new50;
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }
                }

                // n200 given
                (.., Some(_), None, None) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n200 + misses);

                    let min_n3x0 = cmp::min(
                        ((target_total - f64::from(2 * (n_remaining + n200))) / 4.0).floor() as u32,
                        n_remaining - n200,
                    );

                    let max_n3x0 = cmp::min(
                        ((target_total - f64::from(n_remaining + 3 * n200)) / 5.0).ceil() as u32,
                        n_remaining - n200,
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (
                            cmp::min(n_remaining, n320 + n300),
                            cmp::min(n_remaining, n320 + n300),
                        ),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let raw_n100 =
                            target_total - f64::from(n_remaining + 5 * new3x0 + 3 * n200);
                        let min_n100 = cmp::min(
                            raw_n100.floor() as u32,
                            n_remaining.saturating_sub(new3x0 + n200),
                        );
                        let max_n100 = cmp::min(
                            raw_n100.ceil() as u32,
                            n_remaining.saturating_sub(new3x0 + n200),
                        );

                        for new100 in min_n100..=max_n100 {
                            let new50 = n_remaining.saturating_sub(new3x0 + n200 + new100);
                            let curr_dist =
                                (acc - accuracy(new3x0, 0, n200, new100, new50, misses)).abs();

                            if curr_dist < best_dist {
                                best_dist = curr_dist;
                                n3x0 = new3x0;
                                n100 = new100;
                                n50 = new50;
                            }
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }
                }

                // n100 given
                (.., None, Some(_), None) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n100 + misses);

                    let min_n3x0 = cmp::min(
                        (acc * f64::from(3 * n_remaining) - f64::from(2 * n_remaining - n100))
                            .floor() as u32,
                        n_remaining - n100,
                    );

                    let max_n3x0 = cmp::min(
                        ((target_total - f64::from(n_remaining + n100)) / 5.0).ceil() as u32,
                        n_remaining - n100,
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (
                            cmp::min(n_remaining, n320 + n300),
                            cmp::min(n_remaining, n320 + n300),
                        ),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let raw_n200 =
                            (target_total - f64::from(n_remaining + 5 * new3x0 + n100)) / 3.0;
                        let min_n200 = cmp::min(
                            raw_n200.floor() as u32,
                            n_remaining.saturating_sub(new3x0 + n100),
                        );
                        let max_n200 = cmp::min(
                            raw_n200.ceil() as u32,
                            n_remaining.saturating_sub(new3x0 + n100),
                        );

                        for new200 in min_n200..=max_n200 {
                            let new50 = n_remaining.saturating_sub(new3x0 + new200 + n100);
                            let curr_dist =
                                (acc - accuracy(new3x0, 0, new200, n100, new50, misses)).abs();

                            if curr_dist < best_dist {
                                best_dist = curr_dist;
                                n3x0 = new3x0;
                                n200 = new200;
                                n50 = new50;
                            }
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }
                }

                // n50 given
                (.., None, None, Some(_)) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n50 + misses);

                    let min_n3x0 = cmp::min(
                        ((target_total - f64::from(4 * n_remaining - 3 * n50)) / 2.0).floor()
                            as u32,
                        n_remaining - n50,
                    );

                    let max_n3x0 = cmp::min(
                        ((target_total - f64::from(2 * n_remaining - n50)) / 4.0).ceil() as u32,
                        n_remaining - n50,
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (
                            cmp::min(n_remaining, n320 + n300),
                            cmp::min(n_remaining, n320 + n300),
                        ),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let raw_n200 = (target_total - f64::from(2 * n_remaining + 4 * new3x0)
                            + f64::from(n50))
                            / 2.0;
                        let min_n200 = cmp::min(
                            raw_n200.floor() as u32,
                            n_remaining.saturating_sub(new3x0 + n50),
                        );
                        let max_n200 = cmp::min(
                            raw_n200.ceil() as u32,
                            n_remaining.saturating_sub(new3x0 + n50),
                        );

                        for new200 in min_n200..=max_n200 {
                            let new100 = n_remaining.saturating_sub(new3x0 + new200 + n50);
                            let curr_dist =
                                (acc - accuracy(new3x0, 0, new200, new100, n50, misses)).abs();

                            if curr_dist < best_dist {
                                best_dist = curr_dist;
                                n3x0 = new3x0;
                                n200 = new200;
                                n100 = new100;
                            }
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }

                    if self.n320.is_none() {
                        if let HitResultPriority::BestCase = priority {
                            // Distribute n200 onto n320 and n100
                            let n = n200 / 2;
                            n320 += n;
                            n200 -= 2 * n;
                            n100 += n;
                        }
                    }
                }

                // Neither n200, n100, nor n50 given
                (.., None, None, None) => {
                    let mut best_dist = f64::INFINITY;
                    let mut n3x0 = n_objects.saturating_sub(n320 + n300 + n200 + n100 + misses);

                    let min_n3x0 = cmp::min(
                        ((target_total - f64::from(4 * n_remaining)) / 5.0).floor() as u32,
                        n_remaining,
                    );

                    let max_n3x0 = cmp::min(
                        ((target_total - f64::from(n_remaining)) / 5.0)
                            .min(acc * f64::from(3 * n_objects) - f64::from(n_remaining))
                            .ceil() as u32,
                        n_remaining,
                    );

                    let (min_n3x0, max_n3x0) = match (self.n320, self.n300) {
                        (Some(_), Some(_)) => (
                            cmp::min(n_remaining, n320 + n300),
                            cmp::min(n_remaining, n320 + n300),
                        ),
                        (Some(_), None) => (cmp::max(min_n3x0, n320), cmp::max(max_n3x0, n320)),
                        (None, Some(_)) => (cmp::max(min_n3x0, n300), cmp::max(max_n3x0, n300)),
                        (None, None) => (min_n3x0, max_n3x0),
                    };

                    for new3x0 in min_n3x0..=max_n3x0 {
                        let min_n200 = cmp::min(
                            (acc * f64::from(3 * n_objects) - f64::from(n_remaining + 2 * new3x0))
                                .floor() as u32,
                            n_remaining - new3x0,
                        );

                        let max_n200 = cmp::min(
                            ((target_total - f64::from(n_remaining + 5 * new3x0)) / 3.0).ceil()
                                as u32,
                            n_remaining - new3x0,
                        );

                        for new200 in min_n200..=max_n200 {
                            let raw_n100 =
                                target_total - f64::from(n_remaining + 5 * new3x0 + 3 * new200);
                            let min_n100 =
                                cmp::min(raw_n100.floor() as u32, n_remaining - (new3x0 + new200));
                            let max_n100 =
                                cmp::min(raw_n100.ceil() as u32, n_remaining - (new3x0 + new200));

                            for new100 in min_n100..=max_n100 {
                                let new50 = n_remaining - new3x0 - new200 - new100;
                                let curr_acc = accuracy(new3x0, 0, new200, new100, new50, misses);
                                let curr_dist = (acc - curr_acc).abs();

                                if curr_dist < best_dist {
                                    best_dist = curr_dist;
                                    n3x0 = new3x0;
                                    n200 = new200;
                                    n100 = new100;
                                    n50 = new50;
                                }
                            }
                        }
                    }

                    match (self.n320, self.n300) {
                        (None, None) => match priority {
                            HitResultPriority::BestCase => n320 = n3x0,
                            HitResultPriority::WorstCase => n300 = n3x0,
                        },
                        (Some(_), None) => n300 = n3x0 - n320,
                        (None, Some(_)) => n320 = n3x0 - n300,
                        _ => {}
                    }

                    if self.n320.is_none() {
                        if let HitResultPriority::BestCase = priority {
                            // Distribute n200 onto n320 and n100
                            let n = n200 / 2;
                            n320 += n;
                            n200 -= 2 * n;
                            n100 += n;
                        }
                    }
                }
            }
        } else {
            let remaining = n_objects.saturating_sub(n320 + n300 + n200 + n100 + n50 + misses);

            match priority {
                HitResultPriority::BestCase => {
>>>>>>> 42db299 (meow)
                    match (self.n320, self.n300, self.n200, self.n100, self.n50) {
                        (None, ..) => n320 = remaining,
                        (_, None, ..) => n300 = remaining,
                        (_, _, None, ..) => n200 = remaining,
                        (.., None, _) => n100 = remaining,
                        (.., None) => n50 = remaining,
                        _ => n320 += remaining,
                    }
                }
                HitResultPriority::WorstCase => {
                    match (self.n50, self.n100, self.n200, self.n300, self.n320) {
                        (None, ..) => n50 = remaining,
                        (_, None, ..) => n100 = remaining,
                        (_, _, None, ..) => n200 = remaining,
                        (.., None, _) => n300 = remaining,
                        (.., None) => n320 = remaining,
                        _ => n50 += remaining,
                    }
                }
            }
        }

        self.n320 = Some(n320);
        self.n300 = Some(n300);
        self.n200 = Some(n200);
        self.n100 = Some(n100);
        self.n50 = Some(n50);
        self.misses = Some(misses);

        Ok(ManiaScoreState {
            n320,
            n300,
            n200,
            n100,
            n50,
            misses,
        })
    }

    /// Calculate all performance related values, including pp and stars.
    pub fn calculate(mut self) -> Result<ManiaPerformanceAttributes, ConvertError> {
        let state = self.generate_state()?;

        let attrs = match self.map_or_attrs {
            MapOrAttrs::Attrs(attrs) => attrs,
            MapOrAttrs::Map(ref map) => self.difficulty.calculate_for_mode::<Mania>(map)?,
        };

<<<<<<< HEAD
        Ok(ManiaPerformanceCalculator::new(attrs, self.difficulty.get_mods(), state).calculate())
=======
        let inner = ManiaPerformanceInner {
            mods: self.difficulty.get_mods(),
            attrs,
            state,
        };

        Ok(inner.calculate())
>>>>>>> 42db299 (meow)
    }

    pub(crate) const fn from_map_or_attrs(map_or_attrs: MapOrAttrs<'map, Mania>) -> Self {
        Self {
            map_or_attrs,
            difficulty: Difficulty::new(),
            n320: None,
            n300: None,
            n200: None,
            n100: None,
            n50: None,
            misses: None,
            acc: None,
            hitresult_priority: HitResultPriority::DEFAULT,
        }
    }
}

impl<'map> TryFrom<OsuPerformance<'map>> for ManiaPerformance<'map> {
    type Error = OsuPerformance<'map>;

    /// Try to create [`ManiaPerformance`] through [`OsuPerformance`].
    ///
    /// Returns `None` if [`OsuPerformance`] does not contain a beatmap, i.e.
    /// if it was constructed through attributes or
    /// [`OsuPerformance::generate_state`] was called.
    fn try_from(mut osu: OsuPerformance<'map>) -> Result<Self, Self::Error> {
        let mods = osu.difficulty.get_mods();

        let map = match OsuPerformance::try_convert_map(osu.map_or_attrs, GameMode::Mania, mods) {
            Ok(map) => map,
            Err(map_or_attrs) => {
                osu.map_or_attrs = map_or_attrs;

                return Err(osu);
            }
        };

        let OsuPerformance {
            map_or_attrs: _,
            difficulty,
            acc,
            combo: _,
            large_tick_hits: _,
<<<<<<< HEAD
            small_tick_hits: _,
=======
>>>>>>> 42db299 (meow)
            slider_end_hits: _,
            n300,
            n100,
            n50,
            misses,
            hitresult_priority,
        } = osu;

        Ok(Self {
            map_or_attrs: MapOrAttrs::Map(map),
            difficulty,
            n320: None,
            n300,
            n200: None,
            n100,
            n50,
            misses,
            acc,
            hitresult_priority,
        })
    }
}

impl<'map, T: IntoModePerformance<'map, Mania>> From<T> for ManiaPerformance<'map> {
    fn from(into: T) -> Self {
        into.into_performance()
    }
}

<<<<<<< HEAD
#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, sync::OnceLock, time::Instant};

    use proptest::{
        prelude::*,
        test_runner::{RngAlgorithm, TestRng},
    };
    use rosu_map::section::general::GameMode;
    use rosu_mods::GameMod;

    use crate::{
        any::{DifficultyAttributes, PerformanceAttributes},
        mania::ManiaDifficultyAttributes,
=======
struct ManiaPerformanceInner<'mods> {
    attrs: ManiaDifficultyAttributes,
    mods: &'mods GameMods,
    state: ManiaScoreState,
}

impl ManiaPerformanceInner<'_> {
    fn calculate(self) -> ManiaPerformanceAttributes {
        let mut multiplier = 1.0;

        if self.mods.nf() {
            multiplier *= 0.75;
        }

        if self.mods.ez() {
            multiplier *= 0.5;
        }

        let difficulty_value = self.compute_difficulty_value();
        let pp = difficulty_value * multiplier;

        ManiaPerformanceAttributes {
            difficulty: self.attrs,
            pp,
            pp_difficulty: difficulty_value,
        }
    }

    fn compute_difficulty_value(&self) -> f64 {
        // * Star rating to pp curve
        8.0 * (self.attrs.stars - 0.15).max(0.05).powf(2.2)
             // * From 80% accuracy, 1/20th of total pp is awarded per additional 1% accuracy
             * (5.0 * self.calculate_custom_accuracy() - 4.0).max(0.0)
             // * Length bonus, capped at 1500 notes
             * (1.0 + 0.1 * (self.total_hits() / 1500.0).min(1.0))
    }

    const fn total_hits(&self) -> f64 {
        self.state.total_hits() as f64
    }

    fn calculate_custom_accuracy(&self) -> f64 {
        let ManiaScoreState {
            n320,
            n300,
            n200,
            n100,
            n50,
            misses: _,
        } = &self.state;

        let total_hits = self.state.total_hits();

        if total_hits == 0 {
            return 0.0;
        }

        custom_accuracy(*n320, *n300, *n200, *n100, *n50, total_hits)
    }
}

fn custom_accuracy(n320: u32, n300: u32, n200: u32, n100: u32, n50: u32, total_hits: u32) -> f64 {
    let numerator = n320 * 32 + n300 * 30 + n200 * 20 + n100 * 10 + n50 * 5;
    let denominator = total_hits * 32;

    f64::from(numerator) / f64::from(denominator)
}

fn accuracy(n320: u32, n300: u32, n200: u32, n100: u32, n50: u32, misses: u32) -> f64 {
    let numerator = 6 * (n320 + n300) + 4 * n200 + 2 * n100 + n50;
    let denominator = 6 * (n320 + n300 + n200 + n100 + n50 + misses);

    f64::from(numerator) / f64::from(denominator)
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, sync::OnceLock};

    use proptest::prelude::*;
    use rosu_map::section::general::GameMode;

    use crate::{
        any::{DifficultyAttributes, PerformanceAttributes},
>>>>>>> 42db299 (meow)
        osu::{OsuDifficultyAttributes, OsuPerformanceAttributes},
        Beatmap,
    };

<<<<<<< HEAD
    use super::{calculator::custom_accuracy, *};
=======
    use super::*;
>>>>>>> 42db299 (meow)

    static ATTRS: OnceLock<ManiaDifficultyAttributes> = OnceLock::new();

    const N_OBJECTS: u32 = 594;
    const N_HOLD_NOTES: u32 = 121;

    fn beatmap() -> Beatmap {
        Beatmap::from_path("./resources/1638954.osu").unwrap()
    }

    fn attrs() -> ManiaDifficultyAttributes {
        ATTRS
            .get_or_init(|| {
                let map = beatmap();
                let attrs = Difficulty::new().calculate_for_mode::<Mania>(&map).unwrap();

                assert_eq!(N_OBJECTS, map.hit_objects.len() as u32);
                assert_eq!(
                    N_HOLD_NOTES,
                    map.hit_objects.iter().filter(|h| !h.is_circle()).count() as u32
                );

                attrs
            })
            .to_owned()
    }

<<<<<<< HEAD
    /// Creates a [`rosu_mods::GameMods`] instance and inserts `CL` if `classic`
    /// is true.
    fn mods(classic: bool) -> rosu_mods::GameMods {
        if classic {
            let mut mods = rosu_mods::GameMods::new();
            mods.insert(GameMod::ClassicMania(Default::default()));

            mods
        } else {
            rosu_mods::GameMods::new()
        }
    }

=======
>>>>>>> 42db299 (meow)
    /// Checks most remaining hitresult combinations w.r.t. the given parameters
    /// and returns the [`ManiaScoreState`] that matches `acc` the best.
    ///
    /// Very slow but accurate. Only slight optimizations have been applied so
    /// that it doesn't run unreasonably long.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn brute_force_best(
<<<<<<< HEAD
        classic: bool,
=======
        lazer: bool,
>>>>>>> 42db299 (meow)
        acc: f64,
        n320: Option<u32>,
        n300: Option<u32>,
        n200: Option<u32>,
        n100: Option<u32>,
        n50: Option<u32>,
        misses: u32,
        best_case: bool,
    ) -> ManiaScoreState {
        let misses = cmp::min(misses, N_OBJECTS);

        let mut best_state = ManiaScoreState {
            misses,
            ..Default::default()
        };

        let mut best_dist = f64::INFINITY;
        let mut best_custom_acc = 0.0;

<<<<<<< HEAD
=======
        let mut n_remaining = N_OBJECTS - misses;

        if lazer {
            n_remaining += N_HOLD_NOTES;
        }

>>>>>>> 42db299 (meow)
        let multiple_given = (usize::from(n320.is_some())
            + usize::from(n300.is_some())
            + usize::from(n200.is_some())
            + usize::from(n100.is_some())
            + usize::from(n50.is_some()))
            > 1;

        let mut n_objects = N_OBJECTS;

<<<<<<< HEAD
        if !classic {
            n_objects += N_HOLD_NOTES;
        }

        let n_remaining = n_objects - misses;

        let target = acc * f64::from(if classic { 60 } else { 61 } * n_objects);

        let max_left = n_objects.saturating_sub(
            if classic { 0 } else { n300.unwrap_or(0) }
                + n200.unwrap_or(0)
                + n100.unwrap_or(0)
                + n50.unwrap_or(0)
                + misses,
        );

        let min_n320 = cmp::min(
            max_left,
            if classic {
                (target - f64::from(40 * n_remaining)) / 20.0
            } else {
                target - f64::from(60 * n_remaining)
            }
            .floor() as u32,
        );

        let max_n320 = cmp::min(
            max_left,
            ((target - f64::from(10 * n_remaining)) / if classic { 50.0 } else { 51.0 }).ceil()
                as u32,
        );

        let (min_n320, max_n320) = match (n320, n300) {
            (Some(n320), _) if !classic => {
                (cmp::min(n_remaining, n320), cmp::min(n_remaining, n320))
            }
            (None, _) if !classic => (min_n320, max_n320),
=======
        if lazer {
            n_objects += N_HOLD_NOTES;
        }

        let max_left = n_objects
            .saturating_sub(n200.unwrap_or(0) + n100.unwrap_or(0) + n50.unwrap_or(0) + misses);

        let min_n3x0 = cmp::min(
            max_left,
            (acc * f64::from(3 * n_objects) - f64::from(2 * n_remaining)).floor() as u32,
        );

        let max_n3x0 = cmp::min(
            max_left,
            ((acc * f64::from(6 * n_objects) - f64::from(n_remaining)) / 5.0).ceil() as u32,
        );

        let (min_n3x0, max_n3x0) = match (n320, n300) {
>>>>>>> 42db299 (meow)
            (Some(n320), Some(n300)) => (
                cmp::min(n_remaining, n320 + n300),
                cmp::min(n_remaining, n320 + n300),
            ),
            (Some(n320), None) => (
<<<<<<< HEAD
                cmp::max(cmp::min(n_remaining, n320), min_n320),
                cmp::max(max_n320, cmp::min(n320, n_remaining)),
            ),
            (None, Some(n300)) => (
                cmp::max(cmp::min(n_remaining, n300), min_n320),
                cmp::max(max_n320, cmp::min(n300, n_remaining)),
            ),
            (None, None) => (min_n320, max_n320),
        };

        let mut n300_iters = 0;
        let mut n300_skips = 0;

        for new320 in min_n320..=max_n320 {
            let max_left = n_remaining
                .saturating_sub(new320 + n200.unwrap_or(0) + n100.unwrap_or(0) + n50.unwrap_or(0));

            let (min_n300, max_n300) = match n300 {
                _ if classic => (0, 0),
                Some(n300) if multiple_given => {
                    (cmp::min(n_remaining, n300), cmp::min(n_remaining, n300))
                }
                Some(n300) => (cmp::min(max_left, n300), cmp::min(max_left, n300)),
                None if n200.and(n100).and(n50).is_some() => (max_left, max_left),
                None => (0, max_left),
            };

            for new300 in min_n300..=max_n300 {
                let max_left = n_remaining
                    .saturating_sub(new320 + new300 + n100.unwrap_or(0) + n50.unwrap_or(0));

                let min_state = {
                    let n50 = n50.unwrap_or(max_left);
                    let n100 = n100.unwrap_or(
                        n_remaining.saturating_sub(new320 + new300 + n200.unwrap_or(0) + n50),
                    );
                    let n200 =
                        n200.unwrap_or(n_remaining.saturating_sub(new320 + new300 + n100 + n50));

                    ManiaScoreState {
                        n320: new320,
                        n300: new300,
                        n200,
                        n100,
                        n50,
                        misses,
                    }
                };

                let max_state = {
                    let n200 = n200.unwrap_or(max_left);
                    let n100 = n100.unwrap_or(
                        n_remaining.saturating_sub(new320 + new300 + n200 + n50.unwrap_or(0)),
                    );
                    let n50 =
                        n50.unwrap_or(n_remaining.saturating_sub(new320 + new300 + n200 + n100));

                    ManiaScoreState {
                        n320: new320,
                        n300: new300,
                        n200,
                        n100,
                        n50,
                        misses,
                    }
                };

                n300_iters += 1;

                // Skip n200 and n100 iterations if we know we won't be able to
                // get a better result.
                if min_state.accuracy(classic) - best_dist > acc
                    || max_state.accuracy(classic) + best_dist < acc
                {
                    n300_skips += 1;

                    continue;
                }

                let (min_n200, max_n200) = match (n200, n100, n50) {
                    (Some(n200), ..) if multiple_given => {
                        (cmp::min(n_remaining, n200), cmp::min(n_remaining, n200))
                    }
                    (Some(n200), ..) => (cmp::min(max_left, n200), cmp::min(max_left, n200)),
                    (None, Some(_), Some(_)) => (max_left, max_left),
                    _ => (0, max_left),
                };

                for new200 in min_n200..=max_n200 {
                    let max_left =
                        n_remaining.saturating_sub(new320 + new300 + new200 + n50.unwrap_or(0));

                    let (min_n100, max_n100) = match (n100, n50) {
                        (Some(n100), _) if multiple_given => {
                            (cmp::min(n_remaining, n100), cmp::min(n_remaining, n100))
                        }
                        (Some(n100), _) => (cmp::min(max_left, n100), cmp::min(max_left, n100)),
                        (None, Some(_)) => (max_left, max_left),
                        (None, None) => (0, max_left),
                    };

                    for new100 in min_n100..=max_n100 {
                        let max_left =
                            n_remaining.saturating_sub(new320 + new300 + new200 + new100);

                        let new50 = match n50 {
                            Some(n50) if multiple_given => cmp::min(n_remaining, n50),
                            Some(n50) => cmp::min(max_left, n50),
                            None => max_left,
                        };

                        let (new320, new300) = if classic {
                            match (n320, n300) {
                                (Some(n320), Some(n300)) => {
                                    (cmp::min(n_remaining, n320), cmp::min(n_remaining, n300))
                                }
                                (Some(n320), None) => (
                                    cmp::min(n320, n_remaining),
                                    new320 - cmp::min(n320, n_remaining),
                                ),
                                (None, Some(n300)) => (
                                    new320 - cmp::min(n300, n_remaining),
                                    cmp::min(n300, n_remaining),
                                ),
                                (None, None) if best_case => (new320, 0),
                                (None, None) => (0, new320),
                            }
                        } else {
                            (new320, new300)
                        };

                        let curr_acc = ManiaScoreState {
                            n320: new320,
                            n300: new300,
                            n200: new200,
                            n100: new100,
                            n50: new50,
                            misses,
                        }
                        .accuracy(classic);

                        let curr_dist = (acc - curr_acc).abs();

                        let curr_custom_acc =
                            custom_accuracy(new320, new300, new200, new100, new50, n_objects);

                        match curr_dist.total_cmp(&best_dist) {
                            Ordering::Less => {
                                best_dist = curr_dist;
                                best_custom_acc = curr_custom_acc;
                                best_state.n320 = new320;
                                best_state.n300 = new300;
                                best_state.n200 = new200;
                                best_state.n100 = new100;
                                best_state.n50 = new50;
                            }
                            Ordering::Equal if curr_custom_acc < best_custom_acc => {
                                best_custom_acc = curr_custom_acc;
                                best_state.n320 = new320;
                                best_state.n300 = new300;
                                best_state.n200 = new200;
                                best_state.n100 = new100;
                                best_state.n50 = new50;
                            }
                            _ => {}
                        }
=======
                cmp::max(cmp::min(n_remaining, n320), min_n3x0),
                cmp::max(max_n3x0, cmp::min(n320, n_remaining)),
            ),
            (None, Some(n300)) => (
                cmp::max(cmp::min(n_remaining, n300), min_n3x0),
                cmp::max(max_n3x0, cmp::min(n300, n_remaining)),
            ),
            (None, None) => (min_n3x0, max_n3x0),
        };

        for new3x0 in min_n3x0..=max_n3x0 {
            let max_left =
                n_remaining.saturating_sub(new3x0 + n100.unwrap_or(0) + n50.unwrap_or(0));

            let (min_n200, max_n200) = match (n200, n100, n50) {
                (Some(n200), ..) if multiple_given => {
                    (cmp::min(n_remaining, n200), cmp::min(n_remaining, n200))
                }
                (Some(n200), ..) => (cmp::min(max_left, n200), cmp::min(max_left, n200)),
                (None, Some(_), Some(_)) => (max_left, max_left),
                _ => (0, max_left),
            };

            for new200 in min_n200..=max_n200 {
                let max_left = n_remaining.saturating_sub(new3x0 + new200 + n50.unwrap_or(0));

                let (min_n100, max_n100) = match (n100, n50) {
                    (Some(n100), _) if multiple_given => {
                        (cmp::min(n_remaining, n100), cmp::min(n_remaining, n100))
                    }
                    (Some(n100), _) => (cmp::min(max_left, n100), cmp::min(max_left, n100)),
                    (None, Some(_)) => (max_left, max_left),
                    (None, None) => (0, max_left),
                };

                for new100 in min_n100..=max_n100 {
                    let max_left = n_remaining.saturating_sub(new3x0 + new200 + new100);

                    let new50 = match n50 {
                        Some(n50) if multiple_given => cmp::min(n_remaining, n50),
                        Some(n50) => cmp::min(max_left, n50),
                        None => max_left,
                    };

                    let (new320, new300) = match (n320, n300) {
                        (Some(n320), Some(n300)) => {
                            (cmp::min(n_remaining, n320), cmp::min(n_remaining, n300))
                        }
                        (Some(n320), None) => (
                            cmp::min(n320, n_remaining),
                            new3x0 - cmp::min(n320, n_remaining),
                        ),
                        (None, Some(n300)) => (
                            new3x0 - cmp::min(n300, n_remaining),
                            cmp::min(n300, n_remaining),
                        ),
                        (None, None) if best_case => (new3x0, 0),
                        (None, None) => (0, new3x0),
                    };

                    let curr_acc = accuracy(new320, new300, new200, new100, new50, misses);
                    let curr_dist = (acc - curr_acc).abs();

                    let curr_custom_acc =
                        custom_accuracy(new320, new300, new200, new100, new50, n_objects);

                    match curr_dist.total_cmp(&best_dist) {
                        Ordering::Less => {
                            best_dist = curr_dist;
                            best_custom_acc = curr_custom_acc;
                            best_state.n320 = new320;
                            best_state.n300 = new300;
                            best_state.n200 = new200;
                            best_state.n100 = new100;
                            best_state.n50 = new50;
                        }
                        Ordering::Equal if curr_custom_acc < best_custom_acc => {
                            best_custom_acc = curr_custom_acc;
                            best_state.n320 = new320;
                            best_state.n300 = new300;
                            best_state.n200 = new200;
                            best_state.n100 = new100;
                            best_state.n50 = new50;
                        }
                        _ => {}
>>>>>>> 42db299 (meow)
                    }
                }
            }
        }

<<<<<<< HEAD
        eprintln!("Bruteforce skipped {n300_skips}/{n300_iters} n300 iterations");

=======
>>>>>>> 42db299 (meow)
        if best_state.n320 + best_state.n300 + best_state.n200 + best_state.n100 + best_state.n50
            < n_remaining
        {
            let n_remaining = n_remaining
                - (best_state.n320
                    + best_state.n300
                    + best_state.n200
                    + best_state.n100
                    + best_state.n50);

            if best_case {
                match (n320, n300, n200, n100, n50) {
                    (None, ..) => best_state.n320 += n_remaining,
                    (_, None, ..) => best_state.n300 += n_remaining,
                    (_, _, None, ..) => best_state.n200 += n_remaining,
                    (.., None, _) => best_state.n100 += n_remaining,
                    (.., None) => best_state.n50 += n_remaining,
                    _ => best_state.n320 += n_remaining,
                }
            } else {
                match (n50, n100, n200, n300, n320) {
                    (None, ..) => best_state.n50 += n_remaining,
                    (_, None, ..) => best_state.n100 += n_remaining,
                    (_, _, None, ..) => best_state.n200 += n_remaining,
                    (.., None, _) => best_state.n300 += n_remaining,
                    (.., None) => best_state.n320 += n_remaining,
                    _ => best_state.n50 += n_remaining,
                }
            }
        }

<<<<<<< HEAD
        if classic && n320.is_none() {
            let before = best_state.clone();

            if n300.is_none() {
                best_state.n320 += best_state.n300;
                best_state.n300 = 0;
            }

            if best_case {
                if n100.is_none() && n200.is_none() {
                    let n = best_state.n200 / 2;
                    best_state.n320 += n;
                    best_state.n200 -= 2 * n;
                    best_state.n100 += n;
                }

                if n50.is_none() && n200.is_none() {
                    let n = best_state.n200 / 5;
                    best_state.n320 += n * 3;
                    best_state.n200 -= n * 5;
                    best_state.n50 += n * 2;
                }

                if n300.is_none() {
                    best_state.n320 += best_state.n300;
                    best_state.n300 = 0;
                }
            } else {
                if n100.is_none() && n200.is_none() {
                    let n = cmp::min(best_state.n320, best_state.n100);
                    best_state.n320 -= n;
                    best_state.n200 += 2 * n;
                    best_state.n100 -= n;
                }

                if n50.is_none() && n200.is_none() {
                    let n = cmp::min(best_state.n320 / 3, best_state.n50 / 2);
                    best_state.n320 -= n * 3;
                    best_state.n200 += n * 5;
                    best_state.n50 -= n * 2;
                }

                if n300.is_none() {
                    best_state.n300 += best_state.n320;
                    best_state.n320 = 0;
                }
            }

            assert_eq!(best_state.accuracy(classic), before.accuracy(classic));
=======
        if best_case {
            if n320.is_none() && n200.is_none() && n100.is_none() {
                let n = best_state.n200 / 2;
                best_state.n320 += n;
                best_state.n200 -= 2 * n;
                best_state.n100 += n;
            }

            if n100.is_none() && n50.is_none() {
                let n = if n320.is_none() && n300.is_none() {
                    let n = cmp::min(best_state.n320 + best_state.n300, best_state.n50 / 4);

                    let removed320 = cmp::min(best_state.n320, n);
                    let removed300 = n - removed320;

                    best_state.n320 -= removed320;
                    best_state.n300 -= removed300;

                    n
                } else if n320.is_none() {
                    let n = cmp::min(best_state.n320, best_state.n50 / 4);
                    best_state.n320 -= n;

                    n
                } else if n300.is_none() {
                    let n = cmp::min(best_state.n300, best_state.n50 / 4);
                    best_state.n300 -= n;

                    n
                } else {
                    0
                };

                best_state.n100 += 5 * n;
                best_state.n50 -= 4 * n;
            }
        } else if n320.is_none() && n200.is_none() && n100.is_none() {
            let n = cmp::min(best_state.n320, best_state.n100);
            best_state.n320 -= n;
            best_state.n200 += 2 * n;
            best_state.n100 -= n;
>>>>>>> 42db299 (meow)
        }

        best_state
    }

    proptest! {
<<<<<<< HEAD
        #![proptest_config(ProptestConfig {
            cases: 20,
            ..Default::default()
        })]

        #[test]
        #[ignore = "cannot skip persistent failure cases for some reason which run way too slowly"]
        fn mania_hitresults(
            classic in prop::bool::ANY,
=======
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn mania_hitresults(
            lazer in prop::bool::ANY,
>>>>>>> 42db299 (meow)
            acc in 0.0_f64..=1.0,
            n320 in prop::option::weighted(0.10, 0_u32..=N_OBJECTS + N_HOLD_NOTES + 10),
            n300 in prop::option::weighted(0.10, 0_u32..=N_OBJECTS + N_HOLD_NOTES + 10),
            n200 in prop::option::weighted(0.10, 0_u32..=N_OBJECTS + N_HOLD_NOTES + 10),
            n100 in prop::option::weighted(0.10, 0_u32..=N_OBJECTS + N_HOLD_NOTES + 10),
            n50 in prop::option::weighted(0.10, 0_u32..=N_OBJECTS + N_HOLD_NOTES + 10),
            n_misses in prop::option::weighted(0.15, 0_u32..=N_OBJECTS + N_HOLD_NOTES + 10),
            best_case in prop::bool::ANY,
        ) {
<<<<<<< HEAD
            exec_mania_hitresults(classic, acc, n320, n300, n200, n100, n50, n_misses, best_case);
        }
    }

    #[test]
    fn rng_mania_hitresults() {
        /// Generates a random seed by measuring the time it takes to calculate
        /// all primes up to 10_000.
        fn generate_seed() -> [u8; 16] {
            let start = Instant::now();

            const LIMIT: usize = 10_000;
            let mut is_prime = vec![true; LIMIT + 1];
            is_prime.iter_mut().step_by(2).for_each(|n| *n = false);
            is_prime[1] = false;
            is_prime[2] = true;

            for n in (3..=LIMIT).step_by(2) {
                if !is_prime[n] {
                    continue;
                }

                for m in (n * n..=LIMIT).step_by(n) {
                    is_prime[m] = false;
                }
            }

            start.elapsed().as_nanos().to_le_bytes()
        }

        let seed = generate_seed();
        eprintln!("seed={seed:?}");
        let mut rng = TestRng::from_seed(RngAlgorithm::XorShift, &seed);

        // Worst-case test cases can take over 5 minutes to bruteforce on debug
        // mode so we shouldn't over do the amount here.
        const CASES: usize = 4;

        for _ in 0..CASES {
            const LIMIT: u32 = N_OBJECTS + N_HOLD_NOTES + 10;

            let classic = rng.gen();
            let acc = rng.gen_range(0.0..=1.0);
            let n320 = rng.gen_bool(0.1).then(|| rng.gen_range(0..=LIMIT));
            let n300 = rng.gen_bool(0.1).then(|| rng.gen_range(0..=LIMIT));
            let n200 = rng.gen_bool(0.1).then(|| rng.gen_range(0..=LIMIT));
            let n100 = rng.gen_bool(0.1).then(|| rng.gen_range(0..=LIMIT));
            let n50 = rng.gen_bool(0.1).then(|| rng.gen_range(0..=LIMIT));
            let n_misses = rng.gen_bool(0.2).then(|| rng.gen_range(0..=LIMIT));
            let best_case = rng.gen();

            eprintln!(
                "classic={} | acc={} | n320={:?} | n300={:?} | n200={:?} | \
                n100={:?} | n50={:?} | n_misses={:?} | best_case={}",
                classic, acc, n320, n300, n200, n100, n50, n_misses, best_case,
            );

            exec_mania_hitresults(
                classic, acc, n320, n300, n200, n100, n50, n_misses, best_case,
            );
        }
    }

    fn exec_mania_hitresults(
        classic: bool,
        acc: f64,
        n320: Option<u32>,
        n300: Option<u32>,
        n200: Option<u32>,
        n100: Option<u32>,
        n50: Option<u32>,
        n_misses: Option<u32>,
        best_case: bool,
    ) {
        let priority = if best_case {
            HitResultPriority::BestCase
        } else {
            HitResultPriority::WorstCase
        };

        let mut state = ManiaPerformance::from(attrs())
            .accuracy(acc * 100.0)
            .lazer(!classic)
            .mods(mods(classic))
            .hitresult_priority(priority);

        if let Some(n320) = n320 {
            state = state.n320(n320);
        }

        if let Some(n300) = n300 {
            state = state.n300(n300);
        }

        if let Some(n200) = n200 {
            state = state.n200(n200);
        }

        if let Some(n100) = n100 {
            state = state.n100(n100);
        }

        if let Some(n50) = n50 {
            state = state.n50(n50);
        }

        if let Some(misses) = n_misses {
            state = state.misses(misses);
        }

        let start = Instant::now();
        let first = state.generate_state().unwrap();
        let state_elapsed = start.elapsed();
        let state = state.generate_state().unwrap();
        assert_eq!(first, state);

        let start = Instant::now();
        let expected = brute_force_best(
            classic,
            acc,
            n320,
            n300,
            n200,
            n100,
            n50,
            n_misses.unwrap_or(0),
            best_case,
        );
        let bf_elapsed = start.elapsed();

        eprintln!("Elapsed: state={state_elapsed:?} bf={bf_elapsed:?}");

        assert_eq!(
            state,
            expected,
            "dist: {} vs {}",
            (state.accuracy(classic) - acc).abs(),
            (expected.accuracy(classic) - acc).abs(),
        );
    }

    #[test]
    fn hitresults_n320_misses_best() {
        let classic = true;

        let state = ManiaPerformance::from(attrs())
            .lazer(!classic)
            .mods(mods(classic))
=======
            let priority = if best_case {
                HitResultPriority::BestCase
            } else {
                HitResultPriority::WorstCase
            };

            let mut state = ManiaPerformance::from(attrs())
                .accuracy(acc * 100.0)
                .lazer(lazer)
                .hitresult_priority(priority);

            if let Some(n320) = n320 {
                state = state.n320(n320);
            }

            if let Some(n300) = n300 {
                state = state.n300(n300);
            }

            if let Some(n200) = n200 {
                state = state.n200(n200);
            }

            if let Some(n100) = n100 {
                state = state.n100(n100);
            }

            if let Some(n50) = n50 {
                state = state.n50(n50);
            }

            if let Some(misses) = n_misses {
                state = state.misses(misses);
            }

            let first = state.generate_state().unwrap();
            let state = state.generate_state().unwrap();
            assert_eq!(first, state);

            let expected = brute_force_best(
                lazer,
                acc,
                n320,
                n300,
                n200,
                n100,
                n50,
                n_misses.unwrap_or(0),
                best_case,
            );

            assert_eq!(state, expected);
        }
    }

    #[test]
    fn hitresults_n320_misses_best() {
        let state = ManiaPerformance::from(attrs())
            .lazer(false)
>>>>>>> 42db299 (meow)
            .n320(500)
            .misses(2)
            .hitresult_priority(HitResultPriority::BestCase)
            .generate_state()
            .unwrap();

        let expected = ManiaScoreState {
            n320: 500,
            n300: 92,
            n200: 0,
            n100: 0,
            n50: 0,
            misses: 2,
        };

        assert_eq!(state, expected);
    }

    #[test]
    fn hitresults_n100_n50_misses_worst() {
<<<<<<< HEAD
        let classic = true;

        let state = ManiaPerformance::from(attrs())
            .lazer(!classic)
            .mods(mods(classic))
=======
        let state = ManiaPerformance::from(attrs())
            .lazer(false)
>>>>>>> 42db299 (meow)
            .n100(200)
            .n50(50)
            .misses(2)
            .hitresult_priority(HitResultPriority::WorstCase)
            .generate_state()
            .unwrap();

        let expected = ManiaScoreState {
            n320: 0,
            n300: 0,
            n200: 342,
            n100: 200,
            n50: 50,
            misses: 2,
        };

        assert_eq!(state, expected);
    }

    #[test]
    fn create() {
        let mut map = beatmap();

        let _ = ManiaPerformance::new(ManiaDifficultyAttributes::default());
        let _ = ManiaPerformance::new(ManiaPerformanceAttributes::default());
        let _ = ManiaPerformance::new(&map);
        let _ = ManiaPerformance::new(map.clone());

        let _ = ManiaPerformance::try_new(ManiaDifficultyAttributes::default()).unwrap();
        let _ = ManiaPerformance::try_new(ManiaPerformanceAttributes::default()).unwrap();
        let _ = ManiaPerformance::try_new(DifficultyAttributes::Mania(
            ManiaDifficultyAttributes::default(),
        ))
        .unwrap();
        let _ = ManiaPerformance::try_new(PerformanceAttributes::Mania(
            ManiaPerformanceAttributes::default(),
        ))
        .unwrap();
        let _ = ManiaPerformance::try_new(&map).unwrap();
        let _ = ManiaPerformance::try_new(map.clone()).unwrap();

        let _ = ManiaPerformance::from(ManiaDifficultyAttributes::default());
        let _ = ManiaPerformance::from(ManiaPerformanceAttributes::default());
        let _ = ManiaPerformance::from(&map);
        let _ = ManiaPerformance::from(map.clone());

        let _ = ManiaDifficultyAttributes::default().performance();
        let _ = ManiaPerformanceAttributes::default().performance();

        assert!(map
            .convert_mut(GameMode::Osu, &GameMods::default())
            .is_err());

        assert!(ManiaPerformance::try_new(OsuDifficultyAttributes::default()).is_none());
        assert!(ManiaPerformance::try_new(OsuPerformanceAttributes::default()).is_none());
        assert!(ManiaPerformance::try_new(DifficultyAttributes::Osu(
            OsuDifficultyAttributes::default()
        ))
        .is_none());
        assert!(ManiaPerformance::try_new(PerformanceAttributes::Osu(
            OsuPerformanceAttributes::default()
        ))
        .is_none());
    }
}
