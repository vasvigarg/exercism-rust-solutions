#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // Validate bases
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // Validate digits and convert to base 10
    let mut value: u128 = 0;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        value = value
            .checked_mul(from_base as u128)
            .and_then(|v| v.checked_add(digit as u128))
            .ok_or(Error::InvalidDigit(digit))?; // overflow check
    }

    // Special case for empty input or 0
    if value == 0 {
        return Ok(vec![0]);
    }

    // Convert from base 10 to `to_base`
    let mut result = Vec::new();
    let mut n = value;
    while n > 0 {
        result.push((n % to_base as u128) as u32);
        n /= to_base as u128;
    }

    result.reverse();
    Ok(result)
}
