use rust_gpiozero::Buzzer;
use std::{fmt, thread::sleep, time::Duration};

pub mod system;

/// Just here as a marker struct
#[derive(Debug)]
pub enum MorseCode {}

// General Purpose Input Output Pins
#[derive(Debug)]
pub enum ConnectedPins {
    /// IDLE
    IDLE(u64),
    /// Red Light
    GPIO17,
    /// Yellow Light
    GPIO27,
    /// Green Light
    GPIO22,
    /// Buzzer
    GPIO23,
    None,
}

/// Represents the units in a morse code dit/dah
pub enum MorseCodeUnits {
    // acts like dit
    UnitOne(u64),
    // acts like dah dah dah
    UnitThree(u64),
    // acts like dah dah dah dah dah dah dit
    UnitSeven(u64),
}

/// Represents the Dits and Dahs in morse code for buzzer
///
/// ```
///    let one_unit = 50;
///    let three_unit = one_unit * 3;
/// ```
pub enum SuperBuzzer {
    // .  1
    Dit,
    // -  3
    Dah,
    //    7
    Daaih,
}

/// Custom sounds for the buzzer (not actual)
pub trait Sound {
    fn play_dit(buzz: &mut Buzzer, size: u64);
    fn play_dah(buzz: &mut Buzzer, size: u64);
    fn play_daaih(buzz: &mut Buzzer, size: u64);
}

/// Connected pins needed this functionality too so trait it why not?
pub trait Unpack {
    fn unpack(&self) -> u64;
}

/// I needed a way to unpack the values give, just wanted to try it out
impl Unpack for MorseCodeUnits {
    /// Every number in here represents 1 millisecond
    fn unpack(&self) -> u64 {
        match self {
            Self::UnitOne(unit) => unit.to_owned(),
            Self::UnitThree(unit) => unit.to_owned(),
            Self::UnitSeven(unit) => unit.to_owned(),
        }
    }
}

/// Just some hard-encoding for the connected pins
impl Unpack for ConnectedPins {
    fn unpack(&self) -> u64 {
        match self {
            Self::IDLE(milliseconds) => milliseconds.to_owned(),
            _ => 0.to_owned(),
        }
    }
}

impl ConnectedPins {
    pub fn value(&self) -> u8 {
        match self {
            Self::GPIO17 => 17,
            Self::GPIO27 => 27,
            Self::GPIO22 => 22,
            Self::GPIO23 => 23,
            _ => 0,
        }
    }
}

/// Ok, this actually plays the sound, but the beeep!
impl SuperBuzzer {
    // in order to play sound
    fn play_sound(buzz: &mut Buzzer, codes: Vec<&str>) {
        let msg = codes.join("");
        let msg2: Vec<&str> = msg.split("").collect();
        let code: Vec<&str> = msg2.into_iter().filter(|&line| !line.is_empty()).collect();
        // println!("Generated: {:?}", code);

        // Does the Conversion DD -> S (Dit Dah) -> (Sound)
        for spot in code {
            if spot == "." {
                SuperBuzzer::play_dit(buzz, MorseCodeUnits::UnitOne(55).unpack());
            } else if spot == "-" {
                SuperBuzzer::play_dah(buzz, MorseCodeUnits::UnitThree(165).unpack());
            } else if spot == "`" {
                SuperBuzzer::play_daaih(buzz, MorseCodeUnits::UnitSeven(385).unpack());
            }
        }
    }
}

/// Special words for each enum variant specification
impl fmt::Display for SuperBuzzer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuperBuzzer::Dit => write!(f, "Dit"),
            SuperBuzzer::Dah => write!(f, "Dah"),
            SuperBuzzer::Daaih => write!(f, "Daaih"),
        }
    }
}

/// The suupa buzzer plays a sound, just time delays
impl Sound for SuperBuzzer {
    fn play_dit(buzz: &mut Buzzer, size: u64) {
        // buzz.on();
        // Remove if too much
        // println!("{}", format!("{}", SuperBuzzer::Dit));
        sleep(Duration::from_millis(size));
        // buzz.off();
        sleep(Duration::from_millis(ConnectedPins::IDLE(1000).unpack()));
    }

