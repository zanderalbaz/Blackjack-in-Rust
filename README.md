# Blackjack-in-Rust
REQUIREMENTS:
o Project Overview: Briefly describe the purpose of your project and its main features.
o Setup Instructions: Include steps for installation, compilation, and running the program.
o Usage Examples: Provide examples or commands for common use cases.
o Contributors & Licensing: Mention contributors and specify the license.

## Project Overview:
We wanted to learn more about game development, so we decided to create a simple game in Rust for our final project. We decided to create a simple player vs. dealer Blackjack game using Bevy. Our project consists of a startup and a gameplay UI. We implemented a full round of blackjack with the most of the core functionality. We implemented betting with chip buttons, as well as a deal button, which the player can press to start the game after betting. We implemented hit, stand, and double down for the player. We also implemented a dealer play function that plays the dealer automatically. After each round, there is a keep playing button that shows up that allows the player to continue the game. We also implemented a home button that resets the game back to the start screen. There are a few edge cases that we have not been able to implement yet, such as determining a win when neither the dealer nor player bust on a hand. We also currently have a bug where the player and dealer hands do not fully reset when the round resets. Otherwise, the player balance is updated correctly and the win/lose text is displayed correctly and the game is functional.
## Setup Instructions:
1. install rust [here](https://www.rust-lang.org/tools/install).