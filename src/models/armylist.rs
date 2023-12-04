// TODO this should be moved in a different folder than 
// "models", more like "data" or indeed "catalogue"
use crate::models::character::Character;
use crate::models::unit::Unit;
use crate::models::support::Support;
use crate::models::element::Element;

// For serialization
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Tech,
    Enlisted,
    Conglomerate,
    Union,
}

pub struct ArmyList {
    characters : Vec<Character>,
    units: Vec<Unit>,
    supports: Vec<Support>,
}


// Various utility functions for the armylist: 
impl ArmyList {

    pub fn get_characters (&self) -> Vec<(String, u32)> {
        let mut out_vec = Vec::<(String, u32)>::new();
        for elem in &self.characters {
            out_vec.push((elem.get_name(), elem.get_points()));
        }
        out_vec
    }

    pub fn get_units (&self) -> Vec<(String, u32)> {
        let mut out_vec = Vec::<(String, u32)>::new();
        for elem in &self.units {
            out_vec.push((elem.get_name(), elem.get_points()));
        }
        out_vec
    }

    pub fn get_supports (&self) -> Vec<(String, u32)> {
        let mut out_vec = Vec::<(String, u32)>::new();
        for elem in &self.supports {
            out_vec.push((elem.get_name(), elem.get_points()));
        }
        out_vec
    }
}


