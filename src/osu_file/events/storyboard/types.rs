use std::fmt::Display;
use std::str::FromStr;

use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};

use super::error::*;

// TODO what is group_number
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum TriggerType {
    HitSound {
        sample_set: Option<SampleSet>,
        additions_sample_set: Option<SampleSet>,
        addition: Option<Addition>,
        custom_sample_set: Option<usize>,
    },
    Passing,
    Failing,
}

impl FromStr for TriggerType {
    type Err = TriggerTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        match s.strip_prefix("HitSound") {
            Some(s) => match s {
                "Passing" => Ok(TriggerType::Passing),
                "Failing" => Ok(TriggerType::Failing),
                "" => Ok(TriggerType::HitSound {
                    sample_set: None,
                    additions_sample_set: None,
                    addition: None,
                    custom_sample_set: None,
                }),
                _ => {
                    let fields = {
                        let mut fields = Vec::new();
                        let mut builder = String::with_capacity(256);

                        for (i, ch) in s.chars().enumerate() {
                            if i != 0 && (ch.is_uppercase() || ch.is_numeric()) {
                                fields.push(builder.to_owned());
                                builder.clear();
                            }
                            builder.push(ch);
                        }

                        fields.push(builder);

                        fields
                    };

                    // TODO for the project, make sure all fields are used in iterator next call
                    if fields.len() > 4 {
                        return Err(TriggerTypeParseError::TooManyHitSoundFields(fields.len()));
                    }

                    let mut field_parse_attempt_index = 0;

                    let mut sample_set = None;
                    let mut additions_sample_set = None;
                    let mut addition = None;
                    let mut custom_sample_set = None;

                    for field in fields {
                        loop {
                            match field_parse_attempt_index {
                                0 => if let Ok(field) = field.parse() {
                                    sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                1 => if let Ok(field) = field.parse() {
                                    additions_sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                2 => if let Ok(field) = field.parse() {
                                    addition = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                3 => if let Ok(field) = field.parse() {
                                    custom_sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                } else {
                                    return Err(TriggerTypeParseError::UnknownHitSoundType(s.to_string()))
                                }
                                _ => unreachable!("The check for field size is already done so this is impossible to reach")
                            }
                            field_parse_attempt_index += 1;
                        }
                    }

                    Ok(TriggerType::HitSound {
                        sample_set,
                        additions_sample_set,
                        addition,
                        custom_sample_set,
                    })
                }
            },
            None => Err(TriggerTypeParseError::UnknownTriggerType(s.to_string())),
        }
    }
}

impl Display for TriggerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trigger_type = match self {
            TriggerType::HitSound {
                sample_set,
                additions_sample_set,
                addition,
                custom_sample_set,
            } => format!(
                "HitSound{}{}{}{}",
                sample_set.map_or(String::new(), |s| s.to_string()),
                additions_sample_set.map_or(String::new(), |s| s.to_string()),
                addition.map_or(String::new(), |s| s.to_string()),
                custom_sample_set.map_or(String::new(), |s| s.to_string())
            ),
            TriggerType::Passing => "HitSoundPassing".to_string(),
            TriggerType::Failing => "HitSoundFailing".to_string(),
        };

        write!(f, "{trigger_type}")
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumString, Display)]
#[non_exhaustive]
pub enum SampleSet {
    All,
    Normal,
    Soft,
    Drum,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumString, Display)]
#[non_exhaustive]
pub enum Addition {
    Whistle,
    Finish,
    Clap,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromRepr)]
#[non_exhaustive]
pub enum Easing {
    Linear,
    EasingOut,
    EasingIn,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    ElasticIn,
    ElasticOut,
    ElasticHalfOut,
    ElasticQuarterOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, IntoStaticStr, EnumString, Display)]
#[non_exhaustive]
pub enum Parameter {
    #[strum(serialize = "H")]
    ImageFlipHorizontal,
    #[strum(serialize = "V")]
    ImageFlipVertical,
    #[strum(serialize = "A")]
    UseAdditiveColourBlending,
}
