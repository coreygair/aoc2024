use std::{collections::{HashMap, HashSet}, sync::LazyLock};

use regex::Regex;

type Input<'a> = (Vec<Wire>, Vec<&'a str>);

pub fn parse(input: &str) -> Input {
    let mut name_to_id = HashMap::new();
    let mut flag = false;
    for (id, l) in input.lines().enumerate() {
        if l == "" {
            flag = true;
            continue;
        }

        let name = if !flag {
            l.split(": ").next().unwrap()
        } else {
            l.split("-> ").skip(1).next().unwrap()
        };

        name_to_id.insert(name, id - if flag {1} else {0});
    }

    let mut flag = false;
    let wires = input.lines().filter_map(|l| {
        if l == "" {
            flag = true;
            return None;
        }

        if !flag {
            Some(Wire::new_const(l))
        } else {
            Some(Wire::new_operator(&mut name_to_id, l))
        }
    }).collect();

    let mut id_to_name = vec![""; name_to_id.len()];
        for (name, id) in name_to_id {
            id_to_name[id] = name
        }

    (wires, id_to_name)
}

pub fn part1((wires, id_to_name): &Input) -> u64 {
    let mut id_to_value = HashMap::new();
    let mut to_eval = HashSet::new();
    for (id, wire) in wires.iter().enumerate() {
        if let Wire::Const(v) = wire {
            id_to_value.insert(id, *v);
        } else {
            to_eval.insert(id);
        }
    }

    while to_eval.len() > 0 {
        let mut done = true;
        for id in to_eval.iter().cloned() {
            if let Some(v) = wires[id].value(&id_to_value) {
                to_eval.remove(&id);
                id_to_value.insert(id, v);
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }

    let mut z_values = id_to_value.iter().filter_map(|(id, v)| {
        let name = id_to_name[*id];
        if name.starts_with('z') {
            Some((name, *v))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    z_values.sort_by(|(a, _), (b,_)| a.cmp(b));
    z_values.into_iter().enumerate().fold(0, |acc, (i, (_, v))| acc | if v {1<<i} else {0})
}

// Assumption: the circuit is supposed to be a classic carry ripple adder.
//
// Assumed ^, then kept adding rules that a full ripple adder satisfies until solution :)
pub fn part2((wires, id_to_name): &Input) -> String {
    const LAST_Z: &str = "z45";

    let mut switched = Vec::new();

    for (id, wire) in wires.iter().enumerate() {
        let name = id_to_name[id];

        if name.starts_with('z') {
            if name != LAST_Z {
                // All outputs should come from an XOR...
                if matches!(wire, Wire::And(_, _) | Wire::Or(_, _)) {
                    switched.push(id);
                    continue;
                }
            } else {
                // ...apart from the last which is an OR carry out.
                if matches!(wire, Wire::And(_, _) | Wire::Xor(_, _)) {
                    switched.push(id);
                    continue;
                }
            }
        }
    }

    for (id, wire) in wires.iter().enumerate() {
        let name = id_to_name[id];
        
        if let Wire::Xor(a_id, b_id) = wire {
            let a = id_to_name[*a_id];
            let b = id_to_name[*b_id];

            // Inner gates (not output to z or input from x & y) cannot be XOR
            if !name.starts_with('z') && !(a.starts_with('x') && b.starts_with('y')) && !(a.starts_with('y') && b.starts_with('x')) {
                switched.push(id);
                continue;
            }
        }
    }

    for (id, wire) in wires.iter().enumerate() {     
        let name = id_to_name[id];

        if let Wire::Xor(a_id, b_id) = wire {
            let a = id_to_name[*a_id];
            let b = id_to_name[*b_id];

            // x__ XOR y__ MUST be the input to another XOR (unless the first input with no carry)
            if (a.starts_with('x') && b.starts_with('y')) || (a.starts_with('y') && b.starts_with('x')) {
                if name != "z00" {
                    let mut found = false;
                    for w2 in wires {
                        // Look for XOR wire with outer wire as input.
                        if let Wire::Xor(a2, b2) = w2 {
                            if *a2 == id || *b2 == id {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        switched.push(id);
                        continue;
                    }
                }
            }
        }
    }

    for (id, wire) in wires.iter().enumerate() {        
        if let Wire::And(a, b) = wire {
            let a = id_to_name[*a];
            let b = id_to_name[*b];

            if !(a == "x00" && b == "y00") && !(a == "y00" || b == "x00") {
                // AND gates MUST be the input to an OR
                // unless it is the first input bit (no carry in)
                let mut found = false;
                for w2 in wires {
                    // Look for OR wire with outer wire as input.
                    if let Wire::Or(a, b) = w2 {
                        if *a == id || *b == id {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    switched.push(id);
                    continue;
                }
            }
        }
    }

    let mut switched = switched.into_iter().map(|id| id_to_name[id]).collect::<Vec<_>>();
    switched.sort();
    switched.dedup();
    switched.join(",")
}

#[derive(Clone, Copy, Debug)]
pub enum Wire {
    Const(bool),
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

impl Wire {
    fn new_const<'a>(l: &'a str) -> Self {
        let mut parts: std::str::Split<'_, &str> = l.split(": ");

        let _name = parts.next().unwrap();
        
        let value = parts.next().unwrap().parse::<u8>().unwrap() == 1;
        Wire::Const(value)
    }

    fn new_operator<'a>(name_to_id: &mut HashMap<&'a str, usize>, l: &'a str) -> Self {
        static R: LazyLock<Regex> = LazyLock::new(|| Regex::new("^([a-z\\d]{3}) (AND|OR|XOR) ([a-z\\d]{3}) -> ([a-z\\d]{3})$").unwrap());
        
        let captures = R.captures(l).unwrap();

        let a = captures.get(1).unwrap().as_str();
        let op = captures.get(2).unwrap().as_str();
        let b = captures.get(3).unwrap().as_str();
        let _c = captures.get(4).unwrap().as_str();

        let a_id = name_to_id.get(a).unwrap().clone();
        let b_id = name_to_id.get(b).unwrap().clone();

        match op {
            "AND" => Wire::And(a_id, b_id),
            "OR" => Wire::Or(a_id, b_id),
            "XOR" => Wire::Xor(a_id, b_id),
            _ => unreachable!("unknown operator")
        }
    }
    
    fn value(self, others: &HashMap<usize, bool>) -> Option<bool> {
        match self {
            Wire::Const(v) => Some(v),
            Wire::And(a, b) => {
                let a = others.get(&a);
                let b = others.get(&b);
                a.and_then(|a| b.and_then(|b| Some(*a && *b)))
            }
            Wire::Or(a, b) => {
                let a = others.get(&a);
                let b = others.get(&b);
                a.and_then(|a| b.and_then(|b| Some(*a || *b)))
            }
            Wire::Xor(a, b) => {
                let a = others.get(&a);
                let b = others.get(&b);
                a.and_then(|a| b.and_then(|b| Some(*a ^ *b)))
            }
        }
    }
}
