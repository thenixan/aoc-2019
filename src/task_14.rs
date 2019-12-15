use fuel_factory::{Reaction, ReactionSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod fuel_factory {
    use std::collections::HashMap;
    use std::iter::FromIterator;
    use std::ops::Add;
    use std::ops::Mul;
    use std::str::FromStr;

    #[derive(Clone, Debug)]
    struct ElementUnit {
        quantity: usize,
        element: String,
    }

    impl ElementUnit {
        fn suitable(&self, available: &HashMap<String, usize>) -> usize {
            available
                .get(&self.element)
                .map(|r| r / self.quantity)
                .unwrap_or(0)
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

    impl Reaction {
        fn suitable(&self, available: &HashMap<String, usize>) -> usize {
            self.from
                .iter()
                .map(|e| e.suitable(available))
                .min()
                .unwrap_or(0)
        }

        fn apply(&self, available: &mut HashMap<String, usize>) {
            for e in &self.from {
                *available.entry(e.element.clone()).or_insert(0) -= e.quantity;
            }
            *available.entry(self.to.element.clone()).or_insert(0) += self.to.quantity;
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
        pub fn len(&self) -> usize {
            self.reactions.len()
        }

        pub fn evaluate(&self) -> usize {
            let target = self.find_path("FUEL");
            let mut required = target.from.clone();
            let mut result = 0;
            println!("Set: {:?}", required);
            while !required.is_empty() {
                if required.iter().all(|r| r.element == "ORE") {
                    while !required.is_empty() {
                        result += required.pop().unwrap().quantity;
                    }
                } else {
                    let r = required.pop().unwrap();
                    if r.element == "ORE" {
                        required.insert(0, r);
                    } else {
                        let t = self.find_path(&r.element);
                        let mut q = t.clone();
                        let mut i = 2;
                        while q.to.quantity < r.quantity {
                            q = t * i;
                            i += 1;
                        }
                        for i in q.from {
                            if let Some(s) = required.iter_mut().find(|r| r.element == i.element) {
                                s.quantity += i.quantity;
                            } else {
                                required.insert(0, i)
                            }
                        }
                    }
                }
                println!("Set: {:?}", required);
            }
            result
        }

        fn find_path(&self, target: &str) -> &Reaction {
            self.reactions
                .iter()
                .find(|r| r.to.element == target)
                .unwrap()
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

    let result = input.evaluate();

    println!("Result: {}", result)
}

pub fn run_e() {}
