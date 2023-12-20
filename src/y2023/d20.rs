use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> u64 {
    let mut flipflop_states = HashMap::new();
    let mut conjunction_states = HashMap::new();
    let info: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let destinations: Vec<_> = destinations.split(", ").collect();
            if name.starts_with('%') {
                flipflop_states.insert(&name[1..], false);
                (&name[1..], (ModuleType::FlipFlop, destinations))
            } else if name.starts_with('&') {
                conjunction_states.insert(&name[1..], HashMap::new());
                (&name[1..], (ModuleType::Conjunction, destinations))
            } else {
                (name, (ModuleType::Broadcaster, destinations))
            }
        })
        .collect();

    for (&module, (_, dests)) in &info {
        for &dest in dests {
            if let Some(connections) = conjunction_states.get_mut(dest) {
                connections.insert(module, false);
            }
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("broadcaster", false, "button")]);
        while let Some((module, pulse_type, source)) = queue.pop_front() {
            if pulse_type {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            let Some((module_type, destinations)) = info.get(module) else {
                continue;
            };

            match module_type {
                ModuleType::FlipFlop => {
                    let current_state = flipflop_states.get_mut(module).unwrap();
                    if !pulse_type {
                        *current_state = !*current_state;
                        for destination in destinations {
                            queue.push_back((destination, *current_state, module));
                        }
                    }
                }
                ModuleType::Conjunction => {
                    let current_states = &mut conjunction_states.get_mut(module).unwrap();
                    *current_states.get_mut(source).unwrap() = pulse_type;

                    let out_pulse = !current_states.values().copied().all(std::convert::identity);

                    for destination in destinations {
                        queue.push_back((destination, out_pulse, module));
                    }
                }
                ModuleType::Broadcaster => {
                    for destination in destinations {
                        queue.push_back((destination, pulse_type, module));
                    }
                }
            }
        }
    }
    low_pulses * high_pulses
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

pub fn part2(input: &str) -> u64 {
    let mut flipflop_states = HashMap::new();
    let mut conjunction_states = HashMap::new();
    let info: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let destinations: Vec<_> = destinations.split(", ").collect();
            if name.starts_with('%') {
                flipflop_states.insert(&name[1..], false);
                (&name[1..], (ModuleType::FlipFlop, destinations))
            } else if name.starts_with('&') {
                conjunction_states.insert(&name[1..], HashMap::new());
                (&name[1..], (ModuleType::Conjunction, destinations))
            } else {
                (name, (ModuleType::Broadcaster, destinations))
            }
        })
        .collect();

    for (&module, (_, dests)) in &info {
        for &dest in dests {
            if let Some(connections) = conjunction_states.get_mut(dest) {
                connections.insert(module, false);
            }
        }
    }

    // TODO: remove hardcode
    let rx_father = "zh";
    let mut cycles = HashMap::<_, _>::new();

    for button_press in 1.. {
        let mut queue = VecDeque::from([("broadcaster", false, "button")]);
        let mut count: u16 = 0;

        while let Some((module, pulse_type, source)) = queue.pop_front() {
            let Some((module_type, destinations)) = info.get(module) else {
                count += u16::from(!pulse_type);
                continue;
            };

            match module_type {
                ModuleType::FlipFlop => {
                    let current_state = flipflop_states.get_mut(module).unwrap();
                    if !pulse_type {
                        *current_state = !*current_state;
                        for destination in destinations {
                            queue.push_back((destination, *current_state, module));
                        }
                    }
                }
                ModuleType::Conjunction => {
                    let current_states = &mut conjunction_states.get_mut(module).unwrap();
                    *current_states.get_mut(source).unwrap() = pulse_type;

                    if module == rx_father {
                        for m in current_states.iter().filter_map(|(&m, s)| s.then_some(m)) {
                            cycles.entry(m).or_insert(button_press);
                            if cycles.len() == 4 {
                                return cycles.values().copied().fold(1, lcm);
                            }
                        }
                    }

                    let out_pulse = !current_states.values().copied().all(std::convert::identity);

                    for destination in destinations {
                        queue.push_back((destination, out_pulse, module));
                    }
                }
                ModuleType::Broadcaster => {
                    for destination in destinations {
                        queue.push_back((destination, pulse_type, module));
                    }
                }
            }
        }

        if count >= 1 {
            return button_press;
        }
    }
    unreachable!()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        (a, b) = (b, a);
    }
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