    fn play_dah(buzz: &mut Buzzer, size: u64) {
        // buzz.on();
        // Remove if too much
        // println!("{}", format!("{}", SuperBuzzer::Dah));
        sleep(Duration::from_millis(size));
        // buzz.off();
        sleep(Duration::from_millis(ConnectedPins::IDLE(1000).unpack()));
    }

    fn play_daaih(buzz: &mut Buzzer, size: u64) {
        // buzz.on();
        // Remove if too much
        // println!("{}", format!("{}", SuperBuzzer::Daaih));
        sleep(Duration::from_millis(size));
        // buzz.off();
        sleep(Duration::from_millis(ConnectedPins::IDLE(1000).unpack()));
    }
}

/// I had to manually push in the characters, in this impl
impl MorseCode {
    pub fn new(line: &str) -> Vec<&str> {
        let line_lowcase = line.to_lowercase();
        let characters: Vec<&str> = line_lowcase.split("").collect();
        let result: Vec<&str> = characters
            .into_iter()
            .filter(|&line| !line.is_empty())
            .collect();

        // println!("Result: {:?}", result);

        let mut morse_characters: Vec<&str> = Vec::new();

        for character in result {
            // println!("Character: {}", character);
            // Each character is converted to a morse code str
            match character {
                // Alphebet
                "a" => morse_characters.push(".-"),
                "b" => morse_characters.push("-..."),
                "c" => morse_characters.push("-.-."),
                "d" => morse_characters.push("-.."),
                "e" => morse_characters.push("."),
                "f" => morse_characters.push("..-."),
                "g" => morse_characters.push("--."),
                "h" => morse_characters.push("...."),
                "i" => morse_characters.push(".."),
                "j" => morse_characters.push(".---"),
                "k" => morse_characters.push("-.-"),
                "l" => morse_characters.push(".-.."),
                "m" => morse_characters.push("--"),
                "n" => morse_characters.push("-."),
                "o" => morse_characters.push("---"),
                "p" => morse_characters.push(".--."),
                "q" => morse_characters.push("--.-"),
                "r" => morse_characters.push(".-."),
                "s" => morse_characters.push("..."),
                "t" => morse_characters.push("-"),
                "u" => morse_characters.push("..-"),
                "v" => morse_characters.push("...-"),
                "w" => morse_characters.push(".--"),
                "x" => morse_characters.push("-..-"),
                "y" => morse_characters.push("-.--"),
                "z" => morse_characters.push("--.."),
                // Numbers
                "1" => morse_characters.push(".----"),
                "2" => morse_characters.push("..---"),
                "3" => morse_characters.push("...--"),
                "4" => morse_characters.push("....-"),
                "5" => morse_characters.push("....."),
                "6" => morse_characters.push("-...."),
                "7" => morse_characters.push("--..."),
                "8" => morse_characters.push("---.."),
                "9" => morse_characters.push("----."),
                "0" => morse_characters.push("-----"),
                // Special Characters
                "?" => morse_characters.push("..--.."),
                "!" => morse_characters.push("-.-.--"),
                "." => morse_characters.push(".-.-.-"),
                "," => morse_characters.push("--..--"),
                "/" => morse_characters.push("-..-."),
                "+" => morse_characters.push(".-.-."),
                "-" => morse_characters.push("-....-"),
                "=" => morse_characters.push("-...-"),
                ":" => morse_characters.push("---..."),
                ";" => morse_characters.push("-.-.-."),

                // Represents the long pause between words
                " " => morse_characters.push("`"),

                // Represents any emoji character
                "🤔" => morse_emoji("🤔", morse_characters.clone()),
                "🐱" => morse_emoji("🐱", morse_characters.clone()),
                "🌠" => morse_emoji("🌠", morse_characters.clone()),
                "🛰️" => morse_emoji("🛰️", morse_characters.clone()),
                "🚀" => morse_emoji("🚀", morse_characters.clone()),
                _ => morse_characters.push("?"),
            }
        }

        // println!("{:?}", morse_characters);

        // copies vector
        morse_characters.clone()
    }
}

pub fn morse_emoji(char: &str, vec: Vec<&str>) {
    let emoji = emojis::get(char).unwrap().shortcode().unwrap();
    vec.clone().push(MorseCode::new(emoji).join("").as_str())
}
