macro_rules! versioned_inner {
    ($name:ident, $field_type:ty, $error_from_string:ty, $s_from_string:ident, $version_from_string:ident, $inner_from_string:block) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name($field_type);

        impl From<$field_type> for $name {
            fn from(t: $field_type) -> Self {
                $name(t)
            }
        }

        impl From<$name> for $field_type {
            fn from(t: $name) -> Self {
                t.0
            }
        }

        impl crate::osu_file::types::VersionedFromStr for $name {
            type Err = $error_from_string;

            fn from_str(
                $s_from_string: &str,
                $version_from_string: crate::osu_file::types::Version,
            ) -> std::result::Result<Option<Self>, Self::Err> {
                $inner_from_string
            }
        }
    };
}

macro_rules! versioned_field_to_string {
    ($name:ident, $version:ident, $v:ident, $inner:block) => {
        impl crate::osu_file::types::VersionedToString for $name {
            fn to_string(&self, $version: crate::osu_file::types::Version) -> Option<String> {
                let $v = &self.0;
                $inner
            }
        }
    };
}

macro_rules! versioned_field_default {
    ($name:ident, $version:ident, $inner:block) => {
        impl crate::osu_file::types::VersionedDefault for $name {
            fn default($version: crate::osu_file::types::Version) -> Option<Self> {
                $inner
            }
        }
    };
}

macro_rules! versioned_field {
    // full syntax
    // $name:ident, $field_type:ty, no_versions | (nothing), |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block | boolean, $default:block
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident, $version_to_string:ident| $to_string_inner:block, |$version_default:ident| $default:block) => {
        versioned_inner!($name, $field_type, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, $version_to_string, $v, $to_string_inner);
        versioned_field_default!($name, $version_default, { $default.map(|v| v.into()) });
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block, $default:expr) => {
        versioned_inner!($name, $field_type, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, $v, { Some($to_string_inner) });
        versioned_field_default!($name, _version, { Some($default.into()) });
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty,, $default:expr) => {
        versioned_inner!($name, $field_type, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, v, { Some(v.to_string()) });
        versioned_field_default!($name, _version, { Some($default.into()) });
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block,) => {
        versioned_inner!($name, $field_type, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, $v, { Some($to_string_inner) });
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty,,) => {
        versioned_inner!($name, $field_type, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, v, { Some(v.to_string()) });
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, boolean, $default:expr) => {
        versioned_inner!($name, $field_type, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, v, { Some((*v as u8).to_string()) });
        versioned_field_default!($name, _version, { Some($default.into()) });
    };
}

