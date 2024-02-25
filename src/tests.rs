mod digits_test {
    use crate::digits::converting;

    #[test]
    fn any_whole_to_decimal_1_() {
        assert_eq!(converting::any_whole_to_decimal("1110001", 2).unwrap(), "113");
        assert_eq!(converting::any_whole_to_decimal("1375", 8).unwrap(), "765");
        assert_eq!(converting::any_whole_to_decimal("A8F13", 16).unwrap(), "691987");
        assert_eq!(converting::any_whole_to_decimal("A8F-3", 16).is_err(), true);
    }

    #[test]
    fn any_fractional_to_decimal_1_() {
        assert_eq!(converting::any_fractional_to_decimal("112.345", 8).unwrap(), "74.447265625");
        assert_eq!(converting::any_fractional_to_decimal("10.1221", 3).unwrap(), "3.6419753086419755");
        assert_eq!(converting::any_fractional_to_decimal("A3.F14", 16).unwrap(), "163.9423828125");
    }

    #[test]
    fn decimal_whole_to_any_1_() {
        assert_eq!(converting::decimal_whole_to_any("1014", 2).unwrap(), "1111110110");
    }

    #[test]
    fn decimal_whole_to_any_2_() {
        assert_eq!(converting::decimal_whole_to_any("31599", 8).unwrap(), "75557");
    }

    #[test]
    fn decimal_whole_to_any_3_() {
        assert_eq!(converting::decimal_whole_to_any("1680135541", 16).unwrap(), "6424D575");
    }

    #[test]
    fn decimal_fractional_to_any_1_() {
        assert_eq!(converting::decimal_fractional_to_any("113.55", 2, None).unwrap(), "1110001.1000110011");
    }

    #[test]
    fn decimal_fractional_to_any_2_() {
        assert_eq!(converting::decimal_fractional_to_any("31599.4846", 3, None).unwrap(), "1121100100.1110020211");
    }

    #[test]
    fn decimal_fractional_to_any_3_() {
        assert_eq!(converting::decimal_fractional_to_any("12.5", 2, None).unwrap(), "1100.1");
    }

    #[test]
    fn decimal_fractional_to_any_4_() {
        assert_eq!(converting::decimal_fractional_to_any("1234.5678", 16, None).unwrap(), "4D2.915B573EAB");
    }

    #[test]
    fn any_whole_to_any_1_() {
        assert_eq!(converting::any_whole_to_any("0010101011101010", 2, 16).unwrap(), "2AEA");
    }

    #[test]
    fn any_whole_to_any_2_() {
        assert_eq!(converting::any_whole_to_any("1445363", 7, 3).unwrap(), "100222101122");
    }

    #[test]
    fn any_whole_to_any_3_() {
        assert_eq!(converting::any_whole_to_any("A15BB3", 16, 8).unwrap(), "50255663");
    }

    #[test]
    fn any_fractional_to_any_1_() {
        assert_eq!(converting::any_fractional_to_any("13.5533", 8, 16, None).unwrap(), "B.B5B");
    }

    #[test]
    fn any_fractional_to_any_3_() {
        assert_eq!(converting::any_fractional_to_any("0.55357", 8, 16, None).unwrap(), "0.B5DE");
    }
}
