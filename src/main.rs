//
// IMPORTS
//

use std::io;

//
// DANISH LANGUAGE STRINGS
//

const AND: &str = "og";
const PLURAL_SUFFIX: &str = "er"; // The plural suffix of orders of magnitude like millions or billions - "million(er)" or "milliard(er)"

const MINUS: &str = "minus";
const DECIMAL_SEPERATOR: &str = "komma";

// Forms of "one"
const NEUTER_ONE: &str = "et"; // The neuter gender of "one" in Danish
const EMPH_ONE: &str = "én";   // Emphasised "one", to distinguish from indefinite article "en"

const HUNDRED: &str = "hundrede";

const NUMBER_NAMES: &[&[&str]] = &[
  &[
    "nul",
    "en",
    "to",
    "tre",
    "fire",
    "fem",
    "seks",
    "syv",
    "otte",
    "ni" // NAJNE!
  ],
  &[
    "ti",
    "elleve",
    "tolv",
    "tretten",
    "fjorten",
    "femten",
    "seksten",
    "sytten",
    "atten",
    "nitten"
  ],
  &[
    "tyve",
    "tredive",
    "fyrre",
    "halvtreds",
    "tres",
    "halvfjerds",
    "firs",
    "halvfems"
  ],
  &[
    "tusind",
    "million",
    "milliard",
    "billion",
    "billiard",
    "trillion",
    "trilliard",
    "kvadrillion",
    "kvadrilliard",
    "kvintillion",
    "kvintilliard",
    "sekstillion"
  ]
];

//
// SCRIPT STARTS HERE
//

trait DanishCompoundNumeral {
  fn danish_compound_numeral_name(&self) -> String;
}

// Returns the n'th digit of an integer
fn nth_digit(number: i128, n: u32) -> i128 {
  number / (10 as i128).pow(n - 1) % 10
}

