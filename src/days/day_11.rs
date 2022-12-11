use std::{fmt::Display, rc::Rc, cell::RefCell};

type WorryLevel = u64;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square
}

#[derive(Debug)]
struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    test_divisor: u64,
    monkeys: Vec<Rc<RefCell<Monkey>>>,
    inspections: usize
}

impl Monkey {
    fn inspect(&mut self) {
        self.inspections += self.items.len();
        match self.operation {
            Operation::Add(n) => self.items.iter_mut().for_each(|i| *i += n),
            Operation::Multiply(n) => self.items.iter_mut().for_each(|i| *i *= n),
            Operation::Square => self.items.iter_mut().for_each(|i| *i = i.pow(2)),
        };
    }

    fn relief(&mut self) {
        self.items.iter_mut().for_each(|i| *i /= 3);
    }

    fn throw(&mut self) {
        for item in self.items.drain(..) {
            if item % self.test_divisor == 0 {
                self.monkeys[0].borrow_mut().items.push(item);
            } else {
                self.monkeys[1].borrow_mut().items.push(item);
            }
        }
    }

    fn reduce_worry_with_mod(&mut self, mod_prod: u64) {
        self.items.iter_mut().for_each(|i| *i %= mod_prod);
    }

    fn step(&mut self) {
        self.inspect();
        self.relief();
        self.throw();
    }

    fn step_without_relief(& mut self, mod_prod: u64) {
        self.inspect();
        self.reduce_worry_with_mod(mod_prod);
        self.throw();
    }
}

pub fn a(input: &str) -> Box<dyn Display> {
    let mut monkeys = Vec::new();
    let mut throw_monkeys = Vec::new();

    for string_monkey in input.split("\n\n") {
        let mut monkey_attributes = string_monkey.lines();

        let _id = monkey_attributes.next().unwrap()
            .trim().chars().nth(7).unwrap().to_digit(10).unwrap();
        let starting_items = monkey_attributes.next().unwrap()
            .trim()
            .split_once(": ").unwrap().1
            .split(", ")
            .map(|i| u64::from_str_radix(i, 10).unwrap())
            .collect();
        let operation = match monkey_attributes.next().unwrap().trim().split_once("= old ").unwrap().1.split_once(" ").unwrap() {
            ("*", "old") => Operation::Square,
            ("+", n) => Operation::Add(u64::from_str_radix(n, 10).unwrap()),
            ("*", n) => Operation::Multiply(u64::from_str_radix(n, 10).unwrap()),
            _ => unreachable!()
        };
        let divisor = u64::from_str_radix(monkey_attributes.next().unwrap().trim().split(" ").last().unwrap(), 10).unwrap();
        let true_monkey = monkey_attributes.next().unwrap().trim().chars().last().unwrap().to_digit(10).unwrap();
        let false_monkey = monkey_attributes.next().unwrap().trim().chars().last().unwrap().to_digit(10).unwrap();

        throw_monkeys.push([true_monkey as usize, false_monkey as usize]);

        monkeys.push(Rc::new(RefCell::new(Monkey {
            items: starting_items,
            operation: operation,
            test_divisor: divisor,
            monkeys: Vec::new(),
            inspections: 0
        })));
    }

    let mut playing_monkeys = Vec::new();

    
    for (monkey, targets) in monkeys.iter().zip(throw_monkeys) {
        monkey.borrow_mut().monkeys = vec![monkeys[targets[0]].clone(), monkeys[targets[1]].clone()];
        playing_monkeys.push(monkey.clone());
    }

    for _ in 0..20 {
        for p in playing_monkeys.iter() {
            p.borrow_mut().step();
        }
    }

    let mut result = playing_monkeys.into_iter().map(|p| p.borrow().inspections).collect::<Vec<usize>>();
    result.sort_by(|a, b| b.cmp(a));

    Box::new(result[0] * result[1])
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut monkeys = Vec::new();
    let mut throw_monkeys = Vec::new();

    for string_monkey in input.split("\n\n") {
        let mut monkey_attributes = string_monkey.lines();

        let _id = monkey_attributes.next().unwrap()
            .trim().chars().nth(7).unwrap().to_digit(10).unwrap();
        let starting_items = monkey_attributes.next().unwrap()
            .trim()
            .split_once(": ").unwrap().1
            .split(", ")
            .map(|i| u64::from_str_radix(i, 10).unwrap())
            .collect();
        let operation = match monkey_attributes.next().unwrap().trim().split_once("= old ").unwrap().1.split_once(" ").unwrap() {
            ("*", "old") => Operation::Square,
            ("+", n) => Operation::Add(u64::from_str_radix(n, 10).unwrap()),
            ("*", n) => Operation::Multiply(u64::from_str_radix(n, 10).unwrap()),
            _ => unreachable!()
        };
        let divisor = u64::from_str_radix(monkey_attributes.next().unwrap().trim().split(" ").last().unwrap(), 10).unwrap();
        let true_monkey = monkey_attributes.next().unwrap().trim().chars().last().unwrap().to_digit(10).unwrap();
        let false_monkey = monkey_attributes.next().unwrap().trim().chars().last().unwrap().to_digit(10).unwrap();

        throw_monkeys.push([true_monkey as usize, false_monkey as usize]);

        monkeys.push(Rc::new(RefCell::new(Monkey {
            items: starting_items,
            operation: operation,
            test_divisor: divisor,
            monkeys: Vec::new(),
            inspections: 0
        })));
    }

    let mut playing_monkeys = Vec::new();

    
    for (monkey, targets) in monkeys.iter().zip(throw_monkeys) {
        monkey.borrow_mut().monkeys = vec![monkeys[targets[0]].clone(), monkeys[targets[1]].clone()];
        playing_monkeys.push(monkey.clone());
    }

    let mod_prod: u64 = monkeys.iter().map(|m| m.borrow().test_divisor).product();

    for _ in 0..10000 {
        for p in playing_monkeys.iter() {
            p.borrow_mut().step_without_relief(mod_prod);
        }
    }

    let mut result = playing_monkeys.into_iter().map(|p| p.borrow().inspections).collect::<Vec<usize>>();
    result.sort_by(|a, b| b.cmp(a));


    Box::new(result[0] * result[1])
}
