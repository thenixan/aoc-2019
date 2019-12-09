use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl Mode {
    fn get(&self, v: &Programm) -> i64 {
        match self {
            Mode::Immediate(x) => x.clone(),
            Mode::Position(p) => {
                if v.code.len() <= *p {
                    0
                } else {
                    v.code[*p].clone()
                }
            }
            Mode::Relative(p) => {
                let ind = (v.relative_base as i64 + *p) as usize;
                if v.code.len() <= ind {
                    0
                } else {
                    v.code[ind].clone()
                }
            }
        }
    }

    fn set(&self, v: &mut Programm, value: i64) {
        match self {
            Mode::Position(p) => {
                if v.code.len() <= *p {
                    v.code
                        .extend(std::iter::repeat(0).take(*p - v.code.len() + 1));
                }
                v.code[*p] = value;
            }
            Mode::Relative(p) => {
                let ind = (v.relative_base as i64 + *p) as usize;
                if v.code.len() <= ind {
                    v.code
                        .extend(std::iter::repeat(0).take(ind - v.code.len() + 1));
                }
                v.code[ind] = value;
            }
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
    AdjustRelativeBase { to: Mode },
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
            Opcode::AdjustRelativeBase { .. } => 2,
        }
    }

    fn parse_mode(mode: char, val: i64) -> Mode {
        if mode == '0' {
            Mode::Position(val as usize)
        } else if mode == '1' {
            Mode::Immediate(val)
        } else {
            Mode::Relative(val)
        }
    }

    fn from_vec(v: &Vec<i64>, position: usize) -> Self {
        let command = format!("{:05}", v[position]).chars().collect::<Vec<char>>();
        let mode_3 = command[0];
        let mode_2 = command[1];
        let mode_1 = command[2];
        match (command[3], command[4]) {
            ('0', '1') => Opcode::Add {
                left: Self::parse_mode(mode_1, v[position + 1]),
                right: Self::parse_mode(mode_2, v[position + 2]),
                to: Self::parse_mode(mode_3, v[position + 3]),
            },
            ('0', '2') => Opcode::Multiply {
                left: Self::parse_mode(mode_1, v[position + 1]),
                right: Self::parse_mode(mode_2, v[position + 2]),
                to: Self::parse_mode(mode_3, v[position + 3]),
            },
            ('0', '3') => Opcode::Input {
                to: Self::parse_mode(mode_1, v[position + 1]),
            },
            ('0', '4') => Opcode::Output {
                from: Self::parse_mode(mode_1, v[position + 1]),
            },
            ('0', '5') => Opcode::JumpIfTrue {
                check: Self::parse_mode(mode_1, v[position + 1]),
                to: Self::parse_mode(mode_2, v[position + 2]),
            },
            ('0', '6') => Opcode::JumpIfFalse {
                check: Self::parse_mode(mode_1, v[position + 1]),
                to: Self::parse_mode(mode_2, v[position + 2]),
            },
            ('0', '7') => Opcode::Less {
                left: Self::parse_mode(mode_1, v[position + 1]),
                right: Self::parse_mode(mode_2, v[position + 2]),
                to: Self::parse_mode(mode_3, v[position + 3]),
            },
            ('0', '8') => Opcode::Equal {
                left: Self::parse_mode(mode_1, v[position + 1]),
                right: Self::parse_mode(mode_2, v[position + 2]),
                to: Self::parse_mode(mode_3, v[position + 3]),
            },
            ('0', '9') => Opcode::AdjustRelativeBase {
                to: Self::parse_mode(mode_1, v[position + 1]),
            },
            ('9', '9') => Opcode::Halt,
            _ => panic!("Unknown command"),
        }
    }
}

#[derive(Clone)]
pub struct Programm {
    code: Vec<i64>,
    position: usize,
    is_finished: bool,
    relative_base: usize,
}

impl FromStr for Programm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Programm {
            code: s.split(",").filter_map(|l| l.parse::<i64>().ok()).collect(),
            position: 0,
            is_finished: false,
            relative_base: 0,
        })
    }
}

impl Programm {
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
    pub fn run(&mut self, inputs: &mut Vec<i64>) -> Vec<i64> {
        let mut result = vec![];

        loop {
            let before_modifications = self.code[self.position].clone();
            let command = Opcode::from_vec(&self.code, self.position);

            match &command {
                Opcode::Add { left, right, to } => {
                    let l = left.get(self);
                    let r = right.get(self);
                    to.set(self, l + r);
                }
                Opcode::Multiply { left, right, to } => {
                    let l = left.get(self);
                    let r = right.get(self);
                    to.set(self, l * r);
                }
                Opcode::Input { to } => {
                    if inputs.is_empty() {
                        break;
                    } else {
                        to.set(self, inputs.pop().unwrap())
                    }
                }
                Opcode::Output { from } => result.push(from.get(self)),
                Opcode::Halt => {
                    self.is_finished = true;
                    break;
                }
                Opcode::JumpIfTrue { check, to } => {
                    let c = check.get(self);
                    if c != 0 {
                        let t = to.get(self);
                        self.position = t as usize;
                    } else {
                        self.position += 3;
                    }
                }
                Opcode::JumpIfFalse { check, to } => {
                    let c = check.get(self);
                    if c == 0 {
                        let t = to.get(self);
                        self.position = t as usize;
                    } else {
                        self.position += 3;
                    }
                }
                Opcode::Less { left, right, to } => {
                    let l = left.get(self);
                    let r = right.get(self);
                    if l < r {
                        to.set(self, 1);
                    } else {
                        to.set(self, 0);
                    }
                }
                Opcode::Equal { left, right, to } => {
                    let l = left.get(self);
                    let r = right.get(self);
                    if l == r {
                        to.set(self, 1);
                    } else {
                        to.set(self, 0);
                    }
                }
                Opcode::AdjustRelativeBase { to } => {
                    let t = to.get(self);
                    self.relative_base = (self.relative_base as i64 + t) as usize;
                }
            }
            if before_modifications == self.code[self.position] {
                self.position += &command.advance_by();
            }
        }

        result
    }
}
