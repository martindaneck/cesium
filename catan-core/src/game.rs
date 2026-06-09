#![allow(dead_code, unused)]

use rand::prelude::*;

use crate::board::*;
use crate::rules::*;
use crate::rules::*;
use crate::player::*;

pub struct Game {
    board: Board,
    rules: Rules,
    players: Vec<Player>,

    current_player: usize, // index for players
    round: u16,

    rng: ThreadRng // for dice throwing
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::from_json("data/board.json"),
            rules: Rules::my_rules(),
            players: Vec::new(),
            current_player: 0,
            round: 0,
            rng: rand::rng()
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
                let owner = owner as *const PlayerNumber as usize;

                *self.players[owner].state.resources.get_mut(&resource).unwrap() += 1 + *city as u8;
            }
        }

        return false; // don't handle robber
    }

    pub fn resolve_seven(&mut self) {
        // prompt players to give away resources
        // move robber
    }
}