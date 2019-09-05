static ZERO_TO_TEN: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static TEN_TO_TWENTY: [&str; 9] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "nineteen",
];
static TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eigthy", "ninety",
];
static CATEGORIES: [&str; 4] = ["", "thousand", "million", "billion"];

pub struct Cluster {
    hundreds: u8,
    tens: u8,
    units: u8,
    category: &'static str,
}

impl Cluster {
    pub fn new(num: u16, category: &'static str) -> Self {
        assert!(num < 1000);

        Cluster {
            hundreds: (num / 100) as u8,
            tens: (num / 10 % 10) as u8,
            units: (num % 10) as u8,
            category,
        }
    }

    pub fn humanize(&self) -> String {
        let first_digit = match self.hundreds {
            0 => String::new(),
            _ => format!("{} hundred", ZERO_TO_TEN[self.hundreds as usize]),
        };

        let under_twenty = self.tens * 10 + self.units;

        let last_digits = if self.units == 0 {
            TENS[self.tens as usize].to_owned()
        } else if under_twenty < 10 {
            ZERO_TO_TEN[under_twenty as usize].to_owned()
        } else if under_twenty < 20 {
            TEN_TO_TWENTY[(under_twenty - 10) as usize].to_owned()
        } else {
            format!(
                "{} {}",
                TENS[self.tens as usize], ZERO_TO_TEN[self.units as usize]
            )
        };

        let english = match (first_digit.is_empty(), last_digits.is_empty()) {
            (true, true) => String::new(),
            (true, false) => last_digits,
            (false, true) => first_digit,
            (false, false) => format!("{} {}", first_digit, last_digits),
        };
        if self.category.is_empty() || english.is_empty() {
            english
        } else {
            format!("{} {}", english, self.category)
        }
    }
}

pub fn number_to_english(mut num: usize) -> String {
    let mut vec: Vec<String> = vec![];
    let mut category = 0;
    while num != 0 {
        let slice = (num % 1000) as u16;
        let cluster = Cluster::new(slice, CATEGORIES[category]);
        vec.push(cluster.humanize());
        category += 1;
        num /= 1000;
    }
    if vec.is_empty() {
        return String::from("zero");
    }

    let mut vec: Vec<String> = vec
        .into_iter()
        .filter(|word| !word.as_str().trim().is_empty())
        .collect();
    vec.reverse();
    let mut iter = vec.iter().peekable();
    let mut english = String::new();
    while let Some(item) = iter.next() {
        let append = match iter.peek() {
            Some(_) => format!("{} ", item),
            None => item.to_owned(),
        };
        english.push_str(append.as_str());
    }
    english
    // let english = vec.iter()
    // .rfold(String::new(), |mut acc, item| {
    // let group = format!("{} ", item);
    // acc.push_str(group.as_ref());
    // acc
    // });
    // english.trim().to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ten_to_twenty() {
        let res = number_to_english(12);
        assert_eq!(res, "twelve");
    }

    #[test]
    fn bigger_than_twenty() {
        let res = number_to_english(21);
        assert_eq!(res, "twenty one");
    }

    #[test]
    fn simple_tens() {
        let res = number_to_english(50);
        assert_eq!(res, "fifty");
    }

    #[test]
    fn simple_digit() {
        let res = number_to_english(5);
        assert_eq!(res, "five");
    }

    #[test]
    fn zero() {
        let res = number_to_english(0);
        assert_eq!(res, "zero");
    }

    #[test]
    fn nine_nine_nine() {
        let res = number_to_english(999);
        assert_eq!(res, "nine hundred ninety nine");
    }

    #[test]
    fn simple_hundred() {
        let res = number_to_english(200);
        assert_eq!(res, "two hundred");
    }

    #[test]
    fn simple_thousand() {
        let res = number_to_english(2000);
        assert_eq!(res, "two thousand");
    }

    #[test]
    fn thousand_and_hundreds() {
        let res = number_to_english(7800);
        assert_eq!(res, "seven thousand eight hundred");
    }

    #[test]
    fn simple_million() {
        let res = number_to_english(4_000_000);
        assert_eq!(res, "four million");
    }

    #[test]
    fn million_with_numbers() {
        let res = number_to_english(1_367_512);
        assert_eq!(
            res,
            "one million three hundred sixty seven thousand five hundred twelve"
        );
    }

    #[test]
    fn simple_billion() {
        let res = number_to_english(3_000_000_000);
        assert_eq!(res, "three billion");
    }

    #[test]
    fn billion_with_numbers() {
        let res = number_to_english(9_912_870_001);
        assert_eq!(
            res,
            "nine billion nine hundred twelve million eight hundred seventy thousand one"
        );
    }
}
