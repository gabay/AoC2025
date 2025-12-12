use std::collections::{HashSet, VecDeque};
use z3;

pub fn part1(data: &str) -> usize {
    data.split('\n')
        .map(|line| Machine::from(line).steps_to_target())
        .sum()
}

pub fn part2(data: &str) -> usize {
    data.split('\n')
        .map(|line| Machine::from(line).steps_to_joltage())
        .sum()
}

type Button = Vec<usize>;

#[derive(Hash, Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts = value.split(' ').collect::<Vec<_>>();
        let target_str = parts[0];
        let buttons_strs = &parts[1..parts.len() - 1];
        let joltage_str = parts[parts.len() - 1];
        let target: Vec<bool> = target_str[1..target_str.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();
        let buttons = buttons_strs
            .iter()
            .map(|bs| {
                bs[1..bs.len() - 1]
                    .split(',')
                    .map(|c| str::parse::<usize>(c).unwrap())
                    .collect()
            })
            .collect();
        let joltage = joltage_str[1..joltage_str.len() - 1]
            .split(',')
            .map(|c| str::parse::<usize>(c).unwrap())
            .collect();
        Self {
            target,
            buttons,
            joltage,
        }
    }
}

impl Machine {
    fn steps_to_target(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((State1::new(self.target.len()), 0));
        let mut seen = HashSet::new();
        while let Some((state, depth)) = queue.pop_front() {
            if self.target == state.lights {
                return depth;
            }
            if seen.insert(state.clone()) {
                self.buttons
                    .iter()
                    .for_each(|button| queue.push_back((state.press(button), depth + 1)));
            }
        }
        panic!("Cannot reach target. {:?}", self);
    }

    fn steps_to_joltage(&self) -> usize {
        let presses = self
            .buttons
            .iter()
            .map(|_| z3::ast::Int::fresh_const(""))
            .collect::<Vec<_>>();
        let solver = z3::Solver::new();
        // all presses are positive
        for press in presses.iter() {
            solver.assert(press.ge(0));
        }
        // for each joltage - all buttons with its index sum to it
        for (joltage_index, joltage) in self.joltage.iter().enumerate() {
            let relevant_presses = self
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, button)| button.contains(&joltage_index))
                .map(|(index, _)| &presses[index])
                .collect::<Vec<_>>();
            solver.assert(z3::ast::Int::add(&relevant_presses).eq(*joltage as u32));
        }
        solver
            .solutions(&presses, false)
            .map(|solution| {
                solution
                    .iter()
                    .map(|i| i.as_u64().unwrap() as usize)
                    .sum::<usize>()
            })
            .min()
            .unwrap()
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct State1 {
    lights: Vec<bool>,
}

impl State1 {
    fn new(len: usize) -> Self {
        State1 {
            lights: (0..len).map(|_| false).collect(),
        }
    }

    fn press(&self, button: &Button) -> Self {
        let mut new_self = self.clone();
        button.iter().for_each(|index| {
            new_self.lights[*index] = !new_self.lights[*index];
        });
        new_self
    }
}
