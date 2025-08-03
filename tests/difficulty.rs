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
    catch::{Catch, CatchDifficultyAttributes},
    mania::{Mania, ManiaDifficultyAttributes},
    osu::{Osu, OsuDifficultyAttributes},
    taiko::{Taiko, TaikoDifficultyAttributes},
    Beatmap, Difficulty,
};

use self::common::*;

mod common;

macro_rules! test_cases {
    ( $mode:ident: $path:ident {
        $( $( $mods:ident )+ => {
            $( $key:ident: $value:literal $( , )? )*
        } $( ; )? )*
    } ) => {
        let map = Beatmap::from_path(common::$path).unwrap();

        $(
            let mods = 0 $( + $mods )*;
            let expected = test_cases!(@$mode { $( $key: $value, )* });

            let actual = Difficulty::new()
                .mods(mods)
                .calculate_for_mode::<$mode>(&map)
                .unwrap();

            run(&actual, &expected, mods);
        )*
    };
    ( @Osu {
        aim: $aim:literal,
<<<<<<< HEAD
        aim_difficult_slider_count: $aim_difficult_slider_count:literal,
=======
>>>>>>> 42db299 (meow)
        speed: $speed:literal,
        flashlight: $flashlight:literal,
        slider_factor: $slider_factor:literal,
        speed_note_count: $speed_note_count:literal,
        aim_difficult_strain_count: $aim_difficult_strain_count:literal,
        speed_difficult_strain_count: $speed_difficult_strain_count:literal,
        ar: $ar:literal,
<<<<<<< HEAD
        great_hit_window: $great_hit_window:literal,
        ok_hit_window: $ok_hit_window:literal,
        meh_hit_window: $meh_hit_window:literal,
=======
        od: $od:literal,
>>>>>>> 42db299 (meow)
        hp: $hp:literal,
        n_circles: $n_circles:literal,
        n_sliders: $n_sliders:literal,
        n_large_ticks: $n_large_ticks:literal,
        n_spinners: $n_spinners:literal,
        stars: $stars:literal,
        max_combo: $max_combo:literal,
    }) => {
        OsuDifficultyAttributes {
            aim: $aim,
<<<<<<< HEAD
            aim_difficult_slider_count: $aim_difficult_slider_count,
=======
>>>>>>> 42db299 (meow)
            speed: $speed,
            flashlight: $flashlight,
            slider_factor: $slider_factor,
            speed_note_count: $speed_note_count,
            aim_difficult_strain_count: $aim_difficult_strain_count,
            speed_difficult_strain_count: $speed_difficult_strain_count,
            ar: $ar,
<<<<<<< HEAD
            great_hit_window: $great_hit_window,
            ok_hit_window: $ok_hit_window,
            meh_hit_window: $meh_hit_window,
=======
            od: $od,
>>>>>>> 42db299 (meow)
            hp: $hp,
            n_circles: $n_circles,
            n_sliders: $n_sliders,
            n_large_ticks: $n_large_ticks,
            n_spinners: $n_spinners,
            stars: $stars,
            max_combo: $max_combo,
        }
    };
    ( @Taiko {
        stamina: $stamina:literal,
        rhythm: $rhythm:literal,
        color: $color:literal,
<<<<<<< HEAD
        reading: $reading:literal,
        great_hit_window: $great_hit_window:literal,
        ok_hit_window: $ok_hit_window:literal,
        mono_stamina_factor: $mono_stamina_factor:literal,
=======
        peak: $peak:literal,
        mono_stamina_factor: $mono_stamina_factor:literal,
        great_hit_window: $great_hit_window:literal,
        ok_hit_window: $ok_hit_window:literal,
>>>>>>> 42db299 (meow)
        stars: $stars:literal,
        max_combo: $max_combo:literal,
        is_convert: $is_convert:literal,
    }) => {
        TaikoDifficultyAttributes {
            stamina: $stamina,
            rhythm: $rhythm,
            color: $color,
<<<<<<< HEAD
            reading: $reading,
            great_hit_window: $great_hit_window,
            ok_hit_window: $ok_hit_window,
            mono_stamina_factor: $mono_stamina_factor,
=======
            peak: $peak,
            mono_stamina_factor: $mono_stamina_factor,
            great_hit_window: $great_hit_window,
            ok_hit_window: $ok_hit_window,
>>>>>>> 42db299 (meow)
            stars: $stars,
            max_combo: $max_combo,
            is_convert: $is_convert,
        }
    };
    ( @Catch {
        stars: $stars:literal,
        ar: $ar:literal,
        n_fruits: $n_fruits:literal,
        n_droplets: $n_droplets:literal,
        n_tiny_droplets: $n_tiny_droplets:literal,
        is_convert: $is_convert:literal,
    }) => {
        CatchDifficultyAttributes {
            stars: $stars,
            ar: $ar,
            n_fruits: $n_fruits,
            n_droplets: $n_droplets,
            n_tiny_droplets: $n_tiny_droplets,
            is_convert: $is_convert,
        }
    };
    ( @Mania {
        stars: $stars:literal,
<<<<<<< HEAD
=======
        hit_window: $hit_window:literal,
>>>>>>> 42db299 (meow)
        n_objects: $n_objects:literal,
        n_hold_notes: $n_hold_notes:literal,
        max_combo: $max_combo:literal,
        is_convert: $is_convert:literal,
    }) => {
        ManiaDifficultyAttributes {
            stars: $stars,
<<<<<<< HEAD
=======
            hit_window: $hit_window,
>>>>>>> 42db299 (meow)
            n_objects: $n_objects,
            n_hold_notes: $n_hold_notes,
            max_combo: $max_combo,
            is_convert: $is_convert,
        }
    }
}

