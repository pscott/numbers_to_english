static ZERO_TO_TEN: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static TEN_TO_TWENTY: [&str; 10] = [
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
static CATEGORIES: [&str; 7] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

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

pub fn number_to_english(mut num: u64) -> String {
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
            Some(_) => format!("{}, ", item),
            None => item.to_owned(),
        };
        english.push_str(append.as_str());
    }
    english
}