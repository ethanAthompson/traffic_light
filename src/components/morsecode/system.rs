use rust_gpiozero::*;
use std::{fmt, thread::sleep, time::Duration};

const PAUSE: u64 = 1000;
// General Purpose Input Output Pins
const GPIO17: u8 = 17;
// const GPIO27: u8 = 27;
// const GPIO22: u8 = 22;

// Entry Point
// fn main() {
// let mut buzzer = Buzzer::new(17);
// buzzer.on();
// sleep(Duration::from_secs(5));
// buzzer.off();
// sleep(Duration::from_secs(1));
// creates lights
// let mut red_light = LED::new(GPIO17);
// let mut yellow_light = LED::new(GPIO27);
// let mut green_light = LED::new(GPIO22);

// Initialize
// red_light.off();
// yellow_light.off();
// green_light.off();

// Blink
// green_light.on();
// red_light.on();
// yellow_light.on();
// wait..

// let mut buzzer = Buzzer::new(GPIO17);
// let morse_code = MorseCode::new("Rust is a good programming language!");

// println!("{:?}", morse_code.clone());
// next -> translate morse code back into alphabet.

// SuperBuzzer::play_sound(&mut buzzer, morse_code);
// }

#[derive(Debug)]
pub enum MorseCode {}

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
pub enum SuperBuzzer {
    // .  1
    Dit,
    // -  3
    Dah,
    //    7
    Daaih,
}

/// Example:
///
/// let one_unit = 50;
/// let three_unit = one_unit * 3;
///
impl MorseCodeUnits {
    // Every number in here represents 1 millisecond
    fn unpack(&self) -> u64 {
        match self {
            MorseCodeUnits::UnitOne(unit) => unit.to_owned(),
            MorseCodeUnits::UnitThree(unit) => unit.to_owned(),
            MorseCodeUnits::UnitSeven(unit) => unit.to_owned(),
        }
    }
}

pub trait Sound {
    fn play_dit(buzz: &mut Buzzer, size: u64);
    fn play_dah(buzz: &mut Buzzer, size: u64);
    fn play_daaih(buzz: &mut Buzzer, size: u64);
}

impl fmt::Display for SuperBuzzer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuperBuzzer::Dit => write!(f, "Dit"),
            SuperBuzzer::Dah => write!(f, "Dah"),
            SuperBuzzer::Daaih => write!(f, "Daaih"),
        }
    }
}

// too redundant?
impl Sound for SuperBuzzer {
    fn play_dit(buzz: &mut Buzzer, size: u64) {
        // buzz.on();
        // Remove if too much
        // println!("{}", format!("{}", SuperBuzzer::Dit));
        sleep(Duration::from_millis(size));
        // buzz.off();
        sleep(Duration::from_millis(PAUSE));
    }

    fn play_dah(buzz: &mut Buzzer, size: u64) {
        // buzz.on();
        // Remove if too much
        // println!("{}", format!("{}", SuperBuzzer::Dah));
        sleep(Duration::from_millis(size));
        // buzz.off();
        sleep(Duration::from_millis(PAUSE));
    }

    fn play_daaih(buzz: &mut Buzzer, size: u64) {
        // buzz.on();
        // Remove if too much
        // println!("{}", format!("{}", SuperBuzzer::Daaih));
        sleep(Duration::from_millis(size));
        // buzz.off();
        sleep(Duration::from_millis(PAUSE));
    }
}

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
                _ => morse_characters.push("`"),
            }
        }

        // println!("{:?}", morse_characters);

        // copies vector
        morse_characters.clone()
    }
}
