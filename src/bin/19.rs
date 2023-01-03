use crate::FactoryProcess::{ClayBot, GeodeBot, Idle, ObsidianBot, OreBot};
use crate::Material::{Clay, Obsidian, Ore};
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct Factory {
    queue: FactoryProcess,
}

impl Default for Factory {
    fn default() -> Self {
        Self { queue: Idle }
    }
}

#[derive(Debug, PartialEq)]
enum FactoryProcess {
    Idle,
    ClayBot,
    OreBot,
    ObsidianBot,
    GeodeBot,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
}

#[derive(Debug, Default)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Clone, Debug, Default)]
struct Blueprint {
    id: usize,
    ore_robot: FxHashMap<Material, usize>,
    clay_robot: FxHashMap<Material, usize>,
    obsidian_robot: FxHashMap<Material, usize>,
    geode_robot: FxHashMap<Material, usize>,
}

impl Blueprint {
    fn new(input: &str) -> Self {
        let mut slf = Self {
            id: 0,
            ore_robot: Default::default(),
            clay_robot: Default::default(),
            obsidian_robot: Default::default(),
            geode_robot: Default::default(),
        };
        let re = Regex::new(r"\s(\d*)[\s:]").unwrap();
        let cap = re
            .find_iter(input)
            .map(|m| m.as_str().trim().replace(':', "").parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        slf.id = cap[0];
        slf.ore_robot.insert(Ore, cap[1]);
        slf.clay_robot.insert(Ore, cap[2]);
        slf.obsidian_robot.insert(Ore, cap[3]);
        slf.obsidian_robot.insert(Clay, cap[4]);
        slf.geode_robot.insert(Ore, cap[5]);
        slf.geode_robot.insert(Obsidian, cap[6]);

        slf
    }
}

#[derive(Debug)]
struct BotInventory {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl Default for BotInventory {
    fn default() -> Self {
        Self {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }
}

#[derive(Debug, Default)]
struct Simulation {
    blueprint: Blueprint,
    factory: Factory,
    inventory: BotInventory,
    resources: Resources,
}

impl Simulation {
    /// Bots go BRRRRRRRRR
    fn harvest_resources(&mut self) {
        self.resources.ore += self.inventory.ore_robot;
        self.resources.clay += self.inventory.clay_robot;
        self.resources.obsidian += self.inventory.obsidian_robot;
        self.resources.geode += self.inventory.geode_robot;
        println!("Harvest complete: {:?}", self.resources);
    }

    /// Issues the construction of a robot at the factory and returns true if the request could be
    /// satisfied.
    fn queue_robot_build(&mut self, request: &FactoryProcess) -> bool {
        if self.factory.queue == Idle {
            match request {
                Idle => (),
                ClayBot => {
                    let required_ore = *self.blueprint.clay_robot.get(&Ore).unwrap();
                    if self.resources.ore >= required_ore {
                        self.resources.ore -= required_ore;
                        self.factory.queue = ClayBot;
                        println!("Request ClayBot");
                    }
                }
                OreBot => {
                    let required_ore = *self.blueprint.ore_robot.get(&Ore).unwrap();
                    if self.resources.ore >= required_ore {
                        self.resources.ore -= required_ore;
                        self.factory.queue = OreBot;
                        println!("Request OreBot");
                    }
                }
                ObsidianBot => {
                    let required_ore = *self.blueprint.obsidian_robot.get(&Ore).unwrap();
                    let required_clay = *self.blueprint.obsidian_robot.get(&Clay).unwrap();
                    if self.resources.ore >= required_ore && self.resources.clay >= required_clay {
                        self.resources.ore -= required_ore;
                        self.resources.clay -= required_clay;
                        self.factory.queue = ObsidianBot;
                        println!("Request ObsidianBot");
                    }
                }
                GeodeBot => {
                    let required_ore = *self.blueprint.geode_robot.get(&Ore).unwrap();
                    let required_obsidian = *self.blueprint.geode_robot.get(&Obsidian).unwrap();
                    if self.resources.ore >= required_ore
                        && self.resources.obsidian >= required_obsidian
                    {
                        self.resources.ore -= required_ore;
                        self.resources.obsidian -= required_obsidian;
                        self.factory.queue = GeodeBot;
                        println!("Request GeoBot");
                    }
                }
            }
        }
        self.factory.queue != Idle
    }

    /// Delivers a robot in the factory's queue and restores the factory to an Idle state.
    fn deliver_robot(&mut self) {
        match self.factory.queue {
            ClayBot => self.inventory.clay_robot += 1,
            OreBot => self.inventory.ore_robot += 1,
            ObsidianBot => self.inventory.obsidian_robot += 1,
            GeodeBot => self.inventory.geode_robot += 1,
            Idle => (),
        }
        self.factory.queue = Idle;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blueprints: Vec<Blueprint> = input.lines().map(Blueprint::new).collect();
    let mut sim = Simulation::default();
    sim.blueprint = blueprints.get(1).unwrap().clone();

    let mut strategy = vec![
        ClayBot,
        ClayBot,
        ClayBot,
        ObsidianBot,
        ClayBot,
        ObsidianBot,
        GeodeBot,
        GeodeBot,
    ];
    strategy.reverse();
    let mut minutes: usize = 1;
    while minutes <= 24 {
        println!("======Minute {} ========", minutes);
        sim.deliver_robot();
        if let Some(bot_request) = strategy.last() {
            if sim.queue_robot_build(bot_request) {
                strategy.pop();
            }
        }
        sim.harvest_resources();
        minutes += 1;
    }
    println!(
        "Quality Level: {:?}",
        sim.resources.geode * sim.blueprint.id
    );
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
