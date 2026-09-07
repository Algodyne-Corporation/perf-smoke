/// Adds one to the input. The smoke specimen's stable surface.
pub fn bump(value: u8) -> u8 {
    value.saturating_add(1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn bumps() {
        assert_eq!(super::bump(1), 2);
    }

    #[test]
    fn saturates_at_the_boundary() {
        assert_eq!(super::bump(255), 255);
    }
}
