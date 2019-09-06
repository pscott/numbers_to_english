#[cfg(test)]
mod test {
    use crate::converter::number_to_english;
    use crate::format::Config;

    static CONFIG: Config = Config {
        group_separator: ",",
        spacing: " ",
    };

    #[test]
    fn ten_to_twenty() {
        let res = number_to_english(12, &CONFIG);
        assert_eq!(res, "twelve");
    }

    #[test]
    fn bigger_than_twenty() {
        let res = number_to_english(21, &CONFIG);
        assert_eq!(res, "twenty-one");
    }

    #[test]
    fn simple_tens() {
        let res = number_to_english(50, &CONFIG);
        assert_eq!(res, "fifty");
    }

    #[test]
    fn simple_digit() {
        let res = number_to_english(5, &CONFIG);
        assert_eq!(res, "five");
    }

    #[test]
    fn zero() {
        let res = number_to_english(0, &CONFIG);
        assert_eq!(res, "zero");
    }

    #[test]
    fn nine_nine_nine() {
        let res = number_to_english(999, &CONFIG);
        assert_eq!(res, "nine hundred ninety-nine");
    }

    #[test]
    fn simple_hundred() {
        let res = number_to_english(200, &CONFIG);
        assert_eq!(res, "two hundred");
    }

    #[test]
    fn simple_thousand() {
        let res = number_to_english(2000, &CONFIG);
        assert_eq!(res, "two thousand");
    }

    #[test]
    fn thousand_and_hundreds() {
        let res = number_to_english(7800, &CONFIG);
        assert_eq!(res, "seven thousand, eight hundred");
    }

    #[test]
    fn simple_million() {
        let res = number_to_english(4_000_000, &CONFIG);
        assert_eq!(res, "four million");
    }

    #[test]
    fn million_with_numbers() {
        let res = number_to_english(1_367_512, &CONFIG);
        assert_eq!(
            res,
            "one million, three hundred sixty-seven thousand, five hundred twelve"
        );
    }

    #[test]
    fn simple_billion() {
        let res = number_to_english(3_000_000_000, &CONFIG);
        assert_eq!(res, "three billion");
    }

    #[test]
    fn billion_with_numbers() {
        let res = number_to_english(9_912_870_001, &CONFIG);
        assert_eq!(
            res,
            "nine billion, nine hundred twelve million, eight hundred seventy thousand, one"
        );
    }

    #[test]
    fn u64_max() {
        let res = number_to_english(std::u64::MAX, &CONFIG);
        assert_eq!(
            res,
            "eighteen quintillion, four hundred forty-six quadrillion, seven hundred forty-four trillion, seventy-three billion, seven hundred nine million, five hundred fifty-one thousand, six hundred fifteen"
        )
    }

    #[test]
    fn u32_max() {
        let res = number_to_english(u64::from(std::u32::MAX), &CONFIG);
        assert_eq!(
            res,
            "four billion, two hundred ninety-four million, nine hundred sixty-seven thousand, two hundred ninety-five"
        )
    }
}
