#![allow(dead_code, unused)]
use crate::board::*;
use std::collections::HashMap;

pub struct Player { // implements a player - tracks its state + prompts for decisions
    pub state: PlayerState,
    pub controller: Box<dyn PlayerController>,
}

pub struct PlayerState { //implements a player state - everything he owns/has or is unique to it
    pub number: PlayerNumber,

    pub victory_points: u8,
    pub resources: HashMap<ResourceType, u8>,

    pub settlements: Vec<u8>, // holds ids
    pub cities: Vec<u8>, // holds ids
    pub roads: Vec<u8>, // holds ids
    pub ports: Vec<ResourceType>, // for trade info

    pub longest_road: u8, // length of longest road
    pub largest_army: u8, // size of largest army
    pub has_longest_road: bool, // the VP card
    pub has_largest_army: bool, // the VP card

    pub developing_card: DevelopmentCardType,
    pub developed_cards: Vec<DevelopmentCardType>,
}

impl PlayerState { // helper method
    pub fn resource_count(&self) -> u8 { // sum of all resources for robber logic
        self.resources.values().sum()
    }
}

pub struct PlayerView { // everything a player can see from the whole game state, Game makes this for each player every action taken
    // TODO    
}

pub trait PlayerController { // interface for decision making
    fn respond(
        &mut self, // this doesn't need to be mutable in my case i think
        view: &PlayerView,
        decision: Decision
    ) -> usize; // index into decision.legal_responses
}

pub struct HumanController; // Human interface for decision making - GUI
impl PlayerController for HumanController {
    fn respond(
        &mut self, 
        view: &PlayerView,
        decision: Decision
    ) -> usize { // index into decision.legal_responses
        // TODO
        // draw everything 
        // read input
        0
    }
}

pub struct AIController; // AI interface for decision making - PPO
impl PlayerController for AIController {
    fn respond(
        &mut self, 
        view: &PlayerView,
        decision: Decision
    ) -> usize { // index into decision.legal_responses
        // TODO
        // serialize everything into tensors
        // read from ppo network 
        // deserialize response back into index of legal responses
        0
    }
}


pub struct Decision {
    pub request: PlayerRequest, // mostly cosmetic - for UI
    pub legal_responses: Vec<PlayerResponse>,
}

pub enum PlayerRequest {
    InitialSettlement,
    InitialRoad,
    Turn, // build stuff/trade/development cards/propose player trades
    DiscardResources(u8), // discard X resources
    RespondToTrade,
    MoveRobber,
}
pub enum PlayerResponse {
    // turn decisions
    EndTurn,
    BuildSettlement(u8), // settlement id
    BuildRoad(u8), // road id
    BuildCity(u8), // settlement id
    SupplyTrade(ResourceType, ResourceType), // 1. resource to give (amount decided by port logic), 2. resource to receive
    ProposePlayerTrade(PlayerNumber, ResourceType, u8, ResourceType, u8), // REDESIGN AND IMPLEMENT LATER // 1. player to trade with, 2. resource to give, 3. amount, 4. resource to receive, 5. amount
    BuyDevelopmentCard,
    UseDevelopmentCard(DevelopmentCardType),

    // special decisions
    DiscardResources(u8), // discard X resources when prompted
    MoveRobber(u8), // hex id
    RespondToPlayerTrade(bool), // true = accept, false = reject
}
