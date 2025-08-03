use rosu_map::section::general::GameMode;

use crate::{
<<<<<<< HEAD
    any::{difficulty::skills::StrainSkill, Difficulty},
    mania::difficulty::DifficultyValues,
    model::mode::ConvertError,
    Beatmap,
=======
    any::Difficulty, mania::difficulty::DifficultyValues, model::mode::ConvertError, Beatmap,
>>>>>>> 42db299 (meow)
};

/// The result of calculating the strains on a osu!mania map.
///
/// Suitable to plot the difficulty of a map over time.
#[derive(Clone, Debug, PartialEq)]
pub struct ManiaStrains {
    /// Strain peaks of the strain skill.
    pub strains: Vec<f64>,
}

impl ManiaStrains {
    /// Time between two strains in ms.
    pub const SECTION_LEN: f64 = 400.0;
}

pub fn strains(difficulty: &Difficulty, map: &Beatmap) -> Result<ManiaStrains, ConvertError> {
    let map = map.convert_ref(GameMode::Mania, difficulty.get_mods())?;
    let values = DifficultyValues::calculate(difficulty, &map);

    Ok(ManiaStrains {
<<<<<<< HEAD
        strains: values.strain.into_current_strain_peaks().into_vec(),
=======
        strains: values.strain.get_curr_strain_peaks().into_vec(),
>>>>>>> 42db299 (meow)
    })
}
