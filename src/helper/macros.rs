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

macro_rules! versioned_field {
    // full syntax
    // $name:ident, $field_type:ty, no_versions | (nothing), |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block | boolean, $default:block
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block, $default:expr) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name(pub $field_type);

        impl Version for $name {
            type ParseError = $parse_str_error;

            fn from_str($s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
                $from_str_inner.map(|value| Some(Self(value)))
            }

            fn to_string(&self, _: usize) -> Option<String> {
                let $v = self.0;
                Some($to_string_inner)
            }

            fn default(_: usize) -> Option<Self> {
                Some($default.into())
            }
        }

        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty,, $default:expr) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name(pub $field_type);

        impl Version for $name {
            type ParseError = $parse_str_error;

            fn from_str($s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
                $from_str_inner.map(|value| Some(Self(value)))
            }

            fn to_string(&self, _: usize) -> Option<String> {
                Some(self.0.to_string())
            }

            fn default(_: usize) -> Option<Self> {
                Some($default.into())
            }
        }

        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, |$v:ident| $to_string_inner:block,) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name(pub $field_type);

        impl Version for $name {
            type ParseError = $parse_str_error;

            fn from_str($s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
                $from_str_inner.map(|value| Some(Self(value)))
            }

            fn to_string(&self, _: usize) -> Option<String> {
                let $v = self.0;
                Some($to_string_inner)
            }

            fn default(_: usize) -> Option<Self> {
                None
            }
        }

        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty,,) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name(pub $field_type);

        impl Version for $name {
            type ParseError = $parse_str_error;

            fn from_str($s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
                $from_str_inner.map(|value| Some(Self(value)))
            }

            fn to_string(&self, _: usize) -> Option<String> {
                Some(self.0.to_string())
            }

            fn default(_: usize) -> Option<Self> {
                None
            }
        }

        versioned_field_from!($name, $field_type);
    };
    ($name:ident, $field_type:ty, no_versions, |$s:ident| $from_str_inner:block -> $parse_str_error:ty, boolean, $default:expr) => {
        #[derive(PartialEq, Debug, Clone, Eq, Hash)]
        pub struct $name(pub $field_type);

        impl Version for $name {
            type ParseError = $parse_str_error;

            fn from_str($s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
                $from_str_inner.map(|value| Some(Self(value)))
            }

            fn to_string(&self, _: usize) -> Option<String> {
                Some((self.0 as u8).to_string())
            }

            fn default(_: usize) -> Option<Self> {
                Some($default.into())
            }
        }

        versioned_field_from!($name, $field_type);
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
                            stringify!($field) => {
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
                let mut s = String::new();

                $(
                    if let Some(value) = self.$field {
                        if let Some(value) = crate::osu_file::types::Version::to_string(&value, version) {
                            s.push_str(&format!("{}:{value}", stringify!($field)));
                        }
                    }
                )*

                Some(s)
            }
        }
    };
}

pub(crate) use general_section;
pub(crate) use versioned_field;
pub(crate) use versioned_field_from;