impl DanishCompoundNumeral for i128 {
  // Returns the Danish compound numeral name of a compound number
  // (Works for non-compound numbers too)
  fn danish_compound_numeral_name(&self) -> String {
    let mut number = self.clone();

    // We do not *actually* care if a number is negative
    // So let us make it positive and deal with the negativity later :)
    let negative = number < 0;
    if negative { number *= -1; }
    let minus_string = if negative { format!("{MINUS} ") } else { String::new() };

    if number < 1000 {
      return format!("{minus_string}{}",
        // Numbers below 10 are easy, we just return their name from the list
        if number < 10 {
          (if number == 1 { NEUTER_ONE } // Except for "one", as it should be neuter gender
          else { NUMBER_NAMES[0][number as usize] }).to_string()

        // Numbers equal to or greater than 10 are more complicated, yet still relatively simple
        // We treat any such number as three digits. Sometimes requiring left-padding of zeros
        // We evaluate the hundreds' place first, then the tens' and ones' together
        } else {
          let hundreds = nth_digit(number, 3) as usize; // Digit in the hundreds' place
          let tens = nth_digit(number, 2) as usize;     // Digit in the tens' place
          let ones = nth_digit(number, 1) as usize;     // Digit in the ones' place

          format!("{}{}",
            if hundreds > 0 { // If there is something in the hundreds' place, isert it into the string
              format!("{} {HUNDRED}", if hundreds == 1 { NEUTER_ONE } else { NUMBER_NAMES[0][hundreds] } )
            } else { String::new() }, // Else insert an empty string
            format!("{}{}",
              // If there is something in the hundreds' place and tens' and/or ones' place, inject an "and" after the hundreds
              if tens + ones > 0 && hundreds > 0 { format!(" {AND} ") } else { String::new() },
              {
                if tens == 0 { // If thre is nothing in the tens' place
                  (
                    if ones == 0 { "" }             // zero -> Empty string
                    else if ones == 1 { EMPH_ONE }  // one  -> Emphasised one
                    else { NUMBER_NAMES[0][ones] }  // n    -> Name of n
                  ).to_string()
                } else if tens == 1{ // Teens
                  NUMBER_NAMES[1][ones].to_string()
                } else {
                  if ones == 0 { NUMBER_NAMES[2][tens - 2].to_string() } // Only tens' place name
                  else {
                    format!("{}{AND}{}", NUMBER_NAMES[0][ones], NUMBER_NAMES[2][tens - 2]) // Compound of ones and tens
                  }
                }
              }
            )
          )
        }
      )
    }

    // At this point we must have a number that is numerically greater than or equal to 1000
    // This means we can construct a compound number by splitting it into thousands and
    // feeding the groups into this very function
    // Take the number 7_023_461 as an example. It is essentially just made up of what we call it:
    // 7 millions, 23 thousands, and 461 (ones)

    // Construct a list of digits grouped by thousands
    // The above example of 7_023_461 would for an example become
    // -> [7, 23, 461]
    let mut digits_by_thousands = vec![];
    let mut n = number.clone();
    while n > 0 {
      digits_by_thousands.push(n % 1000);
      n /= 1000;
    }

    let mut strings = vec![];
    for (i, digits) in digits_by_thousands.iter().enumerate() {
      if *digits == 0 { continue; } // If group has no digits -> continue

      // Get numeral name of digits
      let mut string = Self::danish_compound_numeral_name(digits);

      // We inject an "and" if we are on the first group and the group value is < 100
      // We also inject an "and" if there are no digits in the thousands' group
      // This is to eliminate cases of a missing stringing "and" when we have group-sized gaps in numbers like
      // 1_000_001, 1_000_000_001 or 1_000_000_000_001 etc.
      if i == 0 && (*digits < 100 || *(digits_by_thousands.get(1).unwrap_or(&1)) == 0) {
        if *digits == 1 { string = EMPH_ONE.to_string() }
        string = format!("{AND} {string}")
      }
      
      // Eliminates cases of wrong gender of definite article
      // Only "thousands" is neuter gender
      if i > 1 && *digits == 1 { string = NUMBER_NAMES[0][1].to_string(); }

      strings.push(if i > 0 { 
        format!("{string} {}{}",
          NUMBER_NAMES[3][i - 1], // Injects order of magnitude
          if i > 1 && *digits > 1 { PLURAL_SUFFIX } else { "" } // Injects plural suffix where needed. Importantly thousands' do not need a suffix
        )
      } else { string })
    }

    // Reverses the list, as up until now we have actually been working in reverse
    strings.reverse();

    // Finally we return our joined list
    // We remember to take negativity into account
    format!("{minus_string}{}", strings.join(" "))
  }
}

impl DanishCompoundNumeral for f64 {
  // Returns the Danish compound numeral name of a compound floating point number
  // (Works for non-compound numbers too)
  fn danish_compound_numeral_name(&self) -> String {
    let number = self.clone();
    let string = number.to_string();

    let before_decimal = number.floor() as i128;

    let number_split: Vec<&str> = string.split('.').collect();
    let decimals = number_split.get(1);
    if decimals.is_some() { // If there are decimals
      let mut decimals_string = String::new();

      // Essentially we are just gonna loop over each decimal and push its name to the decimals_string
      // We explicitly use the NUMBER_NAMES list as we want the *raw* number name - zero included and no care for gender
      for decimal_string in decimals.unwrap().chars() {
        let decimal = decimal_string.to_digit(10).unwrap() as usize;
        decimals_string.push_str(format!("{}, ", NUMBER_NAMES[0][decimal]).as_str());
      }

      // This is bad, but it eliminates trailing ", "
      decimals_string.pop();
      decimals_string.pop();

      // Finally return the two strings seperated by a decimal seperator
      format!("{} {DECIMAL_SEPERATOR} {decimals_string}", before_decimal.danish_compound_numeral_name())
    } else { // If there are no decimals, just return the floored integer
      before_decimal.danish_compound_numeral_name()
    }
  }
}

fn main() {
  println!("Get the Danish compound numeral name of number:");
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let trimmed = input.trim();
  match trimmed.parse::<f64>() {
    Ok(number) => println!("{}", number.danish_compound_numeral_name()),
    Err(..) => println!("Invalid input. Expected input of type f64")
  }

  println!("");
  main()
}
