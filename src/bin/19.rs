use crate::Bots::{ClayBot, GeodeBot, ObsidianBot, OreBot};
use crate::Material::{Clay, Obsidian, Ore};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use rustc_hash::FxHashMap;
use std::cmp::max;
use std::collections::VecDeque;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq)]
enum Bots {
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

#[derive(Debug, Default, Copy, Clone)]
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

    /// Returns the most expensive bot cost for every resource
    #[inline(always)]
    fn get_max_costs(&self) -> (usize, usize, usize) {
        let ore_bot = self.get_bot_costs(&OreBot);
        let clay_bot = self.get_bot_costs(&ClayBot);
        let obsidian_bot = self.get_bot_costs(&ObsidianBot);
        let geode_bot = self.get_bot_costs(&GeodeBot);
        (
            max(ore_bot.0, max(clay_bot.0, max(obsidian_bot.0, geode_bot.0))),
            max(ore_bot.1, max(clay_bot.1, max(obsidian_bot.1, geode_bot.1))),
            max(ore_bot.2, max(clay_bot.2, max(obsidian_bot.2, geode_bot.2))),
        )
    }

    #[inline(always)]
    fn get_bot_costs(&self, bot: &Bots) -> (usize, usize, usize) {
        match bot {
            OreBot => (*self.ore_robot.get(&Ore).unwrap(), 0, 0),
            ClayBot => (*self.clay_robot.get(&Ore).unwrap(), 0, 0),
            ObsidianBot => (
                *self.obsidian_robot.get(&Ore).unwrap(),
                *self.obsidian_robot.get(&Clay).unwrap(),
                0,
            ),
            GeodeBot => (
                *self.geode_robot.get(&Ore).unwrap(),
                0,
                *self.geode_robot.get(&Obsidian).unwrap(),
            ),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
struct State {
    inventory: BotInventory,
    resources: Resources,
    elapsed_time: usize,
}

impl State {
    #[inline(always)]
    fn mine_minerals(&mut self, time: usize) {
        self.resources.ore += self.inventory.ore_robot * time;
        self.resources.clay += self.inventory.clay_robot * time;
        self.resources.obsidian += self.inventory.obsidian_robot * time;
        self.resources.geode += self.inventory.geode_robot * time;
    }

    #[inline(always)]
    fn add_bot(&mut self, bot: &Bots, blueprint: &Blueprint) {
        // println!("Buildling {bot:?}");
        match bot {
            OreBot => {
                self.inventory.ore_robot += 1;
                self.resources.ore -= blueprint.ore_robot.get(&Ore).unwrap();
            }
            ClayBot => {
                self.inventory.clay_robot += 1;
                self.resources.ore -= blueprint.clay_robot.get(&Ore).unwrap();
            }
            ObsidianBot => {
                self.inventory.obsidian_robot += 1;
                self.resources.ore -= blueprint.obsidian_robot.get(&Ore).unwrap();
                self.resources.clay -= blueprint.obsidian_robot.get(&Clay).unwrap();
            }
            GeodeBot => {
                self.inventory.geode_robot += 1;
                self.resources.ore -= blueprint.geode_robot.get(&Ore).unwrap();
                self.resources.obsidian -= blueprint.geode_robot.get(&Obsidian).unwrap();
            }
        }
    }
}

fn max_geodes(state: State, blueprint: &Blueprint, time_limit: usize) -> usize {
    static BOT_TYPES: [Bots; 4] = [GeodeBot, ObsidianBot, ClayBot, OreBot];

    // Prune 1 Setup - Don't overbuild bots, only build up to the max cost of any resource
    let (max_ore_cost, max_clay_cost, max_obsidian_cost) = blueprint.get_max_costs();

    let mut max_geodes = 0;
    let mut queue = VecDeque::new();
    queue.push_back(state);
    // DFS Search across all the solution space for this blueprint
    while let Some(State {
                       inventory,
                       resources,
                       elapsed_time,
                   }) = queue.pop_front()
    {
        // Explore building every bot type as a new branch
        for bot_type in BOT_TYPES.iter() {
            // Prune 1 - Don't overbuild bots, only build up to the max cost of any resource
            if (bot_type == &OreBot && inventory.ore_robot >= max_ore_cost)
                || (bot_type == &ClayBot && inventory.clay_robot >= max_clay_cost)
                || (bot_type == &ObsidianBot && inventory.obsidian_robot >= max_obsidian_cost)
            {
                continue;
            }

            // Figure out how long we must wait before the bot _could_ be made. If there are
            // no pre-requisite bots then the max time is the only option, which prunes the
            // search branch. TODO: This could be pulled into a function, it's messy.
            let (ore_cost, clay_cost, obsidian_cost) = blueprint.get_bot_costs(bot_type);
            let delta_time = if resources.ore >= ore_cost
                && resources.clay >= clay_cost
                && resources.obsidian >= obsidian_cost
            {
                0 // No need to wait for materials, we have what we need
            } else {
                let time_for_ore = if ore_cost == 0 || resources.ore >= ore_cost {
                    0
                } else if inventory.ore_robot >= 1 {
                    (ore_cost - resources.ore + inventory.ore_robot - 1) / inventory.ore_robot
                } else {
                    time_limit
                };
                let time_for_clay = if clay_cost == 0 || resources.clay >= clay_cost {
                    0
                } else if inventory.clay_robot >= 1 {
                    (clay_cost - resources.clay + inventory.clay_robot - 1) / inventory.clay_robot
                } else {
                    time_limit
                };
                let time_for_obsidian = if obsidian_cost == 0 || resources.obsidian >= obsidian_cost
                {
                    0
                } else if inventory.obsidian_robot >= 1 {
                    (obsidian_cost - resources.obsidian + inventory.obsidian_robot - 1)
                        / inventory.obsidian_robot
                } else {
                    time_limit
                };
                max(time_for_ore, max(time_for_clay, time_for_obsidian))
            };

            // BASE CASE - Bot construction takes longer than we have left
            let new_time = elapsed_time + delta_time + 1;
            if new_time >= time_limit {
                continue;
            }

            // Prune 2 - Cheating by creating geodes every remaining minute (sorta, kinda). If this
            // strategy can't be the max from another tree we can bail out. Saves around 90% of the
            // overall runtime. The `5` is a magic number that appeases the search on the puzzle.
            let time_left = time_limit - new_time;
            let cheating_geodes = ((time_left + 5) * time_left)
                + resources.geode
                + time_left * inventory.geode_robot;
            if cheating_geodes < max_geodes {
                continue;
            }

            // Create the new state for the next branch
            let mut new_state = State {
                inventory,
                resources,
                elapsed_time: new_time,
            };
            // Make sure we mine minerals before adding the new robot
            new_state.mine_minerals(delta_time + 1);
            // Now we add the new robot to the inventory for the next round
            new_state.add_bot(bot_type, blueprint);
            queue.push_back(new_state);
        }
        max_geodes = max(
            max_geodes,
            resources.geode + inventory.geode_robot * (time_limit - elapsed_time),
        );
    }
    max_geodes
}

pub fn part_one(input: &str) -> Option<usize> {
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::new).collect();
    let answer = blueprints
        .par_iter()
        .map(|b| {
            let state = State::default();
            b.id * max_geodes(state, b, 24)
        })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let blueprints: Vec<Blueprint> = input.lines().take(3).map(Blueprint::new).collect();
    let answer: usize = blueprints
        .par_iter()
        .map(|b| {
            let state = State::default();
            max_geodes(state, b, 32)
        })
        .reduce(|| 1, |x, acc| x * acc);
    Some(answer)
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
        assert_eq!(part_two(&input), Some(2950));
    }
}
