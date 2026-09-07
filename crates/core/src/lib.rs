/// Adds one to the input. The smoke specimen's stable surface.
pub fn bump(value: u8) -> u8 {
    value.wrapping_add(1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn bumps() {
        assert_eq!(super::bump(1), 2);
    }
}
