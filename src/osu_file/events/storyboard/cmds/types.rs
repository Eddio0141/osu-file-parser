use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct ContinuingFields<T> {
    pub(crate) start: (T, T),
    pub(crate) continuing: Vec<(T, Option<T>)>,
}

impl<T> ContinuingFields<T> {
    pub fn new(
        start: (T, T),
        continuing: Vec<(T, Option<T>)>,
    ) -> Result<Self, InvalidSecondFieldOption> {
        // error if continuing 2nd field is None without it being at the end of the list
        if continuing
            .iter()
            .enumerate()
            .any(|(i, (_, field_2))| field_2.is_none() && i != continuing.len() - 1)
        {
            Err(InvalidSecondFieldOption)
        } else {
            Ok(Self { start, continuing })
        }
    }

    pub fn start_values(&self) -> &(T, T) {
        &self.start
    }

    pub fn start_values_mut(&mut self) -> &mut (T, T) {
        &mut self.start
    }

    pub fn continuing_fields(&self) -> &[(T, Option<T>)] {
        &self.continuing
    }

    pub fn push_continuing_fields(&mut self, continuing_fields: (T, Option<T>))
    where
        T: std::marker::Copy,
    {
        // if the last continuing field 1 is None, its the equalivant of having the previous index's positition 1 (or the start 1 if no elements)
        if let Some(last_continuing) = self.continuing.last() {
            if last_continuing.1.is_none() {
                // find last 1 field
                let last_field = if let Some(last_continuing_with_1) =
                    self.continuing.get(self.continuing.len() - 2)
                {
                    last_continuing_with_1.1.as_ref()
                } else {
                    // backup field 1 is the starting one
                    Some(&self.start.1)
                };

                self.continuing.last_mut().unwrap().1 = last_field.copied();
            }
        }
        self.continuing.push(continuing_fields);
    }

    pub fn set_continuing_fields(
        &mut self,
        index: usize,
        fields: (T, Option<T>),
    ) -> Result<(), ContinuingSetError> {
        // if index isn't the last index, field 1 being none will return an error
        if index != self.continuing.len() - 1 && fields.1.is_none() {
            Err(ContinuingSetError::InvalidSecondFieldOption)
        } else {
            match self.continuing.get_mut(index) {
                Some(continuing_position_xy) => {
                    *continuing_position_xy = fields;
                    Ok(())
                }
                None => Err(ContinuingSetError::IndexOutOfBounds),
            }
        }
    }
}

impl<T> Display for ContinuingFields<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = vec![self.start.0.to_string(), self.start.1.to_string()];

        for fields in &self.continuing {
            builder.push(fields.0.to_string());
            if let Some(field) = &fields.1 {
                builder.push(field.to_string());
            }
        }

        write!(f, "{}", builder.join(","))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Colours {
    pub start: (u8, u8, u8),
    pub continuing: Vec<(u8, Option<u8>, Option<u8>)>,
}

impl Colours {
    pub fn new(
        start: (u8, u8, u8),
        continuing: Vec<(u8, Option<u8>, Option<u8>)>,
    ) -> Result<Self, InvalidColourFieldOption> {
        for (i, (_, green, blue)) in continuing.iter().enumerate() {
            if i != continuing.len() - 1 {
                if green.is_none() {
                    return Err(InvalidColourFieldOption::Green);
                } else if blue.is_none() {
                    return Err(InvalidColourFieldOption::Blue);
                }
            }
            if green.is_none() && blue.is_some() {
                return Err(InvalidColourFieldOption::Blue);
            }
        }
        Ok(Self { start, continuing })
    }

    pub fn start_rgb(&self) -> &(u8, u8, u8) {
        &self.start
    }

    pub fn start_rgb_mut(&mut self) -> &mut (u8, u8, u8) {
        &mut self.start
    }

    pub fn continuing_fields(&self) -> &[(u8, Option<u8>, Option<u8>)] {
        &self.continuing
    }

    pub fn push_continuing_rgbs(&mut self, continuing_fields: (u8, Option<u8>, Option<u8>)) {
        if let Some(last_continuing) = self.continuing.last() {
            if last_continuing.1.is_none() {
                // find last field
                let last_field = if let Some(last_continuing_with_1) =
                    self.continuing.get(self.continuing.len() - 2)
                {
                    last_continuing_with_1.1.as_ref()
                } else {
                    // backup field 1 is the starting one
                    Some(&self.start.1)
                };

                self.continuing.last_mut().unwrap().1 = last_field.copied();
            } else if last_continuing.2.is_none() {
                // find last field
                let last_field = if let Some(last_continuing_with_2) =
                    self.continuing.get(self.continuing.len() - 2)
                {
                    last_continuing_with_2.2.as_ref()
                } else {
                    // backup field 2 is the starting one
                    Some(&self.start.2)
                };

                self.continuing.last_mut().unwrap().2 = last_field.copied();
            }
        }
        self.continuing.push(continuing_fields);
    }

    pub fn set_continuing_rgbs(
        &mut self,
        index: usize,
        fields: (u8, Option<u8>, Option<u8>),
    ) -> Result<(), ContinuingRGBSetError> {
        // if index isn't the last index, b or g being none will return an error
        let index_is_last = index == self.continuing.len() - 1;

        if !index_is_last && fields.1.is_none() {
            Err(ContinuingRGBSetError::InvalidFieldOption(
                InvalidColourFieldOption::Green,
            ))
        } else if (!index_is_last && fields.2.is_none())
            || (fields.1.is_none() && fields.2.is_some())
        {
            Err(ContinuingRGBSetError::InvalidFieldOption(
                InvalidColourFieldOption::Blue,
            ))
        } else {
            match self.continuing.get_mut(index) {
                Some(continuing) => {
                    *continuing = fields;
                    Ok(())
                }
                None => Err(ContinuingRGBSetError::IndexOutOfBounds),
            }
        }
    }
}

impl VersionedToString for Colours {
    fn to_string(&self, _: Version) -> Option<String> {
        let mut builder = vec![
            self.start.0.to_string(),
            self.start.1.to_string(),
            self.start.2.to_string(),
        ];

        for fields in &self.continuing {
            builder.push(fields.0.to_string());
            if let Some(field) = &fields.1 {
                builder.push(field.to_string());
            }
            if let Some(field) = &fields.2 {
                builder.push(field.to_string());
            }
        }

        Some(builder.join(","))
    }
}
