use rand::seq::SliceRandom;
use rand::Rng;
use std::io::stdin;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    fn get_distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct TravelingSalesman {
    n: u32,
    population: Vec<Vec<u32>>,
    population_size: usize,
    towns: Vec<Point>,
}

impl TravelingSalesman {
    const POPULATION_SIZE: usize = 100;
    const POPULATION_KEPT_AMOUNT: usize = TravelingSalesman::POPULATION_SIZE / 2;
    const ITERATIONS_AFTER_BEST: u32 = 50;

    fn population_can_be_generated(n: u32) -> usize {
        let mut variants_count = 1;

        for i in 2..n {
            variants_count *= i;

            if variants_count >= TravelingSalesman::POPULATION_SIZE as u32 {
                return TravelingSalesman::POPULATION_SIZE;
            }
        }

        variants_count as usize
    }

    fn new(n: u32) -> Self {
        let towns = TravelingSalesman::generate_towns(n);
        let population = TravelingSalesman::generate_popultaion(&towns);

        TravelingSalesman {
            n,
            population_size: population.len(),
            population,
            towns,
        }
    }

    fn generate_towns(n: u32) -> Vec<Point> {
        let mut towns = vec![];
        towns.reserve(n as usize);
        let mut rng = rand::thread_rng();

        for _ in 0..n {
            towns.push(Point::new(
                rng.gen_range(0f32, 100f32),
                rng.gen_range(0f32, 100f32),
            ));
        }

        towns
    }

    fn generate_popultaion(towns: &Vec<Point>) -> Vec<Vec<u32>> {
        let population_size = TravelingSalesman::population_can_be_generated(towns.len() as u32);

        let mut population = vec![];
        population.reserve(population_size as usize);
        let mut rng = rand::thread_rng();

        while population.len() < population_size as usize {
            let mut member: Vec<u32> = (0..towns.len()).map(|x| x as u32).collect();
            member.shuffle(&mut rng);

            if !population.contains(&member) {
                population.push(member);
            }
        }

        population
    }

    fn get_member_fitness(&self, member: &Vec<u32>) -> f32 {
        let mut res = 0f32;

        for i in 0..member.len() - 1 {
            res += self.towns[member[i as usize] as usize]
                .get_distance_to(&self.towns[member[i + 1 as usize] as usize]);
        }

        res
    }

    fn crossover_to_full(&mut self) {
        if self.population.len() == 1 {
            unimplemented!();
        }

        let mut rng = rand::thread_rng();
        let mut parents = self.population.clone();
        parents.shuffle(&mut rng);

        let mut i = 0;

        while self.population.len() < self.population_size {
            let child = TravelingSalesman::crossover(&parents[i], &parents[i + 1]);

            if !self.population.contains(&child) {
                self.population.push(child);
            }

            i += 2;

            if i >= parents.len() - 1 {
                parents.shuffle(&mut rng);
                i = 0;
            }
        }
    }

    fn crossover(p1: &Vec<u32>, p2: &Vec<u32>) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut child = vec![];
        let idx1 = rng.gen_range(0, p1.len() - 1);
        let idx2 = rng.gen_range(idx1, p1.len());

        for i in idx1..idx2 {
            child.push(p1[i]);
        }

        let mut i = idx2;

        while child.len() < p2.len() {
            if !child.contains(&p2[i]) {
                child.push(p2[i]);
            }

            i += 1;

            if i >= p2.len() {
                i = 0;
            }
        }

        child
    }

    fn mutate_population(&mut self) {
        let mut rng = rand::thread_rng();
        self.population.shuffle(&mut rng);

        for i in 0..self.population.len() / 2 {
            let new_member = self.mutate(&self.population[i]);

            if !self.population.contains(&new_member) {
                self.population[i] = new_member;
            }
        }
    }

    fn mutate(&self, member: &Vec<u32>) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let idx1 = rng.gen_range(0, member.len() - 1);
        let idx2 = rng.gen_range(idx1, member.len());

        let mut res = member.clone();
        let length = idx2 - idx1;

        for i in 0..length {
            res[idx1 + i] = member[idx2 - i - 1];
        }

        res
    }

    fn solve(&mut self) -> Vec<u32> {
        let mut best = vec![];
        let mut best_score = None;
        let mut found_iter = 0;
        let mut i = 0;
        let mut prints_done = 0;

        loop {
            let mut member_scores: Vec<(usize, f32)> = self
                .population
                .iter()
                .enumerate()
                .map(|(i, x)| (i, self.get_member_fitness(&x)))
                .collect();

            member_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            member_scores.truncate(TravelingSalesman::POPULATION_KEPT_AMOUNT);

            if best_score.is_none() || member_scores[0].1 < best_score.unwrap() {
                best = self.population[member_scores[0].0].clone();
                best_score = Some(member_scores[0].1);
                found_iter = i;
            } else if i > found_iter + TravelingSalesman::ITERATIONS_AFTER_BEST {
                break;
            }

            self.population = member_scores
                .iter()
                .map(|&(i, _)| self.population[i].clone())
                .collect();
            self.crossover_to_full();
            self.mutate_population();

            if i == 10 || (i >= 10 && found_iter == i && prints_done < 4) {
                /*println!(
                    "Best score is {} for {:?} on iteration {}",
                    best_score.unwrap(),
                    best,
                    found_iter
                );*/

                println!("{}", best_score.unwrap());
                prints_done += 1;
            }

            i += 1;
        }

        if prints_done == 4 {
            println!("{}", best_score.unwrap());
            /*println!(
                "Best score is {} for {:?}, found on iteration {}",
                best_score.unwrap(),
                best,
                found_iter
            );*/
        }

        best
    }
}

fn print_to_svg(ts: &TravelingSalesman, solution: &Vec<u32>) {
    let mut svg = "<!DOCTYPE html>
<html>
<body>

<svg height='1000' width='1000'>
"
    .to_string();

    for m in solution {
        let town = &ts.towns[*m as usize];
        let circle = format!(
            "<circle cx='{}' cy='{}' r='5' stroke='black' stroke-wirdth='3' fill='red'/>",
            town.x * 10f32,
            town.y * 10f32
        );
        svg.push_str(&circle);
    }

    for i in 0..solution.len() - 1 as usize {
        let town1 = &ts.towns[solution[i] as usize];
        let town2 = &ts.towns[solution[i + 1] as usize];
        let line =
            format!(
            "<line x1='{}' y1='{}' x2='{}' y2='{}' style='stroke:rgb(255,0,0);stroke-width:2' />",
            town1.x * 10f32, town1.y * 10f32, town2.x * 10f32, town2.y * 10f32
        );
        svg.push_str(&line);
    }

    svg.push_str("</svg></body></html>");
    std::fs::write("test.html", svg).unwrap();
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: u32 = buf.trim().parse().unwrap();

    let mut traveling_salesman = TravelingSalesman::new(n);
    let solution = traveling_salesman.solve();
    print_to_svg(&traveling_salesman, &solution);
}
