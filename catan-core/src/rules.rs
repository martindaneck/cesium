#![allow(dead_code, unused)]
struct Rules {
    players: u8,
    robber: bool, // if false, 7 rerolls die
    settlement_distance_rule: bool,
    infinite_resource_supply: bool, 
    public_resources: bool,
    player_trade: bool,
    win_condition_points: u8, // 10 for standard rules, 0 for player has to exhaust all buildings to build to trigger end of game
    collect_start_resources: bool, // player starts with resources collected from second placed settlement
    instant_card_use: bool // determines whether development cards are used immediately or have to be developed for a turn and kept
}

impl Rules {
    fn my_rules() -> Rules { // rules with which i play myself
        Rules {
            players: 2,
            robber: false,
            settlement_distance_rule: false,
            infinite_resource_supply: true,
            public_resources: true,
            player_trade: false, // change to true once trade is implemented
            win_condition_points: 0, // game ended when one player built all buildings
            collect_start_resources: false,
            instant_card_use: true
        }
    }

    fn standard_rules(players: u8) -> Rules { // follows catan rulebook
        Rules {
            players,
            robber: true,
            settlement_distance_rule: true,
            infinite_resource_supply: false, //
            public_resources: true,
            player_trade: false, // change to true once trade is implemented
            win_condition_points: 10, // game ended when a player achieves 10 VPs
            collect_start_resources: true,
            instant_card_use: false
        }
    }
}