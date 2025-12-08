use std::fmt::Display;

use crate::aoc::DaySolver;
use anyhow::bail;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        solve(input, 1000)
    }

    fn solve_part2(&self, _input: &Vec<String>) -> anyhow::Result<i64> {
        bail!("😱 Part 2 not yet implemented!")
    }
}

fn solve(_input: &Vec<String>, connections: i32) -> Result<i64, anyhow::Error> {
    let mut junctions: Vec<Junction> = _input
        .iter()
        .map(|x| x.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    let distances = calculate_distances(&junctions);
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut cons = 0;
    for Distance { a, b, .. } in distances {
        if cons == connections {
            break;
        }
        cons += 1;
        print!("{}: {a}-{b} -> ", cons);
        match (junctions[a].circuit, junctions[b].circuit) {
            (None, None) => {
                let len = circuits.len();
                junctions[a].circuit = Some(len);
                junctions[b].circuit = Some(len);
                circuits.push(vec![a, b]);
                println!("{len}: {:?}", circuits[len]);
            }
            (None, Some(b)) => {
                junctions[a].circuit = Some(b);
                circuits[b].push(a);
                println!("{b}: {:?}", circuits[b]);
            }
            (Some(a), None) => {
                junctions[b].circuit = Some(a);
                circuits[a].push(b);
                println!("{a}: {:?}", circuits[a]);
            }
            (Some(a), Some(b)) => {
                if a != b {
                    for c in circuits[b].clone() {
                        junctions[c].circuit = Some(a);
                        circuits[a].push(c);
                    }
                    circuits[b] = Vec::new();
                    println!("{a} += {b}: {:?}", circuits[a]);
                } else {
                    println!("in {a}")
                }
            }
        }
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    for (i, c) in circuits[0..3].iter().enumerate() {
        println!("c {i}: {c:?}")
    }
    Ok(circuits[0..3]
        .iter()
        .map(|c| {
            println!("{}", c.len());
            c.len() as i64
        })
        .product())
}

fn calculate_distances(junctions: &[Junction]) -> Vec<Distance> {
    let mut res = Vec::new();
    let len = junctions.len();
    for a in 0..len - 1 {
        for b in a + 1..len {
            res.push(Distance::new(&junctions, a, b));
        }
    }
    res.sort_by_key(|a| a.distance);
    // for (i, Distance { a, b, distance }) in res.iter().enumerate() {
    //     println!("{i}: {a}-{b} {distance}");
    // }
    res
}

#[derive(Debug)]
struct Junction {
    pos: (i32, i32, i32),
    circuit: Option<usize>,
}

impl Junction {
    fn distance2(&self, rhs: &Self) -> i64 {
        let (x1, y1, z1) = self.pos;
        let (x2, y2, z2) = rhs.pos;
        ((x1 - x2) as i64).pow(2) + ((y1 - y2) as i64).pow(2) + ((z1 - z2) as i64).pow(2)
    }
}

impl TryFrom<&String> for Junction {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let tripel = value
            .split(',')
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        if tripel.len() != 3 {
            bail!("invalid input {value}")
        }
        Ok(Junction {
            pos: (tripel[0], tripel[1], tripel[2]),
            circuit: None,
        })
    }
}

#[derive(Debug)]
struct Distance {
    a: usize,
    b: usize,
    distance: i64,
}

impl Distance {
    fn new(junctions: &[Junction], a: usize, b: usize) -> Self {
        Self {
            distance: junctions[a].distance2(&junctions[b]),
            a,
            b,
        }
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}: {}", self.a, self.b, self.distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .lines()
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_solve_part1() -> anyhow::Result<()> {
        for s in test_strings().iter().enumerate() {
            println!("{} {}", s.0, s.1)
        }

        assert_eq!(solve(&test_strings(), 10)?, 40);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part2(&test_strings())?, 0);
        Ok(())
    }
}
