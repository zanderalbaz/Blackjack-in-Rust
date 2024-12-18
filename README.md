# Blackjack-in-Rust

## Project Overview:

We wanted to learn more about game development, so we decided to create a simple game in Rust for our final project. We decided to create a simple player vs. dealer Blackjack game using Bevy. Our project consists of a startup and a gameplay UI. We implemented a full round of blackjack with the most of the core functionality. We implemented betting with chip buttons, as well as a deal button, which the player can press to start the game after betting. We implemented hit, stand, and double down for the player. We also implemented a dealer play function that plays the dealer automatically. After each round, there is a keep playing button that shows up that allows the player to continue the game. We also implemented a home button that resets the game back to the start screen. There are a few edge cases that we have not been able to implement yet, such as determining a win when neither the dealer nor player bust on a hand. We also currently have a bug where the player and dealer hands do not fully reset when the round resets. Otherwise, the player balance is updated correctly and the win/lose text is displayed correctly and the game is functional.

## Setup Instructions:

1. Install rust [here](https://www.rust-lang.org/tools/install).
2. Clone this repository to your local machine.
3. Navigate to the cloned repository in your command line interface.
4. Run the command `cargo build` to let the dependencies install (bevy & rand).
5. Run the command `cargo run`.
6. Enjoy a nice simple game of Blackjack!

## Usage Examples:

+ Follow setup instructions 1 through 5 and play a satisfying game of Blackjack whenever you feel bored!
    + You could play at home, or in the car, or at the beach (not recommended), or anytime anywhere, so long as you have your computer!

## Contributors & Licensing:

This project was developed collaboratively by (Alec Zamora)[https://github.com/azamora96] and (Zander Alba)[https://github.com/zanderalbaz].

This project is open source and is distributed under the (MIT License)[https://mit-license.org]. 