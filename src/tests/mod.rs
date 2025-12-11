
#[cfg(test)]
mod tests {
    use crate::ByteBool;

    // Tests for ByteBool struct
    #[test]
    fn test_byte_bool_new() {
        let byte_bool = ByteBool::default();
        for i in 0..=7 {
            assert_eq!(byte_bool.read(i), false);
        }
    }

    #[test]
    fn test_byte_bool_set_and_read() {
        let mut byte_bool = ByteBool::default();

        // Set a bit to true
        byte_bool.set(3, true);
        assert_eq!(byte_bool.read(3), true);

        // Set a bit to false
        byte_bool.set(5, true); // first set to true
        byte_bool.set(5, false); // then set to false
        assert_eq!(byte_bool.read(5), false);

        // Test other bits are unaffected
        for i in 0..=7 {
            if i != 3 {
                assert_eq!(byte_bool.read(i), false);
            }
        }
    }

    #[test]
    fn test_byte_bool_set_all() {
        let mut byte_bool = ByteBool::default();
        for i in 0..=7 {
            byte_bool.set(i, true);
            assert_eq!(byte_bool.read(i), true);
        }
        for i in 0..=7 {
            byte_bool.set(i, false);
            assert_eq!(byte_bool.read(i), false);
        }
    }

    #[test]
    fn test_byte_bool_toggle() {
        let mut byte_bool = ByteBool::default();

        // Toggle from 0 to 1
        byte_bool.toggle(2);
        assert_eq!(byte_bool.read(2), true);

        // Toggle from 1 to 0
        byte_bool.toggle(2);
        assert_eq!(byte_bool.read(2), false);
    }

    #[test]
    fn test_byte_bool_clear() {
        let mut byte_bool = ByteBool::default();
        for i in 0..=7 {
            byte_bool.set(i, true);
        }
        byte_bool.clear();
        for i in 0..=7 {
            assert_eq!(byte_bool.read(i), false);
        }
    }

    #[test]
    #[should_panic]
    fn test_byte_bool_read_out_of_range() {
        let byte_bool = ByteBool::default();
        byte_bool.read(8);
    }

    #[test]
    #[should_panic]
    fn test_byte_bool_set_out_of_range() {
        let mut byte_bool = ByteBool::default();
        byte_bool.set(8, true);
    }

    #[test]
    #[should_panic]
    fn test_byte_bool_toggle_out_of_range() {
        let mut byte_bool = ByteBool::default();
        byte_bool.toggle(8);
    }
}
