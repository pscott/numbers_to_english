static ZERO_TO_TWENTY: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
static TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eigthy", "ninety",
];
static CATEGORIES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub struct Group {
    hundreds: u8,
    tens: u8,
    units: u8,
    category: &'static str,
}

impl Group {
    pub fn new(num: u16, category: &'static str) -> Self {
        assert!(num < 1000);

        Group {
            hundreds: (num / 100) as u8,
            tens: (num / 10 % 10) as u8,
            units: (num % 10) as u8,
            category,
        }
    }

    fn first_digit(&self) -> String {
        match self.hundreds {
            0 => String::new(),
            _ => format!("{} hundred", ZERO_TO_TWENTY[self.hundreds as usize]),
        }
    }

    fn two_last_digits(&self) -> String {
        match (self.tens, self.units) {
            (0, _) => ZERO_TO_TWENTY[self.units as usize].to_owned(),
            (1, _) => ZERO_TO_TWENTY[(self.tens * 10 + self.units) as usize].to_owned(),
            (_, 0) => TENS[self.tens as usize].to_owned(),
            (_, _) => format!(
                "{} {}",
                TENS[self.tens as usize], ZERO_TO_TWENTY[self.units as usize]
            ),
        }
    }

    pub fn humanized(&self) -> String {
        let first_digit = self.first_digit();
        let two_last_digits = self.two_last_digits();

        let english_number = match (first_digit.is_empty(), two_last_digits.is_empty()) {
            (true, true) => String::new(),
            (true, false) => two_last_digits,
            (false, true) => first_digit,
            (false, false) => format!("{} {}", first_digit, two_last_digits),
        };

        if self.category.is_empty() || english_number.is_empty() {
            english_number
        } else {
            format!("{} {}", english_number, self.category)
        }
    }
}

pub fn number_to_english(mut num: u64) -> String {
    let mut vec: Vec<String> = vec![];
    let mut category = 0;
    while num != 0 {
        let slice = (num % 1000) as u16;
        let group = Group::new(slice, CATEGORIES[category]);
        vec.push(group.humanized());
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
            Some(_) => format!("{}, ", item),
            None => item.to_owned(),
        };
        english.push_str(append.as_str());
    }
    english
}
