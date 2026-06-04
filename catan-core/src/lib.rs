#![allow(dead_code, unused)]

use serde::Deserialize;
use serde_json::Value;
use std::fs::File;

#[derive(Debug, PartialEq)]
enum ResourceType {None, Generic, Wheat, Ore, Sheep, Brick, Wood}

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

enum PlayerNumber {None, Player1, Player2, Player3, Player4}

struct Node {
    id: u8, // id
    neighbours: Vec<u8>, // neighbouring nodes
    roads: Vec<u8>, // neighbouring roads
    hexes: Vec<u8>, // neighbouring resource hexes
    port: ResourceType,
    occupant: PlayerNumber,
    city: bool, 
}

struct Hex {
    id: u8,
    neighbours: Vec<u8>, // neighbouring hexes
    nodes: Vec<u8>, // neighbouring nodes
    resource: ResourceType,
    dice_number: u8,
}

struct Road {
    id: u8,
    nodes: [u8;2], // two nodes it connects
    occupant: PlayerNumber,
}

struct Board {
    nodes: Vec<Node>,
    roads: Vec<Road>,
    hexes: Vec<Hex>,
}

impl Board {
    pub fn from_json(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let json: Value = serde_json::from_reader(file).unwrap();
                
        // json is rather complicated and inconsistent, so parse manually
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

            hexes.push(Hex {
                id,
                neighbours,
                nodes,
                resource,
                dice_number
            });
        }

        Self {
            nodes,
            roads,
            hexes
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