// this function currently contains ALL the information regarding the various factions.
// This would be probably better implemented in separate files. TODO.
//
// Another important TODO:
// The Elements should have a get_rules that returns a closure as part of the Trait.
// This allows special rules for different units to be implemented code wise.
impl ArmyList {
    pub fn new( faction : Faction) -> ArmyList {
        let mut new_list = ArmyList {
            characters: Vec::<Character>::new(),
            units: Vec::<Unit>::new(),
            supports: Vec::<Support>::new(),
        };

        match faction {

            //// TECH
            Faction::Tech => {

                // Characters
                new_list.characters.push(Character {    name: "Sentient AI".to_string(),                            points: 3});
                new_list.characters.push(Character {    name: "Fire Control AI".to_string(),                        points: 3});
                new_list.characters.push(Character {    name: "Pilot AI".to_string(),                               points: 2});
            
                // Units
                new_list.units.push(Unit {              name: "Battle Robots".to_string(),                          points: 3});
                new_list.units.push(Unit {              name: "Heavy Robots".to_string(),                           points: 4});
                new_list.units.push(Unit {              name: "Light Spider Drones".to_string(),                    points: 2});
                new_list.units.push(Unit {              name: "Heavy Spider Drones".to_string(),                    points: 4});
                new_list.units.push(Unit {              name: "Socrates Battle Rig (Brawler)".to_string(),          points: 8});
                new_list.units.push(Unit {              name: "Socrates Battle Rig (Sharpshooter)".to_string(),     points: 8});
                new_list.units.push(Unit {              name: "Socrates Battle Rig (Demolition)".to_string(),       points: 8});
                new_list.units.push(Unit {              name: "Zeno Battle Rig".to_string(),                        points: 5});
                new_list.units.push(Unit {              name: "Solon Battle Tank (Minigun)".to_string(),            points: 6});
                new_list.units.push(Unit {              name: "Solon Battle Tank (Cannon)".to_string(),             points: 6});
                new_list.units.push(Unit {              name: "Syro Runner Rig".to_string(),                        points: 3});
                new_list.units.push(Unit {              name: "Thales Fighter".to_string(),                         points: 7});
                new_list.units.push(Unit {              name: "Gun Platform".to_string(),                           points: 7});
                new_list.units.push(Unit {              name: "Gorgias Transport".to_string(),                      points: 3});
        
                // Supports
                new_list.supports.push(Support {        name: "Satellite Uplink".to_string(),                       points: 3});
                new_list.supports.push(Support {        name: "Cluster Strike".to_string(),                         points: 5});
                new_list.supports.push(Support {        name: "Eye in the Sky".to_string(),                         points: 4});
                new_list.supports.push(Support {        name: "Jamming".to_string(),                                points: 3});
                new_list.supports.push(Support {        name: "Orbital Bombing".to_string(),                        points: 4});
                new_list.supports.push(Support {        name: "Orbital Deployment".to_string(),                     points: 2});
                new_list.supports.push(Support {        name: "Software Upgrade".to_string(),                       points: 1});
                new_list.supports.push(Support {        name: "Twin Missile Strike".to_string(),                    points: 7});
            }

            //// UNION
            Faction::Union => {
                
                // Characters
                new_list.characters.push(Character {    name: "Union Representative".to_string(),                   points: 5});
                new_list.characters.push(Character {    name: "Crew Chief".to_string(),                             points: 2});
                new_list.characters.push(Character {    name: "Demolition Man".to_string(),                         points: 3});
                new_list.characters.push(Character {    name: "Foreman".to_string(),                                points: 3});
                new_list.characters.push(Character {    name: "Mechanic".to_string(),                               points: 2});

                // Units
                new_list.units.push(Unit {              name: "Steeljacks".to_string(),                             points: 2});
                new_list.units.push(Unit {              name: "Dust Riders".to_string(),                            points: 3});
                new_list.units.push(Unit {              name: "Reclaimers".to_string(),                             points: 2});
                new_list.units.push(Unit {              name: "Tugger".to_string(),                                 points: 2});
                new_list.units.push(Unit {              name: "Gunwagon".to_string(),                               points: 4});
                new_list.units.push(Unit {              name: "Ramwagon".to_string(),                               points: 4});
                new_list.units.push(Unit {              name: "Commuter".to_string(),                               points: 5});
                new_list.units.push(Unit {              name: "Mobile Fortress".to_string(),                        points: 16});

                // Supports
                new_list.supports.push(Support {        name: "Assault Siren".to_string(),                          points: 5});
                new_list.supports.push(Support {        name: "Double Shift".to_string(),                           points: 2});
                new_list.supports.push(Support {        name: "Improvised Barrier".to_string(),                     points: 2});
                new_list.supports.push(Support {        name: "Repairing Team".to_string(),                         points: 4});
                new_list.supports.push(Support {        name: "Tunnels and Hooks".to_string(),                      points: 1});
            }

            //// CONGLOMERATE
            Faction::Conglomerate => {

                // Characters
                new_list.characters.push(Character {    name: "Lead Hunter".to_string(),                            points: 2});
                new_list.characters.push(Character {    name: "Local Lobbyst".to_string(),                          points: 5});
                new_list.characters.push(Character {    name: "Placeholder".to_string(),                            points: 1});

                // Units
                new_list.units.push(Unit {    name: "Corporate Infantry".to_string(),                               points: 2});
                new_list.units.push(Unit {    name: "Infantry AT Squad".to_string(),                                points: 3});
                new_list.units.push(Unit {    name: "Kukri Scout vehicle".to_string(),                              points: 4});
                new_list.units.push(Unit {    name: "Saber Artillery".to_string(),                                  points: 6});
                new_list.units.push(Unit {    name: "Scimitar Artillery".to_string(),                               points: 8});
                new_list.units.push(Unit {    name: "Tachi Tank".to_string(),                                       points: 6});
                new_list.units.push(Unit {    name: "Nagamaki Tank".to_string(),                                    points: 7});
                new_list.units.push(Unit {    name: "Odachi Tank".to_string(),                                      points: 8});
                new_list.units.push(Unit {    name: "Dagger Drop Pod".to_string(),                                  points: 3});
                new_list.units.push(Unit {    name: "Mandau Drop Pod".to_string(),                                  points: 3});
                new_list.units.push(Unit {    name: "Parang Drop Pod".to_string(),                                  points: 3});
                new_list.units.push(Unit {    name: "Rapier VTOL".to_string(),                                      points: 5});
                new_list.units.push(Unit {    name: "Dao Walker".to_string(),                                       points: 4});
                new_list.units.push(Unit {    name: "Jian Walker".to_string(),                                      points: 3});
                new_list.units.push(Unit {    name: "Estoc Air Transport".to_string(),                              points: 5});
                new_list.units.push(Unit {    name: "Rapier Gunship".to_string(),                                   points: 7});

                // Supports
                new_list.supports.push(Support {        name: "Orbital Drop".to_string(),                           points: 3});
                new_list.supports.push(Support {        name: "Malware Hack".to_string(),                           points: 2});
                new_list.supports.push(Support {        name: "Satellite Scan".to_string(),                         points: 1});
                new_list.supports.push(Support {        name: "Superior Directive".to_string(),                     points: 1});
            }

            //// ENLISTED
            Faction::Enlisted => {

                // Characters
                new_list.characters.push(Character {    name: "General".to_string(),                                points: 5});
                new_list.characters.push(Character {    name: "Infantry Hero".to_string(),                          points: 4});
                new_list.characters.push(Character {    name: "Infantry NCO".to_string(),                           points: 2});
                new_list.characters.push(Character {    name: "Tank Commander".to_string(),                         points: 3});

                // Units
                new_list.units.push(Unit {              name: "Infantrymen".to_string(),                            points: 2});
                new_list.units.push(Unit {              name: "Armored Infantry".to_string(),                       points: 4});
                new_list.units.push(Unit {              name: "Light Mortar".to_string(),                           points: 3});
                new_list.units.push(Unit {              name: "Scouts".to_string(),                                 points: 3});
                new_list.units.push(Unit {              name: "Support Vehicle".to_string(),                        points: 3});
                new_list.units.push(Unit {              name: "Troop Carrier".to_string(),                          points: 3});
                new_list.units.push(Unit {              name: "Main Battle Tank".to_string(),                       points: 5});
                new_list.units.push(Unit {              name: "Energy Tank".to_string(),                            points: 5});
                new_list.units.push(Unit {              name: "Heavy Battle Tank".to_string(),                      points: 8});
                new_list.units.push(Unit {              name: "Gunner Walker".to_string(),                          points: 3});
                new_list.units.push(Unit {              name: "Support Walker".to_string(),                         points: 3});
                new_list.units.push(Unit {              name: "Rocket Launcher".to_string(),                        points: 6});
                new_list.units.push(Unit {              name: "Self-Propelled Gun".to_string(),                     points: 6});
                
                // Supports
                new_list.supports.push(Support {        name: "Direct Command".to_string(),                         points: 1});
                new_list.supports.push(Support {        name: "Guided Missile".to_string(),                         points: 2});
                new_list.supports.push(Support {        name: "Spotter Drones".to_string(),                         points: 2});
                new_list.supports.push(Support {        name: "Mortar Fire".to_string(),                            points: 6});
                new_list.supports.push(Support {        name: "Strafing Run".to_string(),                           points: 6});

            }
            _ => panic!("Faction not ready yet!")
        }

        new_list
    }
}