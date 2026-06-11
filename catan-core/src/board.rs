#![allow(dead_code, unused)]

use std::ops::{Index, IndexMut};
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::collections::HashMap;

use crate::player::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceType {None, Generic, Wheat, Ore, Sheep, Brick, Wood}

impl From<&str> for ResourceType {
    fn from(value: &str) -> Self {
        match value {
            "ore" => Self::Ore,
            "wood" => Self::Wood,
            "brick" => Self::Brick,
            "wheat" => Self::Wheat,
            "sheep" => Self::Sheep,
            "generic" => Self::Generic,
            _ => Self::None,
        }
    }
}

pub enum Building {
    Road,
    Settlement,
    City,
    DevelopmentCard,
}

impl Building {
    pub fn cost(&self) -> HashMap<ResourceType, u8> { 
        match self {
            Building::Road => HashMap::from([ (ResourceType::Wheat, 1), (ResourceType::Sheep, 1), ]),
            Building::Settlement => HashMap::from([ (ResourceType::Wheat, 1), (ResourceType::Sheep, 1), (ResourceType::Brick, 1), (ResourceType::Wood, 1), ]),
            Building::City => HashMap::from([ (ResourceType::Wheat, 2), (ResourceType::Ore, 3), ]),
            Building::DevelopmentCard => HashMap::from([ (ResourceType::Wheat, 1), (ResourceType::Ore, 1), (ResourceType::Sheep, 1) ]),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DevelopmentCardType {Invention, Monopoly, RoadBuilding, VictoryPoint, Knight}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerNumber {Player1, Player2, Player3, Player4, None} // stuff ownership logic & indexing for Vec<Player>

impl PlayerNumber { 
    fn index(self) -> usize {
        match self {
            PlayerNumber::Player1 => 0,
            PlayerNumber::Player2 => 1,
            PlayerNumber::Player3 => 2,
            PlayerNumber::Player4 => 3,
            PlayerNumber::None => panic!("Cannot index with None"),
        }
    }
}

impl Index<PlayerNumber> for Vec<Player> {
    type Output = Player;

    fn index(&self, index: PlayerNumber) -> &Self::Output {
        &self[index.index()]
    }
}

impl IndexMut<PlayerNumber> for Vec<Player> {
    fn index_mut(&mut self, index: PlayerNumber) -> &mut Self::Output {
        &mut self[index.index()]
    }
}

pub struct Node {
    pub id: u8, // id
    pub neighbours: Vec<u8>, // neighbouring nodes
    pub roads: Vec<u8>, // neighbouring roads
    pub hexes: Vec<u8>, // neighbouring resource hexes
    pub port: ResourceType,
    pub occupant: PlayerNumber,
    pub city: bool, 
}

pub struct Hex {
    pub id: u8,
    pub neighbours: Vec<u8>, // neighbouring hexes
    pub nodes: Vec<u8>, // neighbouring nodes
    pub resource: ResourceType,
    pub dice_number: u8,
    pub robber: bool,
}

pub struct Road {
    pub id: u8,
    pub nodes: [u8;2], // two nodes it connects
    pub occupant: PlayerNumber,
}

pub struct Supply {
    pub resources: HashMap<ResourceType, u8>,
    pub development_cards: Vec<DevelopmentCardType> // for random choosing and popping
}

pub struct Board {
    pub nodes: Vec<Node>,
    pub roads: Vec<Road>,
    pub hexes: Vec<Hex>,
    pub supply: Supply
}

impl Board {

    fn create_supply() -> Supply {
        let mut resources: HashMap<ResourceType, u8> = HashMap::new();
        resources.insert(ResourceType::Wheat, 0);
        resources.insert(ResourceType::Ore, 0);
        resources.insert(ResourceType::Sheep, 0);
        resources.insert(ResourceType::Brick, 0);
        resources.insert(ResourceType::Wood, 0);
        
        let mut development_cards: Vec<DevelopmentCardType> = Vec::new();
        for _ in 0..2 { development_cards.push(DevelopmentCardType::Invention); }
        for _ in 0..2 { development_cards.push(DevelopmentCardType::Monopoly); }
        for _ in 0..2 { development_cards.push(DevelopmentCardType::RoadBuilding); }
        for _ in 0..5 { development_cards.push(DevelopmentCardType::VictoryPoint); }
        for _ in 0..14 { development_cards.push(DevelopmentCardType::Knight); }
        // TODO: shuffle

        Supply {
            resources,
            development_cards
        }
    }
    pub fn from_json(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();
                
        // json has different naming and incomplete information, so parse manually and fill
        let mut nodes: Vec<Node> = Vec::new();
        let mut roads: Vec<Road> = Vec::new();
        let mut hexes: Vec<Hex> = Vec::new();

        // nodes
        for node in json["village_spots"].as_array().unwrap() {
            let id = node["id"].as_u64().unwrap() as u8;
            let neighbours: Vec<u8> = vec![]; // filled when loading roads
            let roads: Vec<u8> = node["adjacent_roads"].as_array().unwrap()
                .iter()
                .map(|n| n.as_u64().unwrap() as u8)
                .collect();
            let hexes: Vec<u8> = node["adjacent_hexes"].as_array().unwrap()
                .iter()
                .map(|n| n.as_u64().unwrap() as u8)
                .collect();
            let port = ResourceType::from(node["port"].as_str().unwrap());
            let occupant = PlayerNumber::None;
            let city = false;

            nodes.push(Node {
                id,
                neighbours,
                roads,
                hexes,
                port,
                occupant,
                city
            });
        }

        // roads
        for road in json["roads"].as_array().unwrap() {
            let id = road["id"].as_u64().unwrap() as u8;
            let node1 = road["village_ids"][0].as_u64().unwrap() as u8;
            let node2 = road["village_ids"][1].as_u64().unwrap() as u8;
            let occupant = PlayerNumber::None;
            
            roads.push(Road {
                id,
                nodes: [node1, node2],
                occupant
            });

            // link nodes
            nodes[node1 as usize].neighbours.push(node2);
            nodes[node2 as usize].neighbours.push(node1);
        }

        
        // hexes
        for hex in json["resource_hexes"].as_array().unwrap() {
            let id = hex["id"].as_u64().unwrap() as u8;
            let neighbours: Vec<u8> = hex["adjacent_hexes"].as_array().unwrap()
                .iter()
                .map(|n| n.as_u64().unwrap() as u8)
                .collect();
            let nodes: Vec<u8> = hex["village_spots"].as_array().unwrap()
                .iter()
                .map(|n| n.as_u64().unwrap() as u8)
                .collect();
            let resource = ResourceType::from(hex["type"].as_str().unwrap());
            let dice_number = hex["dice_number"].as_u64().unwrap() as u8;
            let mut robber = resource == ResourceType::None; // desert        

            hexes.push(Hex {
                id,
                neighbours,
                nodes,
                resource,
                dice_number,
                robber
            });
        }

        // supply
        let supply = Board::create_supply();


        Self {
            nodes,
            roads,
            hexes,
            supply
        }
    }
}



// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_data(){
        let board = Board::from_json("data/board.json");

        assert_eq!(board.nodes.len(), 54);

        assert_eq!(board.nodes[52].hexes.len(), 1);
        assert_eq!(board.nodes[52].port, ResourceType::Generic);
        assert_eq!(board.hexes[ board.nodes[52].hexes[0] as usize ].resource, ResourceType::Ore);
        assert_eq!(board.hexes[ board.nodes[52].hexes[0] as usize ].dice_number, 10);

        assert_eq!(board.roads[0].nodes[0], 0);
        assert_eq!(board.roads[0].nodes[1], 1);

        assert_eq!(board.nodes[0].hexes.len(), 3);
        assert_eq!(board.nodes[0].neighbours, vec![1, 5, 6]);
    }
}