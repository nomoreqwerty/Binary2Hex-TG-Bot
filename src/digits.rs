pub mod converting {
    use std::io::ErrorKind;

    // This is a mask that represents all possible digits in up to 50 decimal system
    pub const DIGITMASK: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    pub fn any_whole_to_decimal(
        num: &str,
        num_base: i128,
    ) -> Result<String, ErrorKind> {
        let mut result: i128 = 0;
        let len = num.len();

        for (index, symb) in num.char_indices() {
            if let Some(dig_index) = DIGITMASK.find(symb) {
                result += dig_index as i128 * num_base.pow((len - index - 1) as u32);
            } else {
                return Err(ErrorKind::InvalidData);
            }
        }

        Ok(result.to_string())
    }

    pub fn any_fractional_to_decimal(
        num: &str,
        num_base: i32,
    ) -> Result<String, ErrorKind> {
        let mut num = num.to_owned();
        let dot: usize =
            if let Some(dot_index) = num.find('.') {
                num.remove(dot_index);
                dot_index
            } else {
                return Err(ErrorKind::InvalidData);
            };
        let mut result: f64 = 0.0;

        for (index, symb) in num[..dot].char_indices()  {
            if let Some(dig) = DIGITMASK.find(symb) {
                result += dig as f64 * num_base.pow((dot - index - 1) as u32) as f64;
            } else {
                return Err(ErrorKind::InvalidData);
            }
        }

        for (index, symb) in num[dot..].char_indices()  {
            if let Some(dig) = DIGITMASK.find(symb) {
                result += dig as f64 / num_base.pow(index as u32 + 1) as f64;
            } else {
                return Err(ErrorKind::InvalidData);
            }
        }

        Ok(result.to_string())
    }

    pub fn decimal_whole_to_any(
        num: &str,
        base_to: i32,
    ) -> Result<String, ErrorKind> {
        let mut num =
            if let Ok(num) = num.parse::<i128>() { num }
            else { return Err(ErrorKind::InvalidData); };
        let mut result = String::new();

        while num > 0 {
            result.insert(0, DIGITMASK.chars().nth((num % base_to as i128) as usize).unwrap());
            num /= base_to as i128;
        }

        Ok(result)
    }

    pub fn decimal_fractional_to_any(
        num: &str,
        base_to: i32,
        mantissa_length: Option<i32>
    ) -> Result<String, ErrorKind> {
        let whole_num_part = (num.parse::<f64>().unwrap() as i32).to_string();
        let converted_whole_part = decimal_whole_to_any(&whole_num_part, base_to).unwrap();
        let mut fractional_part = fractional_part_float(num.parse::<f64>().unwrap());
        let mut converted_fractional_part = String::new();

        let mut length: i32 = 10;
        if let Some(len) = mantissa_length { length = len }

        for _ in 0..length {
            let left_side = (fractional_part * base_to as f64) as i32;
            converted_fractional_part
                .push(DIGITMASK.chars().nth(left_side as usize).unwrap());
            fractional_part = fractional_part_float(fractional_part * base_to as f64);
        }

        let result = format!("{}.{}", converted_whole_part, converted_fractional_part);
        let mut result = result.trim_end_matches('0').to_owned();
        if result.find('.') == Some(0) { result.insert(0, '0') }

        Ok(result)
    }

    pub fn any_whole_to_any(
        num: &str,
        num_base: i32,
        base_to: i32,
    ) -> Result<String, ErrorKind> {
        let integer_decimal = if let Ok(number) = any_whole_to_decimal(num, num_base.into()) {
            number
        } else {
            return Err(ErrorKind::InvalidData);
        };

        decimal_whole_to_any(
            &integer_decimal,
            base_to
        )
    }

    pub fn any_fractional_to_any(
        num: &str,
        num_base: i32,
        base_to: i32,
        mantissa_length: Option<i32>,
    ) -> Result<String, ErrorKind> {
        let fractional_decimal = if let Ok(number) = any_fractional_to_decimal(num, num_base) {
            number
        } else {
            return Err(ErrorKind::InvalidData);
        };

        decimal_fractional_to_any(
            &fractional_decimal,
            base_to,
            mantissa_length,
        )
    }

    /// Returns the fractional part of a float number.
    ///
    /// **Example:**
    /// ```
    /// fractional_part_float(123.456) // 0.456
    /// ```
    fn fractional_part_float(num: f64) -> f64 {
        let whole_part = num as i32 as f64;
        num - whole_part
    }
}