use rand::prelude::*;
use proconio::input;
use itertools::Itertools;

const N: usize = 150;

#[derive(Clone, Debug)]
pub struct Input {
    pub pos: Vec<Vector2>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", N)?;
        writeln!(f, "{}", self.pos.iter().map(|&p| format!("{} {}", p.0, p.1)).join("\n"))
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize,
        pos: [[isize; 2]; n],
    }
    Input { pos: pos.iter().map(|x| Vector2(x[0], x[1])).collect_vec(), }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub route: Vec<usize>,
}

pub fn parse_output(f: &str) -> Result<Output, String> {
    let ans = match f.split('\n').map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, std::num::ParseIntError>>() {
        Ok(ans) => ans,
        Err(err) => { return Err(err.to_string()); },
    };

    if ans.len() != N + 1 {
        return Err("output length must be N + 1".to_string());
    }

    if ans[0] != 1 || ans[N] != 1 {
        return Err("output must be from city #1 to city #1".to_string());
    }

    if ans[1..N].iter().cloned().sorted().collect_vec() != (2..=N).collect_vec() {
        return Err("output must connect all cities".to_string());
    }

    Ok(Output { route: ans.iter().map(|&x| x - 1).collect_vec(), })
}

pub fn compute_score(input: &Input, output: &Output) -> (isize, String) {
    ((1e6 / output.route.windows(2)
        .map(|v| (input.pos[v[0]].dist(&input.pos[v[1]]))).sum::<f64>()) as isize,
    "".to_string())
}

const MIN_X: usize = 0;
const MAN_X: usize = 1000;
const MIN_Y: usize = 0;
const MAN_Y: usize = 1000;

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut pos = Vec::new();
    for _ in 0..N {
        let x = rng.gen_range(MIN_X..=MAN_X) as isize;
        let y = rng.gen_range(MIN_Y..=MAN_Y) as isize;
        pos.push(Vector2(x, y));
    }
    Input { pos }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2(isize, isize);

#[allow(dead_code)]
impl Vector2 {
    fn dot(&self, rhs: &Self) -> isize { self.0 * rhs.0 + self.1 * rhs.1 }
    fn det(&self, rhs: &Self) -> isize { self.0 * rhs.1 - self.1 * rhs.0 }
    fn norm1(&self) -> usize { (self.0.abs() + self.1.abs()) as usize }
    fn norm2(&self) -> usize { self.dot(&self) as usize }
    fn dist(&self, rhs: &Self) -> f64 { ((*self - *rhs).norm2() as f64).sqrt() }
}

impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
