use strum_macros::FromRepr;

use crate::osu_file::{Version, VersionedFromStr, VersionedToString};

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

impl VersionedFromStr for TriggerType {
    type Err = TriggerTypeParseError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        let s = s.trim();

        match s.strip_prefix("HitSound") {
            Some(s) => match s {
                "Passing" => Ok(Some(TriggerType::Passing)),
                "Failing" => Ok(Some(TriggerType::Failing)),
                "" => Ok(Some(TriggerType::HitSound {
                    sample_set: None,
                    additions_sample_set: None,
                    addition: None,
                    custom_sample_set: None,
                })),
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
                                0 => if let Ok(field) = SampleSet::from_str(&field, version).map(|s| s.unwrap()) {
                                    sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                1 => if let Ok(field) = SampleSet::from_str(&field, version).map(|f| f.unwrap()) {
                                    additions_sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                2 => if let Ok(field) = Addition::from_str(&field, version).map(|a| a.unwrap()) {
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

                    Ok(Some(TriggerType::HitSound {
                        sample_set,
                        additions_sample_set,
                        addition,
                        custom_sample_set,
                    }))
                }
            },
            None => Err(TriggerTypeParseError::UnknownTriggerType(s.to_string())),
        }
    }
}

impl VersionedToString for TriggerType {
    fn to_string(&self, version: Version) -> Option<String> {
        let trigger_type = match self {
            TriggerType::HitSound {
                sample_set,
                additions_sample_set,
                addition,
                custom_sample_set,
            } => format!(
                "HitSound{}{}{}{}",
                sample_set.map_or(String::new(), |s| s.to_string(version).unwrap()),
                additions_sample_set.map_or(String::new(), |s| s.to_string(version).unwrap()),
                addition.map_or(String::new(), |s| s.to_string(version).unwrap()),
                custom_sample_set.map_or(String::new(), |s| s.to_string())
            ),
            TriggerType::Passing => "HitSoundPassing".to_string(),
            TriggerType::Failing => "HitSoundFailing".to_string(),
        };

        Some(trigger_type)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum SampleSet {
    All,
    Normal,
    Soft,
    Drum,
}

impl VersionedFromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "All" => Ok(Some(SampleSet::All)),
            "Normal" => Ok(Some(SampleSet::Normal)),
            "Soft" => Ok(Some(SampleSet::Soft)),
            "Drum" => Ok(Some(SampleSet::Drum)),
            _ => Err(SampleSetParseError::UnknownVariant),
        }
    }
}

impl VersionedToString for SampleSet {
    fn to_string(&self, _: Version) -> Option<String> {
        let sample_set = match self {
            SampleSet::All => "All",
            SampleSet::Normal => "Normal",
            SampleSet::Soft => "Soft",
            SampleSet::Drum => "Drum",
        };

        Some(sample_set.to_string())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum Addition {
    Whistle,
    Finish,
    Clap,
}

impl VersionedFromStr for Addition {
    type Err = AdditionParseError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "Whistle" => Ok(Some(Addition::Whistle)),
            "Finish" => Ok(Some(Addition::Finish)),
            "Clap" => Ok(Some(Addition::Clap)),
            _ => Err(AdditionParseError::UnknownVariant),
        }
    }
}

impl VersionedToString for Addition {
    fn to_string(&self, _: Version) -> Option<String> {
        let addtion = match self {
            Addition::Whistle => "Whistle",
            Addition::Finish => "Finish",
            Addition::Clap => "Clap",
        };

        Some(addtion.to_string())
    }
}

// TODO all TryFrom and From impls into versioned types

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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum Parameter {
    ImageFlipHorizontal,
    ImageFlipVertical,
    UseAdditiveColourBlending,
}

impl VersionedToString for Parameter {
    fn to_string(&self, _: Version) -> Option<String> {
        let parameter = match self {
            Parameter::ImageFlipHorizontal => "H",
            Parameter::ImageFlipVertical => "V",
            Parameter::UseAdditiveColourBlending => "A",
        };

        Some(parameter.to_string())
    }
}

impl VersionedFromStr for Parameter {
    type Err = ParameterParseError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "H" => Ok(Some(Parameter::ImageFlipHorizontal)),
            "V" => Ok(Some(Parameter::ImageFlipVertical)),
            "A" => Ok(Some(Parameter::UseAdditiveColourBlending)),
            _ => Err(ParameterParseError::UnknownVariant),
        }
    }
}
