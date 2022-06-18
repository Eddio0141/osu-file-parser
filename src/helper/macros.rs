// TODO clean up macros
macro_rules! versioned_field_from {
    ($name:ident, $field_type:ty) => {
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
    };
}

macro_rules! versioned_field_new_type {
    ($name:ident, $field_type:ty) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name(pub $field_type);
    };
}

macro_rules! versioned_field_from_string {
    ($name:ident, $error:ty, $s:ident, $version:ident, $inner:block) => {
        impl crate::osu_file::types::VersionedFromString for $name {
            type ParseError = $error;

            fn from_str(
                $s: &str,
                $version: usize,
            ) -> std::result::Result<Option<Self>, Self::ParseError> {
                $inner
            }
        }
    };
}

macro_rules! versioned_field_to_string {
    ($name:ident, $version:ident, $v:ident, $inner:block) => {
        impl crate::osu_file::types::VersionedToString for $name {
            fn to_string(&self, $version: usize) -> Option<String> {
                let $v = &self.0;
                $inner
            }
        }
    };
}

macro_rules! versioned_field_default {
    ($name:ident, $version:ident, $inner:block) => {
        impl crate::osu_file::types::VersionedDefault for $name {
            fn default($version: usize) -> Option<Self> {
                $inner
            }
        }
    };
}

macro_rules! versioned_field {
    // full syntax
    // $name:ident, $field_type:ty, no_versions | (nothing), |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block | boolean, $default:block
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident, $version_to_string:ident| $to_string_inner:block, |$version_default:ident| $default:block) => {
        versioned_field_new_type!($name, $field_type);
        versioned_field_from_string!($name, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, $version_to_string, $v, $to_string_inner);
        versioned_field_default!($name, $version_default, { $default.map(|v| v.into()) });
        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block, $default:expr) => {
        versioned_field_new_type!($name, $field_type);
        versioned_field_from_string!($name, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, $v, { Some($to_string_inner) });
        versioned_field_default!($name, _version, { Some($default.into()) });
        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty,, $default:expr) => {
        versioned_field_new_type!($name, $field_type);
        versioned_field_from_string!($name, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, v, { Some(v.to_string()) });
        versioned_field_default!($name, _version, { Some($default.into()) });
        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block,) => {
        versioned_field_new_type!($name, $field_type);
        versioned_field_from_string!($name, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, $v, { Some($to_string_inner) });
        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty,,) => {
        versioned_field_new_type!($name, $field_type);
        versioned_field_from_string!($name, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, v, { Some(v.to_string()) });
        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, boolean, $default:expr) => {
        versioned_field_new_type!($name, $field_type);
        versioned_field_from_string!($name, $parse_str_error, $s, _version, {
            $from_str_inner.map(|value| Some(Self(value)))
        });
        versioned_field_to_string!($name, _version, v, { Some((*v as u8).to_string()) });
        versioned_field_default!($name, _version, { Some($default.into()) });
        versioned_field_from!($name, $field_type);
    };
}

// TODO do we store the space after the colon?
// TODO merge with versioned_field
macro_rules! general_section {
    (
        $(#[$outer:meta])*
        pub struct $section_name:ident {
            $(
                $(#[$inner:meta])*
                pub $field:ident: $field_type:ty,
            )*
        },
        $parse_error:ty
    ) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        $(#[$outer])*
        pub struct $section_name {
            $(
                $(#[$inner])*
                pub $field: Option<$field_type>,
            )*
        }

        impl $section_name {
            /// Creates a new instance, with all fields being `None`.
            pub fn new() -> Self {
                $section_name {
                    $($field: None),*
                }
            }

            pub fn from_str(s: &str, version: usize) -> Result<Option<$section_name>, Error<$parse_error>> {
                let mut section = $section_name::new();

                let (s, fields) = get_colon_field_value_lines(s).unwrap();

                if !s.trim().is_empty() {
                    // line count from fields
                    let line_count = { fields.iter().map(|(_, _, ws)| ws.lines().count()).sum() };

                    return Err(Error::new(<$parse_error>::InvalidColonSet, line_count));
                }

                let mut line_count = 0;
                let mut parsed_fields = Vec::new();

                for (name, value, ws) in fields {
                    if parsed_fields.contains(&name) {
                        return Err(Error::new(ParseError::DuplicateField, line_count));
                    }

                    match name {
                        $(
                            stringify!($field_type) => {
                                section.$field = Error::new_from_result_into(<$field_type>::from_str(value, version), line_count)?;
                            }
                        )*
                        _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
                    }

                    line_count += ws.lines().count();
                    parsed_fields.push(name);
                }

                Ok(Some(section))
            }

            pub fn to_string(&self, version: usize) -> Option<String> {
                let mut v = Vec::new();

                $(
                    if let Some(value) = &self.$field {
                        if let Some(value) = crate::osu_file::types::VersionedToString::to_string(value, version) {
                            v.push(format!("{}: {value}", stringify!($field_type)));
                        }
                    }
                )*

                Some(v.join("\n"))
            }
        }

        impl Default for $section_name {
            fn default() -> Self {
                Self::new()
            }
        }
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
    ) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        $(#[$outer])*
        pub struct $section_name {
            $(
                $(#[$inner])*
                pub $field: Option<$field_type>,
            )*
        }

        impl $section_name {
            /// Creates a new instance, with all fields being `None`.
            pub fn new() -> Self {
                $section_name {
                    $($field: None),*
                }
            }

            pub fn from_str(s: &str, version: usize) -> Result<Option<$section_name>, Error<$parse_error>> {
                let mut section = $section_name::new();

                let (s, fields) = get_colon_field_value_lines(s).unwrap();

                if !s.trim().is_empty() {
                    // line count from fields
                    let line_count = { fields.iter().map(|(_, _, ws)| ws.lines().count()).sum() };

                    return Err(Error::new(<$parse_error>::InvalidColonSet, line_count));
                }

                let mut line_count = 0;
                let mut parsed_fields = Vec::new();

                for (name, value, ws) in fields {
                    if parsed_fields.contains(&name) {
                        return Err(Error::new(ParseError::DuplicateField, line_count));
                    }

                    match name {
                        $(
                            stringify!($field_type) => {
                                section.$field = Error::new_from_result_into(crate::osu_file::types::VersionedFromString::from_str(value, version), line_count)?;
                            }
                        )*
                        _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
                    }

                    line_count += ws.lines().count();
                    parsed_fields.push(name);
                }

                Ok(Some(section))
            }

            pub fn to_string(&self, version: usize) -> Option<String> {
                let mut v = Vec::new();

                $(
                    if let Some(value) = &self.$field {
                        if let Some(value) = crate::osu_file::types::VersionedToString::to_string(value, version) {
                            v.push(format!("{}:{value}", stringify!($field_type)));
                        }
                    }
                )*

                Some(v.join("\n"))
            }
        }

        impl Default for $section_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! verbose_error_to_error {
    ($error_type:ty) => {
        impl From<nom::Err<nom::error::VerboseError<&str>>> for $error_type {
            fn from(err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
                match err {
                    nom::Err::Error(err) | nom::Err::Failure(err) => {
                        for (_, err) in err.errors {
                            if let nom::error::VerboseErrorKind::Context(context) = err {
                                return <$error_type>::from_str(context).unwrap();
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
pub(crate) use verbose_error_to_error;
pub(crate) use versioned_field;
pub(crate) use versioned_field_default;
pub(crate) use versioned_field_from;
pub(crate) use versioned_field_from_string;
pub(crate) use versioned_field_new_type;
pub(crate) use versioned_field_to_string;
