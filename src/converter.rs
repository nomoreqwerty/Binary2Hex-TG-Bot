use crate::digits::converting;
use crate::digits::converting::DIGITMASK;

/// Class that performs different base conversions of numbers.
pub struct Converter {
    pub result: Option<String>,
    pub error_kind: Option<ValidErrorKind>,
}

impl Converter {
    pub fn convert_from_message(text: &str) -> Self {
        let all_elems: Vec<&str> = text.split(' ').collect::<Vec<&str>>();

        // Checks if the input is formatted correctly. //
        if all_elems.len() > 3 {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidFormat) }
        }
        if all_elems.len() < 3 {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidFormat) }
        }
        if all_elems[1] != ">" {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidFormat) }
        }

        let initial: Vec<&str> = all_elems[0].split('_').collect::<Vec<&str>>();
        let condition: Vec<&str> = all_elems[2].split('_').collect::<Vec<&str>>();
        let result: String;

        if initial.len() < 2 || initial.len() > 2 {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidInitialNumber) }
        }
        if condition.len() > 2 {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidCondition) }
        }
        // End of checking. //

        let initial_number: String = initial[0].to_owned();
        let initial_base: i32;
        let mut dot_count = 0u8;

        if let Ok(base) = initial[1].parse::<i32>() {
            initial_base = base;
        } else {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidInitialBase) }
        }

        if !(2..=36).contains(&initial_base) {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidInitialBase) }
        }

        for char in initial_number.to_uppercase().chars() {
            if let Some(index) = DIGITMASK.find(char) {
                if index as i32 >= initial_base {
                    return Self { result: None, error_kind: Some(ValidErrorKind::InvalidInitialNumber) }
                }
            } else if char == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    return Self { result: None, error_kind: Some(ValidErrorKind::InvalidInitialNumber) }
                }
            } else{
                return Self { result: None, error_kind: Some(ValidErrorKind::InvalidInitialNumber) }
            }
        }

        let condition_base: i32;
        let mut mantissa_length: Option<i32> = None;

        if dot_count == 0 {
            if let Ok(base) = condition[0].parse::<i32>() {
                condition_base = base;
            } else {
                return Self { result: None, error_kind: Some(ValidErrorKind::InvalidCondition) }
            }
        } else if dot_count == 1 {
            if condition.len() == 1 {
                if let Ok(base) = condition[0].parse::<i32>() {
                    condition_base = base;
                } else {
                    return Self { result: None, error_kind: Some(ValidErrorKind::InvalidConditionBase) }
                }
            } else if condition.len() == 2 {
                if let Ok(base) = condition[1].parse::<i32>() {
                    condition_base = base;
                } else {
                    return Self { result: None, error_kind: Some(ValidErrorKind::InvalidConditionBase) }
                }

                if let Ok(len) = condition[0].parse::<i32>() {
                    mantissa_length = Some(len)
                } else {
                    return Self { result: None, error_kind: Some(ValidErrorKind::InvalidMantissaLength) }
                }
            } else {
                return Self { result: None, error_kind: Some(ValidErrorKind::InvalidCondition) }
            }
        } else {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidCondition) }
        }

        if !(2..=36).contains(&condition_base) {
            return Self { result: None, error_kind: Some(ValidErrorKind::InvalidConditionBase) }
        }
        if let Some(len) = mantissa_length {
            if len > 50 {
                return Self { result: None, error_kind: Some(ValidErrorKind::InvalidMantissaLength) }
            }
        }

        if dot_count == 0 {
            result =
                if let Ok(res) = converting::any_whole_to_any(&initial_number.to_uppercase(),
                                                              initial_base, condition_base)
                { res }
                else {
                    return Self { result: None, error_kind: Some(ValidErrorKind::UnknownError) }
                }
        } else if dot_count == 1 {
            result =
                if let Ok(res) = converting::any_fractional_to_any(&initial_number.to_uppercase(),
                                                                   initial_base, condition_base,
                                                                   mantissa_length)
                { res }
                else {
                    return Self { result: None, error_kind: Some(ValidErrorKind::UnknownError) }
                }
        } else {
            return Self { result: None, error_kind: Some(ValidErrorKind::UnknownError) }
        }

        Self { result: Some(result), error_kind: None }
    }
}

#[derive(PartialEq)]
pub enum ValidErrorKind {
    InvalidFormat,
    InvalidMantissaLength,
    InvalidInitialBase,
    InvalidInitialNumber,
    InvalidConditionBase,
    InvalidCondition,
    UnknownError,
}

pub mod error_messages {
    pub const INVALID_FORMAT_MESSAGE: &str =
    "Incorrect format. Try /formatguide";

    pub const INVALID_MANTISSA_LENGTH_MESSAGE: &str =
        "Invalid mantissa lenght";

    pub const INVALID_INITIAL_BASE_MESSAGE: &str =
        "Invalid number system for initial number";

    pub const INVALID_INITIAL_NUMBER_MESSAGE: &str =
        "Invalid initial number";

    pub const INVALID_CONDITION_BASE_MESSAGE: &str =
        "Invalid number system for condition";

    pub const INVALID_CONDITION_MESSAGE: &str =
        "Invalid condition";

    pub const UNKNOWN_ERROR_MESSAGE: &str =
        "An unexpected error occured...\n\
        Your last message will be sent to the developer, so he can fix it\n\
        It's fine, it's anonymous\n\
        Thank you";
}