#[test]
fn basic_osu() {
    #[cfg(target_os = "windows")]
    test_cases! {
        Osu: OSU {
            NM => {
<<<<<<< HEAD
                aim: 2.8768763397837276,
                aim_difficult_slider_count: 159.94435184493983,
                speed: 2.4859791945784644,
                flashlight: 2.287810214401711,
                slider_factor: 0.9804419804851772,
                speed_note_count: 203.4377249350029,
                aim_difficult_strain_count: 109.81975624515795,
                speed_difficult_strain_count: 79.45944001555702,
                ar: 9.300000190734863,
                great_hit_window: 27.19999885559082,
                ok_hit_window: 69.5999984741211,
                meh_hit_window: 111.99999809265137,
=======
                aim: 2.881184366758021,
                speed: 2.468469273849314,
                flashlight: 2.287888783550428,
                slider_factor: 0.9803293523973865,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488378,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
>>>>>>> 42db299 (meow)
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
<<<<<<< HEAD
                stars: 5.653394946111255,
                max_combo: 909,
            };
            HD => {
                aim: 2.8768763397837276,
                aim_difficult_slider_count: 159.94435184493983,
                speed: 2.4859791945784644,
                flashlight: 2.605769566257193,
                slider_factor: 0.9804419804851772,
                speed_note_count: 203.4377249350029,
                aim_difficult_strain_count: 109.81975624515795,
                speed_difficult_strain_count: 79.45944001555702,
                ar: 9.300000190734863,
                great_hit_window: 27.19999885559082,
                ok_hit_window: 69.5999984741211,
                meh_hit_window: 111.99999809265137,
=======
                stars: 5.643619989739299,
                max_combo: 909,
            };
            HD => {
                aim: 2.881184366758021,
                speed: 2.468469273849314,
                flashlight: 2.605859779358901,
                slider_factor: 0.9803293523973865,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488378,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
>>>>>>> 42db299 (meow)
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
<<<<<<< HEAD
                stars: 5.653394946111255,
                max_combo: 909,
            };
            HR => {
                aim: 3.2524329530597154,
                aim_difficult_slider_count: 166.54650855486952,
                speed: 2.642601593996546,
                flashlight: 2.8536623110985535,
                slider_factor: 0.9698688450936457,
                speed_note_count: 178.23533084488034,
                aim_difficult_strain_count: 109.72159733275541,
                speed_difficult_strain_count: 73.57962712314226,
                ar: 10.0,
                great_hit_window: 20.0,
                ok_hit_window: 60.0,
                meh_hit_window: 100.0,
=======
                stars: 5.643619989739299,
                max_combo: 909,
            };
            HR => {
                aim: 3.2515300463985666,
                speed: 2.6323568908654615,
                flashlight: 2.853761577136605,
                slider_factor: 0.969089944826546,
                speed_note_count: 178.52041495886283,
                aim_difficult_strain_count: 108.03970474535397,
                speed_difficult_strain_count: 73.27713411796513,
                ar: 10.0,
                od: 10.0,
>>>>>>> 42db299 (meow)
                hp: 7.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
<<<<<<< HEAD
                stars: 6.252509796432301,
                max_combo: 909,
            };
            DT => {
                aim: 4.048049265438377,
                aim_difficult_slider_count: 175.9330312155201,
                speed: 3.5966087398365265,
                flashlight: 3.3180899820069842,
                slider_factor: 0.9780331301164369,
                speed_note_count: 209.75822813365252,
                aim_difficult_strain_count: 132.07784609607364,
                speed_difficult_strain_count: 94.98876627500661,
                ar: 10.53333346048991,
                great_hit_window: 18.13333257039388,
                ok_hit_window: 46.3999989827474,
                meh_hit_window: 74.6666653951009,
=======
                stars: 6.243301253337941,
                max_combo: 909,
            };
            DT => {
                aim: 4.058080039906945,
                speed: 3.570932204630734,
                flashlight: 3.318209122186825,
                slider_factor: 0.9777224379583133,
                speed_note_count: 211.29204189490912,
                aim_difficult_strain_count: 126.9561362975524,
                speed_difficult_strain_count: 95.63810649133869,
                ar: 10.53333346048991,
                od: 10.311111238267687,
>>>>>>> 42db299 (meow)
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
<<<<<<< HEAD
                stars: 8.041658803681496,
                max_combo: 909,
            };
            FL => {
                aim: 2.8768763397837276,
                aim_difficult_slider_count: 159.94435184493983,
                speed: 2.4859791945784644,
                flashlight: 2.287810214401711,
                slider_factor: 0.9804419804851772,
                speed_note_count: 203.4377249350029,
                aim_difficult_strain_count: 109.81975624515795,
                speed_difficult_strain_count: 79.45944001555702,
                ar: 9.300000190734863,
                great_hit_window: 27.19999885559082,
                ok_hit_window: 69.5999984741211,
                meh_hit_window: 111.99999809265137,
=======
                stars: 8.030649319285482,
                max_combo: 909,
            };
            FL => {
                aim: 2.881184366758021,
                speed: 2.468469273849314,
                flashlight: 2.287888783550428,
                slider_factor: 0.9803293523973865,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488378,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
>>>>>>> 42db299 (meow)
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
<<<<<<< HEAD
                stars: 6.864894125872836,
                max_combo: 909,
            };
            HD FL => {
                aim: 2.8768763397837276,
                aim_difficult_slider_count: 159.94435184493983,
                speed: 2.4859791945784644,
                flashlight: 2.605769566257193,
                slider_factor: 0.9804419804851772,
                speed_note_count: 203.4377249350029,
                aim_difficult_strain_count: 109.81975624515795,
                speed_difficult_strain_count: 79.45944001555702,
                ar: 9.300000190734863,
                great_hit_window: 27.19999885559082,
                ok_hit_window: 69.5999984741211,
                meh_hit_window: 111.99999809265137,
=======
                stars: 6.858771801534423,
                max_combo: 909,
            };
            HD FL => {
                aim: 2.881184366758021,
                speed: 2.468469273849314,
                flashlight: 2.605859779358901,
                slider_factor: 0.9803293523973865,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488378,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
>>>>>>> 42db299 (meow)
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
<<<<<<< HEAD
                stars: 7.173433597920448,
=======
                stars: 7.167932950561898,
                max_combo: 909,
            };
        }
    };
    #[cfg(target_os = "linux")]
    test_cases! {
        Osu: OSU {
            NM => {
                aim: 2.8811843667580206,
                speed: 2.468469273849314,
                flashlight: 2.287888783550428,
                slider_factor: 0.9803293523973866,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488393,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
                stars: 5.6436199897393005,
                max_combo: 909,
            };
            HD => {
                aim: 2.8811843667580206,
                speed: 2.468469273849314,
                flashlight: 2.605859779358901,
                slider_factor: 0.9803293523973866,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488393,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
                stars: 5.6436199897393005,
                max_combo: 909,
            };
            HR => {
                aim: 3.2515300463985666,
                speed: 2.6323568908654615,
                flashlight: 2.853761577136605,
                slider_factor: 0.969089944826546,
                speed_note_count: 178.52041495886283,
                aim_difficult_strain_count: 108.03970474535397,
                speed_difficult_strain_count: 73.27713411796513,
                ar: 10.0,
                od: 10.0,
                hp: 7.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
                stars: 6.2433012533379415,
                max_combo: 909,
            };
            DT => {
                aim: 4.058080039906945,
                speed: 3.570932204630734,
                flashlight: 3.318209122186825,
                slider_factor: 0.9777224379583133,
                speed_note_count: 211.29204189490912,
                aim_difficult_strain_count: 126.95613629755243,
                speed_difficult_strain_count: 95.63810649133869,
                ar: 10.53333346048991,
                od: 10.311111238267687,
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
                stars: 8.030649319285482,
                max_combo: 909,
            };
            FL => {
                aim: 2.8811843667580206,
                speed: 2.468469273849314,
                flashlight: 2.287888783550428,
                slider_factor: 0.9803293523973866,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488393,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
                stars: 6.858771801534423,
                max_combo: 909,
            };
            HD FL => {
                aim: 2.8811843667580206,
                speed: 2.468469273849314,
                flashlight: 2.605859779358901,
                slider_factor: 0.9803293523973866,
                speed_note_count: 204.88794724609374,
                aim_difficult_strain_count: 106.63833474488393,
                speed_difficult_strain_count: 79.9883004295862,
                ar: 9.300000190734863,
                od: 8.800000190734863,
                hp: 5.0,
                n_circles: 307,
                n_sliders: 293,
                n_large_ticks: 15,
                n_spinners: 1,
                stars: 7.167932950561899,
>>>>>>> 42db299 (meow)
                max_combo: 909,
            };
        }
    };
}

