pub fn calculate_salary(base_salary: u32, bonus: u32) -> u32 {
    base_salary + bonus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_salary() {
        assert_eq!(calculate_salary(10, 20), 30);
    }
}
