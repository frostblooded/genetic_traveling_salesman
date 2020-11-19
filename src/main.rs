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
    towns: Vec<Point>,
}

impl TravelingSalesman {
    const POPULATION_SIZE: usize = 4;
    const POPULATION_KEPT_AMOUNT: usize = 2;

    fn new(n: u32) -> Self {
        let towns = TravelingSalesman::generate_towns(n);
        let population =
            TravelingSalesman::generate_popultaion(TravelingSalesman::POPULATION_SIZE, &towns);

        TravelingSalesman {
            n,
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

    fn generate_popultaion(k: usize, towns: &Vec<Point>) -> Vec<Vec<u32>> {
        let mut population = vec![];
        population.reserve(k);
        let mut rng = rand::thread_rng();

        for _ in 0..k {
            let mut member: Vec<u32> = (0..towns.len()).map(|x| x as u32).collect();
            member.shuffle(&mut rng);
            population.push(member);
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

        while self.population.len() < TravelingSalesman::POPULATION_SIZE as usize {
            let child = TravelingSalesman::crossover(&parents[i], &parents[i + 1]);
            self.population.push(child);
            i += 2;

            if i >= parents.len() - 1 {
                i = 0;
            }
        }
    }

    fn crossover(p1: &Vec<u32>, p2: &Vec<u32>) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut child = vec![];
        let idx = rng.gen_range(0, p1.len());

        for i in 0..p1.len() {
            if i < idx {
                child.push(p1[i]);
            } else {
                child.push(p2[i]);
            }
        }

        child
    }

    fn solve(&mut self) {
        for _ in 0..1 {
            let mut member_scores: Vec<(usize, f32)> = self
                .population
                .iter()
                .enumerate()
                .map(|(i, x)| (i, self.get_member_fitness(&x)))
                .collect();

            member_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            dbg!(&member_scores);
            member_scores.truncate(TravelingSalesman::POPULATION_KEPT_AMOUNT);
            dbg!(&member_scores);
            self.population = member_scores
                .iter()
                .map(|&(i, _)| self.population[i].clone())
                .collect();
            dbg!(&self.population);
            self.crossover_to_full();
            dbg!(&self.population);
        }
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: u32 = buf.trim().parse().unwrap();

    let mut traveling_salesman = TravelingSalesman::new(n);
    dbg!(&traveling_salesman);
    traveling_salesman.solve();
}
