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

    current_player: usize, // index for players through players_ordering
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

    pub fn create_view(&self, p_index: usize) -> &PlayerView { // all available info for player -> used for observation tensor / GUI

        &PlayerView { 
            // TODO
        }
    }

    pub fn round(&mut self) { // all players have a turn
        self.current_player = 0; 

        // handle starting rounds
        if self.round == 0 { // first round
            for _ in 0..self.players_ordering.len() {
                self.starting_turn();
                self.current_player += 1;
                self.round += 1;
                return;
            }
        } else if self.round == 1 { // second round
            for _ in 0..self.players_ordering.len() {
                self.current_player = self.players_ordering.len() - 1;
                self.starting_turn(); 
                if self.rules.collect_start_resources { self.collect_starting_resources(); }
                self.current_player = self.current_player - 1;
                self.round += 1;
                return;
            }
        }

        // regular rounds
        for _ in 0..self.players_ordering.len() {
            self.turn();
            self.current_player += 1;
            self.round += 1;
        }
    }

    pub fn starting_turn(&mut self) {
        // choose free settlement
        self.players[ self.players_ordering[self.current_player] ].controller.respond(
            self.create_view(self.current_player), 
            Decision { 
                request: (PlayerRequest::InitialSettlement), 
                legal_responses: self.list_legal_settlements(true) }
        );
        // choose free road
        self.players[ self.players_ordering[self.current_player] ].controller.respond(
            self.create_view(self.current_player), 
            Decision { 
                request: (PlayerRequest::InitialRoad), 
                legal_responses: self.list_legal_roads(true) }
        );
    }

    pub fn turn(&mut self) { // all a player does in their turn
        // production phase
        if !self.rules.turn_start_roll_dice { 
            // play development card (optional)
        }

        // roll the dice
        let roll = self.roll_dice();

        if roll == 7 {
            self.resolve_seven();
        } else {
            self.collect_resources(roll);
        }
        
        // action phase
        // trade/build stuff/development cards/spam VP cards
        // check win condition
    }

    pub fn roll_dice(&mut self) -> u8 {
        // roll of dice
        let mut dice1: u8 = self.rng.random_range(1..=6);
        let mut dice2: u8 = self.rng.random_range(1..=6);

        while dice1 + dice2 == 7 && !self.rules.robber { // if not playing with robber, roll until not 7
            dice1 = self.rng.random_range(1..=6);
            dice2 = self.rng.random_range(1..=6);
        }

        // return sum
        dice1 + dice2
    }

    pub fn collect_resources(&mut self, roll: u8) {
        // collect resources
        for hex in self.board.hexes.iter() {
            if roll != hex.dice_number { continue; }

            if hex.id == self.board.robber { continue; } // if robber on hex dont give resources

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
    }

    pub fn collect_starting_resources(&mut self) {
        // collect starting resources
    }

    pub fn resolve_seven(&mut self) {
        // prompt players to give away resources
        for p in 0..self.players.len() {
            let n_resources = self.players[ self.players_ordering[p] ].state.resources.values().sum::<u8>();
            if n_resources <= 7 { continue; }

            let mut n_resources_to_discard = n_resources / 2;
            
            for i in 0..n_resources_to_discard {
                let actions = vec![ResourceType::Wheat, ResourceType::Sheep, ResourceType::Wood, ResourceType::Brick, ResourceType::Ore]// each resource type
                            .into_iter()
                            .filter(|r| self.players[ self.players_ordering[p] ].state.resources[r] > 0)
                            .map(|r| PlayerResponse::DiscardResource(r))
                            .collect();

                let action = self.players[ self.players_ordering[p] ].controller.respond(
                    self.create_view(i as usize), 
                    Decision { 
                        request: PlayerRequest::DiscardResources(n_resources_to_discard - i), 
                        legal_responses: actions
                    }
                );         

                if let PlayerResponse::DiscardResource(resource) = action {
                    *self.players[ self.players_ordering[p] ].state.resources.get_mut(&resource).unwrap() -= 1;
                }
            }
        }

        // move robber
        let actions = self.list_legal_hexes();
        let action = self.players[ self.players_ordering[self.current_player] ].controller.respond(
            self.create_view(self.current_player), 
            Decision { 
                request: PlayerRequest::MoveRobber, 
                legal_responses: actions }
        );

        if let PlayerResponse::MoveRobber(hex_id) = action {
            self.board.robber = hex_id;
        }

        // steal resource
        let mut actions = Vec::new();

        for p in 0..self.players.len() {
            for node in self.board.hexes[self.board.robber as usize].nodes.iter() {
                if self.players_ordering[p] != PlayerNumber::None // if a different player has a settlement next to robber
                    && self.players_ordering[p] != self.players_ordering[self.current_player]
                    && !actions.contains(&PlayerResponse::StealResource(p)) // and said player isn't already in actions
                {
                    actions.push(PlayerResponse::StealResource(p));
                }
            }
        }

        // return if no players to steal from
        if actions.is_empty() { return; }

        let action = self.players[ self.players_ordering[self.current_player] ].controller.respond(
            self.create_view(self.current_player), 
            Decision { 
                request: PlayerRequest::StealResource, 
                legal_responses: actions }
        );

        if let PlayerResponse::StealResource(p) = action {
            let mut choices = Vec::new();

            for (&resource, &count) in &self.players[ self.players_ordering[p] ].state.resources {
                for _ in 0..count {
                    choices.push(resource);
                }
            }

            // return if no stealable resources
            if choices.is_empty() { return; }

            let picked = choices.choose(&mut self.rng).unwrap();
            *self.players[ self.players_ordering[self.current_player] ].state.resources.get_mut(picked).unwrap() += 1;
            *self.players[ self.players_ordering[p] ].state.resources.get_mut(picked).unwrap() -= 1;
        }
    }


    // LEGALITY LOGIC
    pub fn list_legal_settlements(&self, game_start: bool) -> Vec<PlayerResponse> {
        let mut legal_settlements = Vec::new();

        if !game_start { // don't check cost if first or second round
            let cost = Building::Settlement.cost();

            for resource_type in cost.keys() {
                if self.players[ self.players_ordering[self.current_player] ].state.resources[resource_type] < cost[resource_type] {
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

            // check if player has road to node
            if node.roads.iter().any(|road| self.board.roads[*road as usize].occupant == self.players_ordering[self.current_player]) {
                legal_settlements.push(PlayerResponse::BuildSettlement(node.id));
            }
        }

        legal_settlements
    }


    pub fn list_legal_roads(&self, free_road: bool) -> Vec<PlayerResponse> {
        let mut legal_roads = Vec::new();

        if !free_road {
            let cost = Building::Road.cost();

            for resource_type in cost.keys() {
                if self.players[ self.players_ordering[self.current_player] ].state.resources[resource_type] < cost[resource_type] {
                    return legal_roads; // player broke, can't build road, return empty vector
                }
            }
        }

        for road in self.board.roads.iter() {
            if road.occupant != PlayerNumber::None { continue; } // already occupied

            // check if player has settlement next to road
            if road.nodes.iter().any(|node| self.board.nodes[*node as usize].occupant == self.players_ordering[self.current_player]) {
                legal_roads.push(PlayerResponse::BuildRoad(road.id));
            }
        }

        legal_roads
    }

    pub fn list_legal_cities(&self) -> Vec<PlayerResponse> {
        let mut legal_cities = Vec::new();

        // cost check
        let cost = Building::City.cost();
        for resource_type in cost.keys() {
            if self.players[ self.players_ordering[self.current_player] ].state.resources[resource_type] < cost[resource_type] {
                return legal_cities; // player broke, can't build city, return empty vector
            }
        }

        // has to build on settlement
        for node in self.board.nodes.iter() {
            if node.occupant == self.players_ordering[self.current_player] && !node.city { 
                legal_cities.push(PlayerResponse::BuildCity(node.id));
             }
        }

        legal_cities
    }

    pub fn list_legal_hexes(&self) -> Vec<PlayerResponse> { // for moving robber
        let mut legal_hexes = self.board.hexes.iter()
            .filter(|hex| hex.id != self.board.robber)
            .map(|hex| PlayerResponse::MoveRobber(hex.id))
            .collect();

        legal_hexes
    }

    pub fn can_buy_development_card(&self) -> Vec<PlayerResponse> {
        let mut card_legal = Vec::new();

        let cost = Building::DevelopmentCard.cost();
        for resource_type in cost.keys() {
            if self.players[ self.players_ordering[self.current_player] ].state.resources[resource_type] < cost[resource_type] {
                return card_legal; // player broke, can't buy development card, return empty vector
            }
        }

        card_legal.push(PlayerResponse::BuyDevelopmentCard);

        card_legal
    }

    pub fn list_legal_development_cards(&self) -> Vec<PlayerResponse> {
        let mut legal_development_cards = Vec::new();

        let cards = &self.players[ self.players_ordering[self.current_player] ].state.developed_cards;

        for card in cards.iter(){
            legal_development_cards.push(PlayerResponse::UseDevelopmentCard(*card));
        }

        legal_development_cards
    }

    pub fn list_legal_supply_trades(&self) -> Vec<PlayerResponse> {
        let mut legal_supply_trades = Vec::new();

        let owned_resources = &self.players[ self.players_ordering[self.current_player] ].state.resources;

        let ports = &self.players[ self.players_ordering[self.current_player] ].state.ports;

        for resource_type in owned_resources.keys() { // create trade actions for each resource possible given away
            let cost = if ports.contains(resource_type) {
                2
            } else if ports.contains(&ResourceType::Generic) {
                3
            } else {
                4
            };

            // check if player has enough of resource to trade
            if owned_resources[resource_type] < cost { continue; } 

            for resource_type_to_receive in owned_resources.keys() {
                if resource_type_to_receive != resource_type {
                    legal_supply_trades.push(PlayerResponse::SupplyTrade(*resource_type, *resource_type_to_receive));
                }
            }
            
        }

        legal_supply_trades
    }

    pub fn list_legal_player_trades(&self) -> Vec<PlayerResponse> {
        let mut legal_player_trades = Vec::new();

        // TODO, implement and deal with player trades later

        legal_player_trades
    }
}