#[test]
fn basic_taiko() {
<<<<<<< HEAD
    #[cfg(target_os = "windows")]
    test_cases! {
        Taiko: TAIKO {
            NM => {
                stamina: 1.6946519339726132,
                rhythm: 0.1469836127567273,
                color: 0.517224522408358,
                reading: 1.3847421923785603E-05,
                great_hit_window: 35.0,
                ok_hit_window: 80.0,
                mono_stamina_factor: 2.5849839992846003E-07,
                stars: 2.9052867123859096,
=======
    test_cases! {
        Taiko: TAIKO {
            NM => {
                stamina: 1.3991746883284406,
                rhythm: 0.20130047251681948,
                color: 1.0487315549761433,
                peak: 1.8422453377400778,
                mono_stamina_factor: 2.66403971858592e-07,
                great_hit_window: 35.0,
                ok_hit_window: 80.0,
                stars: 2.914589700180437,
>>>>>>> 42db299 (meow)
                max_combo: 289,
                is_convert: false,
            };
            HR => {
<<<<<<< HEAD
                stamina: 1.6946519339726132,
                rhythm: 0.1481975365980239,
                color: 0.517224522408358,
                reading: 0.48269837569606633,
                great_hit_window: 29.0,
                ok_hit_window: 68.0,
                mono_stamina_factor: 2.5849839992846003E-07,
                stars: 2.987035945441977,
=======
                stamina: 1.3991746883284406,
                rhythm: 0.20130047251681948,
                color: 1.0487315549761433,
                peak: 1.8422453377400778,
                mono_stamina_factor: 2.66403971858592e-07,
                great_hit_window: 29.0,
                ok_hit_window: 68.0,
                stars: 2.914589700180437,
>>>>>>> 42db299 (meow)
                max_combo: 289,
                is_convert: false,
            };
            DT => {
<<<<<<< HEAD
                stamina: 2.469654573687006,
                rhythm: 0.49890717637373116,
                color: 0.6582785829498352,
                reading: 0.18343567306243425,
                great_hit_window: 23.333333333333332,
                ok_hit_window: 53.333333333333336,
                mono_stamina_factor: 2.464552111442399E-07,
                stars: 4.032774431778195,
=======
                stamina: 2.0358868555131586,
                rhythm: 0.4448175371191029,
                color: 1.363762496098889,
                peak: 2.625066421324458,
                mono_stamina_factor: 2.515617502055679e-07,
                great_hit_window: 23.333333333333332,
                ok_hit_window: 53.333333333333336,
                stars: 3.942709244618132,
>>>>>>> 42db299 (meow)
                max_combo: 289,
                is_convert: false,
            };
        }
    };
}

#[test]
fn convert_taiko() {
    test_cases! {
        Taiko: OSU {
            NM => {
<<<<<<< HEAD
                stamina: 3.6244157068104066,
                rhythm: 1.1790872487169137,
                color: 1.3884965692008842,
                reading: 1.7629163125391505,
                great_hit_window: 23.59999942779541,
                ok_hit_window: 57.19999885559082,
                mono_stamina_factor: 0.0014311041774359666,
                stars: 4.856701972823887,
=======
                stamina: 2.9127139214411444,
                rhythm: 1.4696991260446617,
                color: 2.303228172964907,
                peak: 4.117779264387738,
                mono_stamina_factor: 0.0016957378202796742,
                great_hit_window: 23.59999942779541,
                ok_hit_window: 57.19999885559082,
                stars: 5.235637844901627,
>>>>>>> 42db299 (meow)
                max_combo: 908,
                is_convert: true,
            };
            HR => {
<<<<<<< HEAD
                stamina: 3.6244157068104066,
                rhythm: 1.198436503252946,
                color: 1.3884965692008842,
                reading: 2.26853983796185,
                great_hit_window: 20.0,
                ok_hit_window: 50.0,
                mono_stamina_factor: 0.0014311041774359666,
                stars: 5.335018644413355,
=======
                stamina: 2.9127139214411444,
                rhythm: 1.4696991260446617,
                color: 2.303228172964907,
                peak: 4.117779264387738,
                mono_stamina_factor: 0.0016957378202796742,
                great_hit_window: 20.0,
                ok_hit_window: 50.0
                stars: 5.235637844901627,
>>>>>>> 42db299 (meow)
                max_combo: 908,
                is_convert: true,
            };
            DT => {
<<<<<<< HEAD
                stamina: 5.506714422351734,
                rhythm: 1.8409407770037334,
                color: 1.8926518387821614,
                reading: 3.0585370998057053,
                great_hit_window: 15.733332951863607,
                ok_hit_window: 38.13333257039388,
                mono_stamina_factor: 0.0014418086037955797,
                stars: 7.1789968695409225,
=======
                stamina: 4.379782453136822,
                rhythm: 2.002843919169095,
                color: 3.1864894777399986,
                peak: 6.103209631166694,
                mono_stamina_factor: 0.0017075184344987763,
                great_hit_window: 15.733332951863607,
                ok_hit_window: 38.13333257039388,
                stars: 7.010168846394131,
>>>>>>> 42db299 (meow)
                max_combo: 908,
                is_convert: true,
            };
        }
    };
}

#[test]
fn basic_catch() {
    test_cases! {
        Catch: CATCH {
            NM => {
                stars: 3.250266313373984,
                ar: 8.0,
                n_fruits: 728,
                n_droplets: 2,
                n_tiny_droplets: 263,
                is_convert: false,
            };
            HR => {
                stars: 4.313360856186517,
                ar: 10.0,
                n_fruits: 728,
                n_droplets: 2,
                n_tiny_droplets: 263,
                is_convert: false,
            };
            EZ => {
                stars: 4.06522224010957,
                ar: 4.0,
                n_fruits: 728,
                n_droplets: 2,
                n_tiny_droplets: 263,
                is_convert: false,
            };
            DT => {
                stars: 4.635262826575386,
                ar: 9.666666666666668,
                n_fruits: 728,
                n_droplets: 2,
                n_tiny_droplets: 263,
                is_convert: false,
            };
        }
    };
}

#[test]
fn convert_catch() {
    test_cases! {
        Catch: OSU {
            NM => {
                stars: 4.528720977989276,
<<<<<<< HEAD
                ar: 9.300000190734863,
=======
                ar: 9.300000190734863
>>>>>>> 42db299 (meow)
                n_fruits: 908,
                n_droplets: 0,
                n_tiny_droplets: 159,
                is_convert: true,
            };
            HR => {
                stars: 5.076698043567007,
                ar: 10.0,
                n_fruits: 908,
                n_droplets: 0,
                n_tiny_droplets: 159,
                is_convert: true,
            };
            EZ => {
                stars: 3.593264064535228,
                ar: 4.650000095367432,
                n_fruits: 908,
                n_droplets: 0,
                n_tiny_droplets: 159,
                is_convert: true,
            };
            DT => {
                stars: 6.15540143757313,
                ar: 10.53333346048991,
                n_fruits: 908,
                n_droplets: 0,
                n_tiny_droplets: 159,
                is_convert: true,
            };
        }
    };
}

#[test]
fn basic_mania() {
    test_cases! {
        Mania: MANIA {
            NM => {
                stars: 3.358304846842773,
<<<<<<< HEAD
=======
                hit_window: 40.0,
>>>>>>> 42db299 (meow)
                n_objects: 594,
                n_hold_notes: 121,
                max_combo: 956,
                is_convert: false,
            };
            DT => {
                stars: 4.6072892053157295,
<<<<<<< HEAD
=======
                hit_window: 40.0,
>>>>>>> 42db299 (meow)
                n_objects: 594,
                n_hold_notes: 121,
                max_combo: 956,
                is_convert: false,
            };
        }
    };
}

#[test]
fn convert_mania() {
    test_cases! {
        Mania: OSU {
            NM => {
                stars: 3.2033142085672255,
<<<<<<< HEAD
                n_objects: 1046,
                n_hold_notes: 266,
=======
                hit_window: 34.0,
                n_objects: 1046,
                n_hold_notes: 293,
>>>>>>> 42db299 (meow)
                max_combo: 1381,
                is_convert: true,
            };
            DT => {
<<<<<<< HEAD
                stars: 4.2934063021960185,
                n_objects: 1046,
                n_hold_notes: 266,
=======
                stars:  4.2934063021960185,
                hit_window: 34.0,
                n_objects: 1046,
                n_hold_notes: 293,
>>>>>>> 42db299 (meow)
                max_combo: 1381,
                is_convert: true,
            };
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

impl AssertEq for OsuDifficultyAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            aim,
            aim_difficult_slider_count,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            aim_difficult_strain_count,
            speed_difficult_strain_count,
            ar,
            great_hit_window,
            ok_hit_window,
            meh_hit_window,
            hp,
            n_circles,
            n_sliders,
            n_large_ticks,
            n_spinners,
            stars,
            max_combo,
        } = self;

        assert_eq_float(*aim, expected.aim);
        assert_eq_float(
            *aim_difficult_slider_count,
            expected.aim_difficult_slider_count,
        );
        assert_eq_float(*speed, expected.speed);
        assert_eq_float(*flashlight, expected.flashlight);
        assert_eq_float(*slider_factor, expected.slider_factor);
        assert_eq_float(*speed_note_count, expected.speed_note_count);
        assert_eq_float(
            *aim_difficult_strain_count,
            expected.aim_difficult_strain_count,
        );
        assert_eq_float(
            *speed_difficult_strain_count,
            expected.speed_difficult_strain_count,
        );
        assert_eq_float(*ar, expected.ar);
        assert_eq_float(*great_hit_window, expected.great_hit_window);
        assert_eq_float(*ok_hit_window, expected.ok_hit_window);
        assert_eq_float(*meh_hit_window, expected.meh_hit_window);
        assert_eq_float(*hp, expected.hp);
        assert_eq!(*n_circles, expected.n_circles);
        assert_eq!(*n_sliders, expected.n_sliders);
        assert_eq!(*n_large_ticks, expected.n_large_ticks);
        assert_eq!(*n_spinners, expected.n_spinners);
        assert_eq_float(*stars, expected.stars);
        assert_eq!(*max_combo, expected.max_combo);
=======
        assert_eq_float(self.aim, expected.aim);
        assert_eq_float(self.speed, expected.speed);
        assert_eq_float(self.flashlight, expected.flashlight);
        assert_eq_float(self.slider_factor, expected.slider_factor);
        assert_eq_float(self.speed_note_count, expected.speed_note_count);
        assert_eq_float(self.ar, expected.ar);
        assert_eq_float(self.od, expected.od);
        assert_eq_float(self.hp, expected.hp);
        assert_eq!(self.n_circles, expected.n_circles);
        assert_eq!(self.n_sliders, expected.n_sliders);
        assert_eq!(self.n_spinners, expected.n_spinners);
        assert_eq_float(self.stars, expected.stars);
        assert_eq!(self.max_combo, expected.max_combo);
>>>>>>> 42db299 (meow)
    }
}

impl AssertEq for TaikoDifficultyAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            stamina,
            rhythm,
            color,
            reading,
            great_hit_window,
            ok_hit_window,
            mono_stamina_factor,
            stars,
            max_combo,
            is_convert,
        } = self;

        assert_eq_float(*stamina, expected.stamina);
        assert_eq_float(*rhythm, expected.rhythm);
        assert_eq_float(*color, expected.color);
        assert_eq_float(*reading, expected.reading);
        assert_eq_float(*great_hit_window, expected.great_hit_window);
        assert_eq_float(*ok_hit_window, expected.ok_hit_window);
        assert_eq_float(*mono_stamina_factor, expected.mono_stamina_factor);
        assert_eq_float(*stars, expected.stars);
        assert_eq!(*max_combo, expected.max_combo);
        assert_eq!(*is_convert, expected.is_convert);
=======
        assert_eq_float(self.stamina, expected.stamina);
        assert_eq_float(self.rhythm, expected.rhythm);
        assert_eq_float(self.color, expected.color);
        assert_eq_float(self.peak, expected.peak);
        assert_eq_float(self.great_hit_window, expected.great_hit_window);
        assert_eq_float(self.ok_hit_window, expected.ok_hit_window);
        assert_eq_float(self.stars, expected.stars);
        assert_eq!(self.max_combo, expected.max_combo);
        assert_eq!(self.is_convert, expected.is_convert);
>>>>>>> 42db299 (meow)
    }
}

