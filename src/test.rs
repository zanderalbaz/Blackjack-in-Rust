use crate::game::{bundles::PlayerBundle, components::{PlayerBalance, PlayerHand, PlayerHands, PlayerName}, resources::BalanceValue, traits::Dealable};
use crate::game::player_systems::spawn_player;

#[test]
fn test_spawn_player(){
    use bevy::prelude::*;
    use crate::game::components::Deck;

    let mut app = App::new();
    app.insert_resource(Deck::default());
    app.insert_resource(BalanceValue{value: 1000});
    app.add_systems(Update, spawn_player);

    app.update();
    let mut player_query = app.world_mut().query::<(&PlayerBalance, &PlayerHands)>();
    let mut player_found = false;

    for (balance, player_hands) in player_query.iter(&app.world()) {
        player_found = true;
        assert_eq!(balance.0, 1000.0);
        assert_eq!(player_hands.0.len(), 1);
        let player_hand = &player_hands.0[0];
        assert_eq!(player_hand.cards.len(), 2);
    }
    assert!(player_found);


}