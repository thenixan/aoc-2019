use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Position(usize),
    Immediate(i32),
}

impl Mode {
    fn get(&self, v: &Vec<i32>) -> i32 {
        match self {
            Mode::Immediate(x) => x.clone(),
            Mode::Position(p) => v[*p].clone(),
        }
    }

    fn set(&self, v: &mut Vec<i32>, value: i32) {
        match self {
            Mode::Position(p) => v[*p] = value,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Add { left: Mode, right: Mode, to: Mode },
    Multiply { left: Mode, right: Mode, to: Mode },
    Halt,
    Input { to: Mode },
    Output { from: Mode },
    JumpIfTrue { check: Mode, to: Mode },
    JumpIfFalse { check: Mode, to: Mode },
    Less { left: Mode, right: Mode, to: Mode },
    Equal { left: Mode, right: Mode, to: Mode },
}

impl Opcode {
    fn advance_by(&self) -> usize {
        match self {
            Opcode::Add { .. } => 4,
            Opcode::Multiply { .. } => 4,
            Opcode::Halt => 0,
            Opcode::Input { .. } => 2,
            Opcode::Output { .. } => 2,
            Opcode::JumpIfFalse { .. } => 0,
            Opcode::JumpIfTrue { .. } => 0,
            Opcode::Less { .. } => 4,
            Opcode::Equal { .. } => 4,
        }
    }

    fn from_vec(v: &Vec<i32>, position: usize) -> Self {
        let command = format!("{:05}", v[position]).chars().collect::<Vec<char>>();
        let mode_3 = command[0] == '0';
        let mode_2 = command[1] == '0';
        let mode_1 = command[2] == '0';
        match (command[3], command[4]) {
            ('0', '1') => Opcode::Add {
                left: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
                right: if mode_2 {
                    Mode::Position(v[position + 2] as usize)
                } else {
                    Mode::Immediate(v[position + 2])
                },
                to: if mode_3 {
                    Mode::Position(v[position + 3] as usize)
                } else {
                    Mode::Immediate(v[position + 3])
                },
            },
            ('0', '2') => Opcode::Multiply {
                left: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
                right: if mode_2 {
                    Mode::Position(v[position + 2] as usize)
                } else {
                    Mode::Immediate(v[position + 2])
                },
                to: if mode_3 {
                    Mode::Position(v[position + 3] as usize)
                } else {
                    Mode::Immediate(v[position + 3])
                },
            },
            ('0', '3') => Opcode::Input {
                to: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
            },
            ('0', '4') => Opcode::Output {
                from: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
            },
            ('0', '5') => Opcode::JumpIfTrue {
                check: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
                to: if mode_2 {
                    Mode::Position(v[position + 2] as usize)
                } else {
                    Mode::Immediate(v[position + 2])
                },
            },
            ('0', '6') => Opcode::JumpIfFalse {
                check: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
                to: if mode_2 {
                    Mode::Position(v[position + 2] as usize)
                } else {
                    Mode::Immediate(v[position + 2])
                },
            },
            ('0', '7') => Opcode::Less {
                left: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
                right: if mode_2 {
                    Mode::Position(v[position + 2] as usize)
                } else {
                    Mode::Immediate(v[position + 2])
                },
                to: if mode_3 {
                    Mode::Position(v[position + 3] as usize)
                } else {
                    Mode::Immediate(v[position + 3])
                },
            },
            ('0', '8') => Opcode::Equal {
                left: if mode_1 {
                    Mode::Position(v[position + 1] as usize)
                } else {
                    Mode::Immediate(v[position + 1])
                },
                right: if mode_2 {
                    Mode::Position(v[position + 2] as usize)
                } else {
                    Mode::Immediate(v[position + 2])
                },
                to: if mode_3 {
                    Mode::Position(v[position + 3] as usize)
                } else {
                    Mode::Immediate(v[position + 3])
                },
            },
            ('9', '9') => Opcode::Halt,
            _ => panic!("Unknown command"),
        }
    }
}

#[derive(Clone)]
pub struct Programm {
    code: Vec<i32>,
    position: usize,
    is_finished: bool,
}

impl FromStr for Programm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Programm {
            code: s.split(",").filter_map(|l| l.parse::<i32>().ok()).collect(),
            position: 0,
            is_finished: false,
        })
    }
}

impl Programm {
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
    pub fn run(&mut self, inputs: &mut Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        loop {
            let before_modifications = self.code[self.position].clone();
            let command = Opcode::from_vec(&self.code, self.position);

            match &command {
                Opcode::Add { left, right, to } => {
                    let l = left.get(&self.code);
                    let r = right.get(&self.code);
                    to.set(&mut self.code, l + r);
                }
                Opcode::Multiply { left, right, to } => {
                    let l = left.get(&self.code);
                    let r = right.get(&self.code);
                    to.set(&mut self.code, l * r);
                }
                Opcode::Input { to } => {
                    if inputs.is_empty() {
                        break;
                    } else {
                        to.set(&mut self.code, inputs.pop().unwrap())
                    }
                }
                Opcode::Output { from } => result.push(from.get(&self.code)),
                Opcode::Halt => {
                    self.is_finished = true;
                    break;
                }
                Opcode::JumpIfTrue { check, to } => {
                    let c = check.get(&self.code);
                    if c != 0 {
                        let t = to.get(&self.code);
                        self.position = t as usize;
                    } else {
                        self.position += 3;
                    }
                }
                Opcode::JumpIfFalse { check, to } => {
                    let c = check.get(&self.code);
                    if c == 0 {
                        let t = to.get(&self.code);
                        self.position = t as usize;
                    } else {
                        self.position += 3;
                    }
                }
                Opcode::Less { left, right, to } => {
                    let l = left.get(&self.code);
                    let r = right.get(&self.code);
                    if l < r {
                        to.set(&mut self.code, 1);
                    } else {
                        to.set(&mut self.code, 0);
                    }
                }
                Opcode::Equal { left, right, to } => {
                    let l = left.get(&self.code);
                    let r = right.get(&self.code);
                    if l == r {
                        to.set(&mut self.code, 1);
                    } else {
                        to.set(&mut self.code, 0);
                    }
                }
            }
            if before_modifications == self.code[self.position] {
                self.position += &command.advance_by();
            }
        }

        result
    }
}
