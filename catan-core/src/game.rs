#![allow(dead_code, unused)]

use rand::prelude::*;

use crate::board::*;
use crate::player;
use crate::rules::*;
use crate::player::*;

pub struct Game {
    board: Board,
    rules: Rules,
    players: Vec<Player>,
    players_ordering: Vec<PlayerNumber>,

    current_player: usize, // index for players
    round: u16,

    rng: ThreadRng // for dice throwing
}

impl Game {
    pub fn new() -> Game {
        let rules = Rules::my_rules();
        let mut rng = rand::rng();

        // TODO - create number of players based on rules 
        let mut players = Vec::new(); 
        // temp - work with two human players, later adjust according to rules
        players.push(Player::new(Box::new(HumanController), PlayerNumber::Player1));
        players.push(Player::new(Box::new(HumanController), PlayerNumber::Player2));

        let mut players_ordering = Vec::new();
        for player in players.iter() {
            players_ordering.push(player.state.number);
        }
        players_ordering.shuffle(&mut rng); // players play in random order

        Game {
            board: Board::from_json("data/board.json"),
            rules,
            players,
            players_ordering,
            current_player: 0,
            round: 0,
            rng
        }
    }

    pub fn create_view(&self, player: usize) -> PlayerView { // all available info for player -> used for observation tensor / GUI
        PlayerView { 
            // TODO
        }
    }

    pub fn round(&mut self) { // do while game is not over
        /// production phase
        if !self.rules.turn_start_roll_dice { 
            // play development card (optional)
        }

        // roll the dice
        let resolve_seven = self.roll_dice();

        // move robber if 7 is rolled
        if resolve_seven {
            self.resolve_seven();
        }
        
        // action phase
        // trade/build stuff/development cards/spam VP cards
        // check win condition

        // do for all players
    }

    pub fn roll_dice(&mut self) -> bool {
        // roll of dice
        let mut dice1: u8 = self.rng.random_range(1..=6);
        let mut dice2: u8 = self.rng.random_range(1..=6);

        while dice1 + dice2 == 7 && !self.rules.robber { // if not playing with robber, until not 7
            dice1 = self.rng.random_range(1..=6);
            dice2 = self.rng.random_range(1..=6);
        }

        let roll = dice1 + dice2;

        if roll == 7 {
            // handle robber
            return true;
        }

        // collect resources
        for hex in self.board.hexes.iter() {
            if roll != hex.dice_number { continue; }

            if hex.robber { continue; } // if robber on hex dont give resources

            let resource: &ResourceType = &hex.resource;

            if !self.rules.infinite_resource_supply {
                let mut amount_to_give = 0;

                for node_id in hex.nodes.iter() {
                    if self.board.nodes[*node_id as usize].occupant != PlayerNumber::None {
                        amount_to_give += 1;
                    }
                }

                if amount_to_give > self.board.supply.resources[&resource] { continue; }
            }

            for node_id in hex.nodes.iter() {
                let owner = &self.board.nodes[*node_id as usize].occupant;
                let city = &self.board.nodes[*node_id as usize].city;
                if *owner == PlayerNumber::None { continue; }

                *self.players[*owner].state.resources.get_mut(&resource).unwrap() += 1 + *city as u8;
            }
        }

        return false; // don't handle robber
    }

    pub fn resolve_seven(&mut self) {
        // prompt players to give away resources
        // move robber
    }


    // LEGALITY LOGIC
    pub fn list_legal_settelements(&self, game_start: bool) -> Vec<PlayerResponse> {
        let mut legal_settlements = Vec::new();

        if !game_start { // don't check cost if first or second round
            for resource_type in Building::Settlement.cost().keys() {
                if self.players[ self.players_ordering[self.current_player] ].state.resources[resource_type] < Building::Settlement.cost()[resource_type] {
                    return legal_settlements; // player broke, can't build settlement, return empty vector
                }
            }
        }

        for node in self.board.nodes.iter() {
            if node.occupant != PlayerNumber::None { continue; } // already occupied

            if self.rules.settlement_distance_rule { // distance rule
                if node.neighbours.iter().any(|n| self.board.nodes[*n as usize].occupant != PlayerNumber::None) {
                    continue;
                }
            }

            if game_start { // can be built anywhere, no road checking
                legal_settlements.push(PlayerResponse::BuildSettlement(node.id));
                continue;
            }

            let mut node_reachable = false;
            for road in node.roads.iter() {
                if self.board.roads[*road as usize].occupant == self.players_ordering[self.current_player] {
                    node_reachable = true;
                    break;
                }
            }

            if node_reachable {
                legal_settlements.push(PlayerResponse::BuildSettlement(node.id));
            }
        }

        legal_settlements
    }
    
}