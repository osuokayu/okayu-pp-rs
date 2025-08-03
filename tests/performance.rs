use std::panic::{self, UnwindSafe};

<<<<<<< HEAD
<<<<<<< HEAD
use okayu_pp::{
=======
use akatsuki_pp::{
>>>>>>> 42db299 (meow)
=======
use okayu_pp::{
>>>>>>> 0929f74 (change akatsuki to okayu)
    catch::{CatchPerformance, CatchPerformanceAttributes},
    mania::{ManiaPerformance, ManiaPerformanceAttributes},
    osu::{OsuPerformance, OsuPerformanceAttributes},
    taiko::{TaikoPerformance, TaikoPerformanceAttributes},
    Beatmap,
};

use self::common::*;

mod common;

macro_rules! test_cases {
    ( $mode:ident: $path:ident {
        $( $( $mods:ident )+ => {
            $( $key:ident: $value:expr $( , )? )*
        } ;)*
    } ) => {
        let map = Beatmap::from_path(common::$path).unwrap();

        $(
            let mods = 0 $( + $mods )*;
            let (calc, expected) = test_cases!(@$mode { map, $( $key: $value, )* });
            let actual = calc.mods(mods).calculate().unwrap();
            run(&actual, &expected, mods);
        )*
    };
    ( @Osu {
        $map:ident,
        pp: $pp:expr,
        pp_acc: $pp_acc:expr,
        pp_aim: $pp_aim:expr,
        pp_flashlight: $pp_flashlight:expr,
        pp_speed: $pp_speed:expr,
        effective_miss_count: $effective_miss_count:expr,
<<<<<<< HEAD
        speed_deviation: $speed_deviation:expr,
=======
>>>>>>> 42db299 (meow)
    }) => {
        (
            OsuPerformance::from(&$map).lazer(true),
            OsuPerformanceAttributes {
                pp: $pp,
                pp_acc: $pp_acc,
                pp_aim: $pp_aim,
                pp_flashlight: $pp_flashlight,
                pp_speed: $pp_speed,
                effective_miss_count: $effective_miss_count,
<<<<<<< HEAD
                speed_deviation: $speed_deviation,
=======
>>>>>>> 42db299 (meow)
                ..Default::default()
            },
        )
    };
    ( @Taiko {
        $map: ident,
        pp: $pp:expr,
        pp_acc: $pp_acc:expr,
        pp_difficulty: $pp_difficulty:expr,
        effective_miss_count: $effective_miss_count:expr,
        estimated_unstable_rate: $estimated_unstable_rate:expr,
    }) => {
        (
            TaikoPerformance::from(&$map),
            TaikoPerformanceAttributes {
                pp: $pp,
                pp_acc: $pp_acc,
                pp_difficulty: $pp_difficulty,
                effective_miss_count: $effective_miss_count,
                estimated_unstable_rate: $estimated_unstable_rate,
                ..Default::default()
            },
        )
    };
    ( @Catch {
        $map:ident,
        pp: $pp:expr,
    }) => {
        (
            CatchPerformance::from(&$map),
            CatchPerformanceAttributes {
                pp: $pp,
                ..Default::default()
            },
        )
    };
    ( @Mania {
        $map:ident,
        pp: $pp:expr,
        pp_difficulty: $pp_difficulty:expr,
    }) => {
        (
            ManiaPerformance::from(&$map),
            ManiaPerformanceAttributes {
                pp: $pp,
                pp_difficulty: $pp_difficulty,
                ..Default::default()
            },
        )
    };
}

#[test]
fn basic_osu() {
    #[cfg(target_os = "windows")]
    test_cases! {
        Osu: OSU {
            NM => {
<<<<<<< HEAD
                pp: 273.55482143018787,
                pp_acc: 97.62287463107766,
                pp_aim: 98.91907543994563,
                pp_flashlight: 0.0,
                pp_speed: 65.89857918351103,
                effective_miss_count: 0.0,
                speed_deviation: Some(11.855079578025586),
            };
            HD => {
                pp: 300.22543969392876,
                pp_acc: 105.43270460156388,
                pp_aim: 109.6023348327671,
                pp_flashlight: 0.0,
                pp_speed: 73.01562523256396,
                effective_miss_count: 0.0,
                speed_deviation: Some(11.855079578025586),
            };
            EZ HD => {
                pp: 186.89279040924507,
                pp_acc: 16.6270597231239,
                pp_aim: 96.66495037071935,
                pp_flashlight: 0.0,
                pp_speed: 63.20254195234833,
                effective_miss_count: 0.0,
                speed_deviation: Some(23.1539101317497),
            };
            HR => {
                pp: 405.7923033508039,
                pp_acc: 161.55575439788055,
                pp_aim: 145.16956208688748,
                pp_flashlight: 0.0,
                pp_speed: 81.7375074976892,
                effective_miss_count: 0.0,
                speed_deviation: Some(8.857953596258472),
            };
            DT => {
                pp: 741.3788121712137,
                pp_acc: 184.09450675506795,
                pp_aim: 301.88645138363296,
                pp_flashlight: 0.0,
                pp_speed: 224.91817628556774,
                effective_miss_count: 0.0,
                speed_deviation: Some(7.873979522967204),
            };
            FL => {
                pp: 403.31324405364177,
                pp_acc: 99.57533212369923,
                pp_aim: 98.91907543994563,
                pp_flashlight: 132.28811994208644,
                pp_speed: 65.89857918351103,
                effective_miss_count: 0.0,
                speed_deviation: Some(11.855079578025586),
            };
            HD FL => {
                pp: 470.3193633451629,
                pp_acc: 107.54135869359516,
                pp_aim: 109.6023348327671,
                pp_flashlight: 171.61406165164135,
                pp_speed: 73.01562523256396,
                effective_miss_count: 0.0,
                speed_deviation: Some(11.855079578025586),
=======
                pp: 272.6047426867276,
                pp_acc: 97.62287463107766,
                pp_aim: 99.3726518686143,
                pp_flashlight: 0.0,
                pp_speed: 64.48542022217285,
                effective_miss_count: 0.0,
            };
            HD => {
                pp: 299.17174736245374,
                pp_acc: 105.43270460156388,
                pp_aim: 110.10489751227146,
                pp_flashlight: 0.0,
                pp_speed: 71.4498451141828,
                effective_miss_count: 0.0,
            };
            EZ HD => {
                pp: 186.7137498214991,
                pp_acc: 16.6270597231239,
                pp_aim: 98.11121656070222,
                pp_flashlight: 0.0,
                pp_speed: 61.51901495973101,
                effective_miss_count: 0.0,
            };
            HR => {
                pp: 404.7030358947424,
                pp_acc: 161.55575439788055,
                pp_aim: 145.04665418031985,
                pp_flashlight: 0.0,
                pp_speed: 80.77088499277514,
                effective_miss_count: 0.0,
            };
            DT => {
                pp: 738.7899608061098,
                pp_acc: 184.09450675506795,
                pp_aim: 304.16666833057235,
                pp_flashlight: 0.0,
                pp_speed: 220.06297202966698,
                effective_miss_count: 0.0,
            };
            FL => {
                pp: 402.408877784248,
                pp_acc: 99.57533212369923,
                pp_aim: 99.3726518686143,
                pp_flashlight: 132.29720631068272,
                pp_speed: 64.48542022217285,
                effective_miss_count: 0.0,
            };
            HD FL => {
                pp: 469.3245236137446,
                pp_acc: 107.54135869359516,
                pp_aim: 110.10489751227146,
                pp_flashlight: 171.62594459401154,
                pp_speed: 71.4498451141828,
                effective_miss_count: 0.0,
            };
        }
    };
    #[cfg(target_os = "linux")]
    test_cases! {
        Osu: OSU {
            NM => {
                pp: 272.6047426867276,
                pp_acc: 97.62287463107766,
                pp_aim: 99.37265186861426,
                pp_flashlight: 0.0,
                pp_speed: 64.48542022217285,
                effective_miss_count: 0.0,
            };
            HD => {
                pp: 299.17174736245363,
                pp_acc: 105.43270460156388,
                pp_aim: 110.10489751227142,
                pp_flashlight: 0.0,
                pp_speed: 71.4498451141828,
                effective_miss_count: 0.0,
            };
            EZ HD => {
                pp: 186.7137498214991,
                pp_acc: 16.6270597231239,
                pp_aim: 98.11121656070222,
                pp_flashlight: 0.0,
                pp_speed: 61.51901495973101,
                effective_miss_count: 0.0,
            };
            HR => {
                pp: 404.7030358947424,
                pp_acc: 161.55575439788055,
                pp_aim: 145.04665418031985,
                pp_flashlight: 0.0,
                pp_speed: 80.77088499277514,
                effective_miss_count: 0.0,
            };
            DT => {
                pp: 738.7899608061098,
                pp_acc: 184.09450675506795,
                pp_aim: 304.16666833057235,
                pp_flashlight: 0.0,
                pp_speed: 220.06297202966698,
                effective_miss_count: 0.0,
            };
            FL => {
                pp: 402.408877784248,
                pp_acc: 99.57533212369923,
                pp_aim: 99.37265186861426,
                pp_flashlight: 132.29720631068272,
                pp_speed: 64.48542022217285,
                effective_miss_count: 0.0,
            };
            HD FL => {
                pp: 469.3245236137446,
                pp_acc: 107.54135869359516,
                pp_aim: 110.10489751227142,
                pp_flashlight: 171.62594459401154,
                pp_speed: 71.4498451141828,
                effective_miss_count: 0.0,
>>>>>>> 42db299 (meow)
            };
        }
    };
}

#[test]
fn basic_taiko() {
    test_cases! {
        Taiko: TAIKO {
            NM => {
<<<<<<< HEAD
                pp: 104.65974235594882,
                pp_acc: 67.01508452097738,
                pp_difficulty: 30.951117266143964,
=======
                pp: 114.68651694107942,
                pp_acc: 67.10083752258917,
                pp_difficulty: 40.6658183165898,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(148.44150180469418),
            };
            HD => {
<<<<<<< HEAD
                pp: 113.35231886537841,
                pp_acc: 67.01508452097738,
                pp_difficulty: 31.72489519779756,
=======
                pp: 124.41592086295445,
                pp_acc: 67.10083752258917,
                pp_difficulty: 41.68246377450454,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(148.44150180469418),
            };
            HR => {
<<<<<<< HEAD
                pp: 125.39316057548226,
                pp_acc: 83.3355298805701,
                pp_difficulty: 33.77220597125385,
=======
                pp: 138.3981102935321,
                pp_acc: 82.52109686788792,
                pp_difficulty: 47.44272798866182,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(122.99438720960376),
            };
            DT => {
<<<<<<< HEAD
                pp: 217.2255599983772,
                pp_acc: 119.35453575917016,
                pp_difficulty: 85.09547264616562,
=======
                pp: 220.07140899937482,
                pp_acc: 118.28107309573312,
                pp_difficulty: 88.93091255724303,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(98.96100120312946),
            };
        }
    };
}

#[test]
fn convert_taiko() {
    #[cfg(target_os = "windows")]
    test_cases! {
        Taiko: OSU {
            NM => {
<<<<<<< HEAD
                pp: 321.96508788209525,
                pp_acc: 150.50068595207387,
                pp_difficulty: 152.95500113793892,
=======
                pp: 353.6961706002712,
                pp_acc: 155.09212159726567,
                pp_difficulty: 178.19145253120928,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(85.75868894575865),
            };
            HD => {
<<<<<<< HEAD
                pp: 326.0279405978374,
                pp_acc: 150.50068595207387,
                pp_difficulty: 156.7788761663874,
=======
                pp: 358.45704044422996,
                pp_acc: 155.09212159726567,
                pp_difficulty: 182.6462388444895,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(85.75868894575865),
            };
            HR => {
<<<<<<< HEAD
                pp: 400.1259115798042,
                pp_acc: 187.46770845243455,
                pp_difficulty: 189.65602547641478,
=======
                pp: 405.57235351353773,
                pp_acc: 186.06296332183615,
                pp_difficulty: 196.1813610529617,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(72.67685680089848),
            };
            DT => {
<<<<<<< HEAD
                pp: 688.6809319343615,
                pp_acc: 274.8702821415836,
                pp_difficulty: 373.46911205993484,
=======
                pp: 658.0214875413873,
                pp_acc: 272.26616492989393,
                pp_difficulty: 347.4712042359611,
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(57.17245929717244),
            };
        }
    }
    #[cfg(target_os = "linux")]
    test_cases! {
        Taiko: OSU {
            NM => {
                pp: 353.6961706002712,
                pp_acc: 155.09212159726567,
                pp_difficulty: 178.19145253120928,
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(85.75868894575865),
            };
            HD => {
                pp: 358.45704044423
                pp_acc: 155.09212159726567,
                pp_difficulty: 182.6462388444895,
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(85.75868894575865),
            };
            HR => {
                pp: 405.57235351353773,
                pp_acc: 186.06296332183615,
                pp_difficulty: 196.1813610529617,
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(72.67685680089848),
            };
            DT => {
                pp: 658.0214875413873
                pp_acc: 272.26616492989393,
                pp_difficulty: 347.4712042359611,
>>>>>>> 42db299 (meow)
                effective_miss_count: 0.0,
                estimated_unstable_rate: Some(57.17245929717244),
            };
        }
    }
}

#[test]
fn basic_catch() {
    test_cases! {
        Catch: CATCH {
            NM => { pp: 113.85903714373046 };
            HD => { pp: 136.63084457247655 };
            HD HR => { pp: 231.7403429678108 };
            DT => { pp: 247.18402249125842 };
        }
    };
}

#[test]
fn convert_catch() {
    test_cases! {
        Catch: OSU {
            NM => { pp: 232.52175944328079 };
            HD => { pp: 256.35523645996665 };
            HD HR => { pp: 327.71861407740374 };
            DT => { pp: 503.47065792054815 };
        }
    };
}

#[test]
fn basic_mania() {
    test_cases! {
        Mania: MANIA {
            NM => { pp: 108.92297471705167, pp_difficulty: 108.92297471705167 };
            EZ => { pp: 54.46148735852584, pp_difficulty: 108.92297471705167 };
            DT => { pp: 224.52717042937203, pp_difficulty: 224.52717042937203 };
        }
    };
}

#[test]
fn convert_mania() {
    test_cases! {
        Mania: OSU {
            NM => { pp: 101.39189449271568, pp_difficulty: 101.39189449271568 };
            EZ => { pp: 50.69594724635784, pp_difficulty: 101.39189449271568 };
            DT => { pp: 198.46891237015896, pp_difficulty: 198.46891237015896 };
        }
    };
}

fn run<A>(actual: &A, expected: &A, mods: u32)
where
    A: AssertEq,
    for<'a> &'a A: UnwindSafe,
{
    if panic::catch_unwind(|| actual.assert_eq(expected)).is_err() {
        panic!("Mods: {mods}");
    }
}

impl AssertEq for OsuPerformanceAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            difficulty: _,
            pp,
            pp_acc,
            pp_aim,
            pp_flashlight,
            pp_speed,
            effective_miss_count,
            speed_deviation,
        } = self;

        assert_eq_float(*pp, expected.pp);
        assert_eq_float(*pp_acc, expected.pp_acc);
        assert_eq_float(*pp_aim, expected.pp_aim);
        assert_eq_float(*pp_flashlight, expected.pp_flashlight);
        assert_eq_float(*pp_speed, expected.pp_speed);
        assert_eq_float(*effective_miss_count, expected.effective_miss_count);
        assert_eq_option(*speed_deviation, expected.speed_deviation);
=======
        assert_eq_float(self.pp, expected.pp);
        assert_eq_float(self.pp_acc, expected.pp_acc);
        assert_eq_float(self.pp_aim, expected.pp_aim);
        assert_eq_float(self.pp_flashlight, expected.pp_flashlight);
        assert_eq_float(self.pp_speed, expected.pp_speed);
        assert_eq_float(self.effective_miss_count, expected.effective_miss_count);
>>>>>>> 42db299 (meow)
    }
}