macro_rules! general_section_inner {
    ($(#[$outer:meta])*, $section_name:ident, $($(#[$inner:meta])*, $field:ident, $field_type:ty)*, $parse_error:ty, $default_spacing:expr, $default_version:ident, $default_field_name:ident) => {
        #[derive(Default, Debug, Clone)]
        // TODO solve this problem
        pub struct SectionSpacing {
            $(
                pub $field: Option<usize>,
            )*
        }

        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub struct $section_name {
            pub spacing: SectionSpacing,
            $(
                $(#[$inner])*
                pub $field: Option<$field_type>,
            )*
        }

        impl PartialEq for $section_name {
            fn eq(&self, other: &Self) -> bool {
                $(
                    self.$field == other.$field &&
                )*
                true
            }
        }

        impl std::hash::Hash for $section_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $(
                    self.$field.hash(state);
                )*
            }
        }

        impl $section_name {
            /// Creates a new instance, with all fields being `None`.
            pub fn new() -> Self {
                $section_name {
                    spacing: Default::default(),
                    $($field: None),*
                }
            }

            pub fn from_str(s: &str, version: crate::osu_file::types::Version) -> Result<Option<$section_name>, crate::osu_file::types::Error<$parse_error>> {
                let mut section = $section_name::new();

                let (s, fields) = crate::parsers::get_colon_field_value_lines(s).unwrap();

                if !s.trim().is_empty() {
                    // line count from fields
                    let line_count = { fields.iter().map(|(_, _, _, ws)| ws.lines().count()).sum() };

                    return Err(crate::osu_file::types::Error::new(<$parse_error>::InvalidColonSet, line_count));
                }

                let mut line_count = 0;
                let mut parsed_fields = Vec::new();

                for (name, ws, value, ws_2) in fields {
                    if parsed_fields.contains(&name) {
                        return Err(crate::osu_file::types::Error::new(ParseError::DuplicateField, line_count));
                    }

                    match name {
                        $(
                            stringify!($field_type) => {
                                section.$field = crate::osu_file::types::Error::new_from_result_into(<$field_type as crate::osu_file::types::VersionedFromStr>::from_str(value, version), line_count)?;
                                section.spacing.$field = Some(ws.len());
                            }
                        )*
                        _ => return Err(crate::osu_file::types::Error::new(ParseError::InvalidKey, line_count)),
                    }

                    line_count += ws_2.lines().count();
                    parsed_fields.push(name);
                }

                Ok(Some(section))
            }

            pub fn to_string(&self, $default_version: crate::osu_file::types::Version) -> Option<String> {
                let mut v = Vec::new();

                $(
                    if let Some(value) = &self.$field {
                        if let Some($default_field_name) = crate::osu_file::types::VersionedToString::to_string(value, $default_version) {
                            let spacing = match self.spacing.$field {
                                Some(spacing) => " ".repeat(spacing),
                                None => $default_spacing,
                            };
                            let field_name = stringify!($field_type);

                            v.push(format!("{field_name}:{spacing}{}", $default_field_name));
                        }
                    }
                )*

                Some(v.join("\n"))
            }

            /// Resets the custom spacings of the gap between the colon and the value.
            pub fn reset_spacing(&mut self) {
                $(
                    self.spacing.$field = None;
                )*
            }
        }

        impl Default for $section_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! general_section {
    (
        $(#[$outer:meta])*
        pub struct $section_name:ident {
            $(
                $(#[$inner:meta])*
                pub $field:ident: $field_type:ty,
            )*
        },
        $parse_error:ty,
        $default_spacing:expr,
    ) => {
        general_section_inner!($(#[$outer])*, $section_name, $($(#[$inner])*, $field, $field_type)*, $parse_error, { $default_spacing.to_string() }, _version, _field_name);
    };
    (
        $(#[$outer:meta])*
        pub struct $section_name:ident {
            $(
                $(#[$inner:meta])*
                pub $field:ident: $field_type:ty,
            )*
        },
        $parse_error:ty,
        $default_spacing:expr,
        $(
            {
                $version:expr,
                $($field_spacing:ident: $field_spacing_count:expr,)*
            }
        )*
    ) => {
        general_section_inner!($(#[$outer])*, $section_name, $($(#[$inner])*, $field, $field_type)*, $parse_error,
            {
                let mut spacing = $default_spacing.to_string();

                $(
                    if $version.contains(&version) {
                        $(
                            if field_name == stringify!($field_spacing) {
                                spacing = " ".repeat($field_spacing_count);
                            }
                        )*
                    }
                )*

                spacing
            },
            version,
            field_name
        );
    }
}

macro_rules! verbose_error_to_error {
    ($error_type:ty) => {
        impl From<nom::Err<nom::error::VerboseError<&str>>> for $error_type {
            fn from(err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
                match err {
                    nom::Err::Error(err) | nom::Err::Failure(err) => {
                        for (_, err) in err.errors {
                            if let nom::error::VerboseErrorKind::Context(context) = err {
                                return <$error_type as std::str::FromStr>::from_str(context)
                                    .unwrap();
                            }
                        }

                        unreachable!()
                    }
                    // should never happen
                    nom::Err::Incomplete(_) => unreachable!(),
                }
            }
        }
    };
}

pub(crate) use general_section;
pub(crate) use general_section_inner;
pub(crate) use verbose_error_to_error;
pub(crate) use versioned_field;
pub(crate) use versioned_field_default;
pub(crate) use versioned_field_to_string;
pub(crate) use versioned_inner;
