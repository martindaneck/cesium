#![allow(dead_code, unused)]
use std::collections::HashMap;
use crate::board::*;
struct Player { //implements a real life player interface - what he has, sees, and can do - controlled by Game; used for humans and ai agents
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