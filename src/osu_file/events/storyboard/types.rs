use crate::{
    osu_file::{Version, VersionedFromStr, VersionedToString},
    VersionedFrom,
};

use super::error::*;

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
    type Err = ParseTriggerTypeError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        let s = s.trim();

        match s {
            "Passing" => Ok(Some(TriggerType::Passing)),
            "Failing" => Ok(Some(TriggerType::Failing)),
            _ => match s.strip_prefix("HitSound") {
                Some(s) => match s {
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
                            return Err(ParseTriggerTypeError::TooManyHitSoundFields);
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
                                            return Err(ParseTriggerTypeError::UnknownHitSoundType)
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
                None => Err(ParseTriggerTypeError::UnknownTriggerType),
            },
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
            TriggerType::Passing => "Passing".to_string(),
            TriggerType::Failing => "Failing".to_string(),
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
    type Err = ParseSampleSetError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "All" => Ok(Some(SampleSet::All)),
            "Normal" => Ok(Some(SampleSet::Normal)),
            "Soft" => Ok(Some(SampleSet::Soft)),
            "Drum" => Ok(Some(SampleSet::Drum)),
            _ => Err(ParseSampleSetError::UnknownVariant),
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
    type Err = ParseAdditionError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "Whistle" => Ok(Some(Addition::Whistle)),
            "Finish" => Ok(Some(Addition::Finish)),
            "Clap" => Ok(Some(Addition::Clap)),
            _ => Err(ParseAdditionError::UnknownVariant),
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
    Other(usize),
}

impl VersionedFrom<usize> for Easing {
    fn from(value: usize, _: Version) -> Option<Self> {
        match value {
            0 => Some(Easing::Linear),
            1 => Some(Easing::EasingOut),
            2 => Some(Easing::EasingIn),
            3 => Some(Easing::QuadIn),
            4 => Some(Easing::QuadOut),
            5 => Some(Easing::QuadInOut),
            6 => Some(Easing::CubicIn),
            7 => Some(Easing::CubicOut),
            8 => Some(Easing::CubicInOut),
            9 => Some(Easing::QuartIn),
            10 => Some(Easing::QuartOut),
            11 => Some(Easing::QuartInOut),
            12 => Some(Easing::QuintIn),
            13 => Some(Easing::QuintOut),
            14 => Some(Easing::QuintInOut),
            15 => Some(Easing::SineIn),
            16 => Some(Easing::SineOut),
            17 => Some(Easing::SineInOut),
            18 => Some(Easing::ExpoIn),
            19 => Some(Easing::ExpoOut),
            20 => Some(Easing::ExpoInOut),
            21 => Some(Easing::CircIn),
            22 => Some(Easing::CircOut),
            23 => Some(Easing::CircInOut),
            24 => Some(Easing::ElasticIn),
            25 => Some(Easing::ElasticOut),
            26 => Some(Easing::ElasticHalfOut),
            27 => Some(Easing::ElasticQuarterOut),
            28 => Some(Easing::ElasticInOut),
            29 => Some(Easing::BackIn),
            30 => Some(Easing::BackOut),
            31 => Some(Easing::BackInOut),
            32 => Some(Easing::BounceIn),
            33 => Some(Easing::BounceOut),
            34 => Some(Easing::BounceInOut),
            _ => Some(Easing::Other(value)),
        }
    }
}

impl VersionedFrom<Easing> for usize {
    fn from(value: Easing, _: Version) -> Option<Self> {
        match value {
            Easing::Linear => Some(0),
            Easing::EasingOut => Some(1),
            Easing::EasingIn => Some(2),
            Easing::QuadIn => Some(3),
            Easing::QuadOut => Some(4),
            Easing::QuadInOut => Some(5),
            Easing::CubicIn => Some(6),
            Easing::CubicOut => Some(7),
            Easing::CubicInOut => Some(8),
            Easing::QuartIn => Some(9),
            Easing::QuartOut => Some(10),
            Easing::QuartInOut => Some(11),
            Easing::QuintIn => Some(12),
            Easing::QuintOut => Some(13),
            Easing::QuintInOut => Some(14),
            Easing::SineIn => Some(15),
            Easing::SineOut => Some(16),
            Easing::SineInOut => Some(17),
            Easing::ExpoIn => Some(18),
            Easing::ExpoOut => Some(19),
            Easing::ExpoInOut => Some(20),
            Easing::CircIn => Some(21),
            Easing::CircOut => Some(22),
            Easing::CircInOut => Some(23),
            Easing::ElasticIn => Some(24),
            Easing::ElasticOut => Some(25),
            Easing::ElasticHalfOut => Some(26),
            Easing::ElasticQuarterOut => Some(27),
            Easing::ElasticInOut => Some(28),
            Easing::BackIn => Some(29),
            Easing::BackOut => Some(30),
            Easing::BackInOut => Some(31),
            Easing::BounceIn => Some(32),
            Easing::BounceOut => Some(33),
            Easing::BounceInOut => Some(34),
            Easing::Other(value) => Some(value),
        }
    }
}

impl VersionedToString for Easing {
    fn to_string(&self, version: Version) -> Option<String> {
        <usize as VersionedFrom<Easing>>::from(*self, version).map(|value| value.to_string())
    }
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
    type Err = ParseParameterError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "H" => Ok(Some(Parameter::ImageFlipHorizontal)),
            "V" => Ok(Some(Parameter::ImageFlipVertical)),
            "A" => Ok(Some(Parameter::UseAdditiveColourBlending)),
            _ => Err(ParseParameterError::UnknownVariant),
        }
    }
}