impl AssertEq for CatchDifficultyAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            stars,
            ar,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            is_convert,
        } = self;

        assert_eq_float(*stars, expected.stars);
        assert_eq_float(*ar, expected.ar);
        assert_eq!(*n_fruits, expected.n_fruits);
        assert_eq!(*n_droplets, expected.n_droplets);
        assert_eq!(*n_tiny_droplets, expected.n_tiny_droplets);
        assert_eq!(*is_convert, expected.is_convert);
=======
        assert_eq_float(self.stars, expected.stars);
        assert_eq_float(self.ar, expected.ar);
        assert_eq!(self.n_fruits, expected.n_fruits);
        assert_eq!(self.n_droplets, expected.n_droplets);
        assert_eq!(self.n_tiny_droplets, expected.n_tiny_droplets);
>>>>>>> 42db299 (meow)
    }
}

impl AssertEq for ManiaDifficultyAttributes {
    fn assert_eq(&self, expected: &Self) {
<<<<<<< HEAD
        let Self {
            stars,
            n_objects,
            n_hold_notes,
            max_combo,
            is_convert,
        } = self;

        assert_eq_float(*stars, expected.stars);
        assert_eq!(*n_objects, expected.n_objects);
        assert_eq!(*n_hold_notes, expected.n_hold_notes);
        assert_eq!(*max_combo, expected.max_combo);
        assert_eq!(*is_convert, expected.is_convert);
=======
        assert_eq_float(self.stars, expected.stars);
        assert_eq_float(self.hit_window, expected.hit_window);
        assert_eq!(self.n_objects, expected.n_objects);
        assert_eq!(self.max_combo, expected.max_combo);
        assert_eq!(self.is_convert, expected.is_convert);
>>>>>>> 42db299 (meow)
    }
}
