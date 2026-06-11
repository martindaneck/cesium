#![allow(dead_code, unused)]
pub struct Rules {
    pub players: u8,
    pub robber: bool, // if false, 7 rerolls die
    pub settlement_distance_rule: bool,
    pub infinite_resource_supply: bool, 
    pub public_resources: bool,
    pub player_trade: bool,
    pub win_condition_points: u8, // 10 for standard rules, 0 for player has to exhaust all buildings to build to trigger end of game
    pub collect_start_resources: bool, // player starts with resources collected from second placed settlement
    pub instant_card_development: bool, // determines whether development cards are possible to be used immediately or have to be developed for a turn and kept
    pub turn_start_roll_dice: bool, // whether you can use a development card at the start of your turn
    pub one_development_card_per_turn: bool
}

impl Rules {
    pub fn my_rules() -> Rules { // rules with which i play myself
        Rules {
            players: 2,
            robber: false,
            settlement_distance_rule: false,
            infinite_resource_supply: true,
            public_resources: true,
            player_trade: false, // change to true once trade is implemented
            win_condition_points: 0, // game ended when one player built all buildings
            collect_start_resources: false,
            instant_card_development: true,
            turn_start_roll_dice: true,
            one_development_card_per_turn: false
        }
    }

    pub fn standard_rules(players: u8) -> Rules { // follows catan rulebook
        Rules {
            players,
            robber: true,
            settlement_distance_rule: true,
            infinite_resource_supply: false, //
            public_resources: true,
            player_trade: false, // change to true once trade is implemented
            win_condition_points: 10, // game ended when a player achieves 10 VPs
            collect_start_resources: true,
            instant_card_development: false,
            turn_start_roll_dice: false,
            one_development_card_per_turn: false
        }
    }
}