#[derive(Debug)]
enum EnemyShapes {
    Rock,
    Paper,
    Scissors,
}
impl EnemyShapes {
    fn getShape(shape: char) -> EnemyShapes {
        match shape {
            'A' => EnemyShapes::Rock,
            'B' => EnemyShapes::Paper,
            'C' => EnemyShapes::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

#[derive(Debug)]
enum MyShapes {
    Rock,
    Paper,
    Scissors,
}

impl MyShapes {
    fn getShape(shape: char) -> MyShapes {
        match shape {
            'X' => MyShapes::Rock,
            'Y' => MyShapes::Paper,
            'Z' => MyShapes::Scissors,
            _ => panic!("Invalid shape"),
        }
    }

    fn getPoints(&self) -> u32 {
        match self {
            MyShapes::Rock => 1,
            MyShapes::Paper => 2,
            MyShapes::Scissors => 3,
        }
    }

    fn getResult(&self, enemyShape: &EnemyShapes) -> Result {
        match self {
            MyShapes::Rock => match enemyShape {
                EnemyShapes::Rock => Result::Draw,
                EnemyShapes::Paper => Result::Lose,
                EnemyShapes::Scissors => Result::Win,
            },
            MyShapes::Paper => match enemyShape {
                EnemyShapes::Rock => Result::Win,
                EnemyShapes::Paper => Result::Draw,
                EnemyShapes::Scissors => Result::Lose,
            },
            MyShapes::Scissors => match enemyShape {
                EnemyShapes::Rock => Result::Lose,
                EnemyShapes::Paper => Result::Win,
                EnemyShapes::Scissors => Result::Draw,
            },
        }
    }
}

#[derive(Debug)]
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Result {
    fn getResult(chr: char) -> Result {
        match chr {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Invalid result"),
        }
    }

    fn getMyShape(&self, enemyShape: &EnemyShapes) -> MyShapes {
        match self {
            Result::Win => match enemyShape {
                EnemyShapes::Rock => MyShapes::Paper,
                EnemyShapes::Paper => MyShapes::Scissors,
                EnemyShapes::Scissors => MyShapes::Rock,
            },
            Result::Draw => match enemyShape {
                EnemyShapes::Rock => MyShapes::Rock,
                EnemyShapes::Paper => MyShapes::Paper,
                EnemyShapes::Scissors => MyShapes::Scissors,
            },
            Result::Lose => match enemyShape {
                EnemyShapes::Rock => MyShapes::Scissors,
                EnemyShapes::Paper => MyShapes::Rock,
                EnemyShapes::Scissors => MyShapes::Paper,
            },
        }
    }

    fn getPoints(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

fn part1getPointsForRound(line: &str) -> u32 {
    let (enemyShape, myShape) = (
        EnemyShapes::getShape(line.chars().nth(0).unwrap()),
        MyShapes::getShape(line.chars().nth(line.len() - 1).unwrap()),
    );

    let res = myShape.getResult(&enemyShape);
    let points: u32 = myShape.getPoints() + res.getPoints();

    println!(
        "Enemy: {:?}\t Me: {:?} \t Result: {:?} \t Points: {}",
        enemyShape, myShape, res, points
    );

    return points;
}

fn part2getPointsForRound(line: &str) -> u32 {
    let (enemyShape, result) = (
        EnemyShapes::getShape(line.chars().nth(0).unwrap()),
        Result::getResult(line.chars().nth(line.len() - 1).unwrap()),
    );

    let myShape = result.getMyShape(&enemyShape);
    let points: u32 = result.getPoints() + myShape.getPoints();

    println!(
        "Enemy: {:?}\t Me: {:?} \t Result: {:?} \t Points: {}",
        enemyShape, myShape, result, points
    );

    return points;
}

pub fn solve() {
    println!("--- Day 2 : Rock Paper Scissors ---");
    let input = include_str!("./input.txt");
    let mut acc = 0;

    input
        .lines()
        .for_each(|line| acc += part1getPointsForRound(line));

    println!("Part 1 Total points: {}", acc);

    acc = 0;

    input
        .lines()
        .for_each(|line| acc += part2getPointsForRound(line));

    println!("Part 2 Total points: {}", acc);
}