impl AssertEq for TaikoPerformanceAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            difficulty: _,
            pp,
            pp_acc,
            pp_difficulty,
            effective_miss_count,
            estimated_unstable_rate,
        } = self;

        assert_eq_float(*pp, expected.pp);
        assert_eq_float(*pp_acc, expected.pp_acc);
        assert_eq_float(*pp_difficulty, expected.pp_difficulty);
        assert_eq_float(*effective_miss_count, expected.effective_miss_count);
        assert_eq_option(*estimated_unstable_rate, expected.estimated_unstable_rate);
=======
        assert_eq_float(self.pp, expected.pp);
        assert_eq_float(self.pp_acc, expected.pp_acc);
        assert_eq_float(self.pp_difficulty, expected.pp_difficulty);
        assert_eq_float(self.effective_miss_count, expected.effective_miss_count);
>>>>>>> 42db299 (meow)
    }
}

impl AssertEq for CatchPerformanceAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self { difficulty: _, pp } = self;

        assert_eq_float(*pp, expected.pp);
=======
        assert_eq_float(self.pp, expected.pp);
>>>>>>> 42db299 (meow)
    }
}

impl AssertEq for ManiaPerformanceAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            difficulty: _,
            pp,
            pp_difficulty,
        } = self;

        assert_eq_float(*pp_difficulty, expected.pp_difficulty);
        assert_eq_float(*pp, expected.pp);
=======
        assert_eq_float(self.pp_difficulty, expected.pp_difficulty);
        assert_eq_float(self.pp, expected.pp);
>>>>>>> 42db299 (meow)
    }
}
