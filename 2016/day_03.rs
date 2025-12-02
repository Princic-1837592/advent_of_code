//! https://adventofcode.com/2016/day/3
//! https://adventofcode.com/2016/day/3/input

use std::{fs::read_to_string, time::Instant};

type Triangle = (usize, usize, usize);
fn generic_solve(triangles: Vec<Triangle>) -> usize {
    triangles
        .iter()
        .filter(|triangle| {
            triangle.0 + triangle.1 > triangle.2
                && triangle.2 + triangle.1 > triangle.0
                && triangle.0 + triangle.2 > triangle.1
        })
        .count()
}

pub mod part1 {
    use super::{generic_solve, Triangle};

    fn parse(input: &str) -> Vec<Triangle> {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    pub fn solve(input: &str) -> usize {
        generic_solve(parse(input))
    }
}

pub mod part2 {
    use super::{generic_solve, Triangle};

    fn parse(input: &str) -> Vec<Triangle> {
        let numbers: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line.split_whitespace().flat_map(str::parse).collect())
            .collect();
        let mut result = vec![];
        for i in (0..numbers.len()).step_by(3) {
            for j in 0..3 {
                result.push((numbers[i][j], numbers[i + 1][j], numbers[i + 2][j]))
            }
        }
        result
    }

    pub fn solve(input: &str) -> usize {
        generic_solve(parse(input))
    }
}

pub fn main(test: bool) {
    let test_input = "775  785  361
  622  375  125
  297  839  375
  245   38  891
  503  463  849
  731  482  759
   29  734  734
  245  771  269
  261  315  904
  669   96  581
  570  745  156
  124  678  684
  472  360   73
  174  251  926
  406  408  976
  413  238  571
  375  554   22
  211  379  590
  271  821  847
  696  253  116
  513  972  959
  539  557  752
  168  362  550
  690  236  284
  434   91  818
  859  393  779
  620  313   56
  188  983  783
  799  900  573
  932  359  565
  357  670   69
  525   71   52
  640  654   43
  695  781  907
  676  680  938
   63  507  570
  985  492  587
  984   34  333
   25  489  399
  470  158   43
  715  491  617
  508  412  607
  365  446  743
  504  189  378
  225  424  517
  473   45  649
  847  927  424
  455  889  697
   64  230  846
  579  368  881
  639  536   74
  433  803  943
   14  629  963
  432  481  136
  781  625  323
  836  215  201
  620  614  366
  801  679  673
  745  376  326
  891  957  751
   64  430  347
  784  534  237
  740  485  470
  570  894  790"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2016/day_03_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
