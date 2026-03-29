/// Adds two numbers.
#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Subtracts two numbers.
#[must_use]
pub const fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "2 + 2 should equal 4");
    }
}
