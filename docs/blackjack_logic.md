# Blackjack Logic

#### This document will outline the logic used for the dealer and player for our implementation for the game blackjack.
---

## Game Logic
- **Hit**
: A hit consists of a player or dealer drawing a new card from the deck.
- **Stand**
: A stand consists of a player or dealer indicating that they are no longer allowed to draw a card from the deck.
- **Bust**
: A bust consists of a player or dealer's hand exceeding the value 21.
- **Aces**
: Aces can be played either as a 1 or an 11. If playing and ace as an 11 will cause the hand to bust, the ace is played as a 1.
- **Double Down**
: A double down consists of a player doubling their original bet in exchange for exactly 1 more card for a hand. After a double down, the hand must stand until the end of the round.
- **Split**
: A split consists of a player, who has a hand with two cards of the exact same face value (i.e. two 6s, two 10s, two aces, two kings, etc.) splitting their original hand into two hands, played independently. The bet for the original hand must be matched for both hands after splitting for that round.
- **Surrender**
: A surrender consists of a player voiding their hand after seeing their hand and the dealer's face up card. Half of your original bet is returned and the other half is given up to the dealer.
- **Push**
: A push consists of a player and the dealer ending the round with the same card total. In this case, the player's bet is returned.
- **Blackjack**
: A blackjack consists of a hand whose first two cards total to 21. A blackjack beats any hand that is not a blackjack, even hands that total to 21.
---



## Dealer Logic
- Must hit until their card total is 17 or higher
- Must stand if the card total is 17 or higher
- Option to toggle whether hit or stand on "soft" 17 (17 using an ace as a value of 11)
- Cannot double down, split, or surrender a hand
- Must payout remaining players on a bust
- Dealer blackjack beats all other non-blackjack hands. Pushes for player blackjack hands
---

## Player Logic
**Conditions** : The table minimum is bet, the player has not busted, and the player has enough chips to perform their desired action.
- Can hit on any hand
- Can stand on any hand
- Can surrender before hitting or standing on a hand for the first time
- Can double down on any hand
- Can split a hand when appropriate. The player is allowed to split their hand multiple times (i.e. splitting an already split hand)
- Bet is given up to the dealer no matter the circumstances in the event of a bust

### Win Logic
Note: Win logic changes in the event of a dealer or player blackjack. *See Game Logic: Blackjack*
- A bust results in an instant loss for the hand
- A win occurs when a hand that has not busted has a greater card total than the dealer
- A push occurs when a player has the exact same card total as the dealer
- A blackjack win pays out 2.5x the player's bet on that hand
- A non-blackjack win pays out 2.0x the player's bet on that hand
