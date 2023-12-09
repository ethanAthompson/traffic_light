# Rust Traffic Light

## Table of Contents

  * [Introduction](#introduction)
  * [Goals](#goals)
  * [How does this program synchronize morse code with the physical buzzer?](#how-does-this-program-synchronize-morse-code-with-the-physical-buzzer)
  * [Summary](#summary)

## Introduction
This is an implementation of a traffic light system using led lights and a buzzer.


## How does this program synchronize morse code with the physical buzzer?
```rust
    let mut buzzer = Buzzer::new(GPIO17);
    let morse_code = MorseCode::new("Welcome!");
    SuperBuzzer::play_sound(&mut buzzer, morse_code);  
```
> its all dits (.) or dahs (-) and sometimes daaihs


## Goals
- [ ] Visual Traffic light implementation that syncs in real time
- [ ] Morse Coded Traffic Light
- [ x ] Morse Code Support

## Summary
We went over how to program a traffic light system for the raspbery GPIO pin layout.
Check your morse code here: [Morse Code](https://morsecode.world/international/translator.html)