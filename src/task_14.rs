use fuel_factory::{Reaction, ReactionSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod fuel_factory {
    use std::collections::HashMap;
    use std::iter::FromIterator;
    use std::ops::Mul;
    use std::str::FromStr;

    #[derive(Clone, Debug)]
    pub struct ElementUnit {
        quantity: usize,
        element: String,
    }

    impl ElementUnit {
        pub fn new(name: String, quantity: usize) -> Self {
            ElementUnit {
                quantity,
                element: name,
            }
        }
    }

    impl FromStr for ElementUnit {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.split(" ").collect::<Vec<&str>>();
            Ok(ElementUnit {
                quantity: s[0].parse::<usize>().unwrap(),
                element: s[1].to_string(),
            })
        }
    }

    #[derive(Clone)]
    pub struct Reaction {
        from: Vec<ElementUnit>,
        to: ElementUnit,
    }

    impl Mul<usize> for &Reaction {
        type Output = Reaction;
        fn mul(self, rhs: usize) -> Self::Output {
            let mut result = self.clone();
            result.to.quantity *= rhs;
            result.from.iter_mut().for_each(|r| r.quantity *= rhs);
            result
        }
    }

    impl FromStr for Reaction {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.split(" => ").collect::<Vec<&str>>();
            let target = s[1].parse::<ElementUnit>().unwrap();
            let s = s[0]
                .split(", ")
                .filter_map(|l| l.parse::<ElementUnit>().ok())
                .collect();
            Ok(Reaction {
                from: s,
                to: target,
            })
        }
    }

    pub struct ReactionSet {
        reactions: Vec<Reaction>,
    }

    impl ReactionSet {
        fn sorted_reactions(&self) -> HashMap<String, usize> {
            let mut result = HashMap::new();
            result.insert("ORE".to_string(), 0);
            while !result.contains_key("FUEL") {
                let level = result.iter().map(|r| *r.1).max().unwrap() + 1;
                let mut changes = HashMap::new();
                for reaction in &self.reactions {
                    if !result.contains_key(&reaction.to.element)
                        && reaction
                            .from
                            .iter()
                            .all(|r| result.contains_key(&r.element))
                    {
                        changes.insert(reaction.to.element.clone(), level);
                    }
                }
                for c in changes {
                    result.insert(c.0, c.1);
                }
            }
            result
        }

        pub fn evaluate_reversed(&self, available: usize) -> usize {
            let mut result = available / self.evaluate(1);

            let mut addition = 10_usize.pow(0);
            while result / addition != 0 {
                addition *= 10;
            }
            addition /= 10;

            while addition != 1 {
                if available >= self.evaluate(result + addition / 2) {
                    result += addition / 2;
                }
                addition /= 2;
            }

            if available < self.evaluate(result + addition) {
                result + addition - 1
            } else {
                result + addition
            }
        }

        pub fn evaluate(&self, target: usize) -> usize {
            let sorted = self.sorted_reactions();

            let current_level = *sorted.values().max().unwrap_or(&0);

            let mut available_items = vec![];
            for (key, _) in sorted.iter().filter(|s| s.1 == &current_level) {
                available_items.push(ElementUnit::new(key.clone(), target));
            }
            while !available_items
                .iter()
                .map(|item| sorted[&item.element])
                .all(|level| level == 0)
            {
                available_items.sort_by(|l, r| sorted[&l.element].cmp(&sorted[&r.element]));
                let element = available_items.pop().unwrap();
                let reaction = self
                    .reactions
                    .iter()
                    .find(|r| r.to.element == element.element)
                    .unwrap()
                    .clone();
                let mut mltplr = element.quantity / reaction.to.quantity;
                if element.quantity % reaction.to.quantity != 0 {
                    mltplr += 1;
                }
                for f in reaction.from {
                    if let Some((i, e)) = available_items
                        .iter()
                        .enumerate()
                        .find(|i| i.1.element == f.element)
                        .map(|(i, e)| (i, e.clone()))
                    {
                        available_items.remove(i);
                        available_items.push(ElementUnit::new(
                            f.element,
                            e.quantity + f.quantity * mltplr,
                        ));
                    } else {
                        available_items.push(ElementUnit::new(f.element, f.quantity * mltplr));
                    }
                }
            }
            available_items.iter().map(|e| e.quantity).sum()
        }
    }

    impl FromIterator<Reaction> for ReactionSet {
        fn from_iter<I: IntoIterator<Item = Reaction>>(iter: I) -> Self {
            ReactionSet {
                reactions: iter.into_iter().collect(),
            }
        }
    }
}

pub fn run() {
    let input = File::open("input/task_14").unwrap();
    let input = BufReader::new(input);
    let input: ReactionSet = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Reaction>().ok())
        .collect();

    let result = input.evaluate(1);

    println!("Result: {}", result)
}

pub fn run_e() {
    let input = File::open("input/task_14").unwrap();
    let input = BufReader::new(input);
    let input: ReactionSet = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Reaction>().ok())
        .collect();

    let result = input.evaluate_reversed(1_000_000_000_000);

    println!("Result: {}", result)
}
