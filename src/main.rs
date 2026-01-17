use std::vec;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// Odd prime number
    n: usize,
    /// A primitive root of N
    r: i32,
    /// Depth steps to fold behind neighboring wells
    f: i32,
    /// Well width
    w: i32,
}

#[derive(Clone)]
enum FoldDirection {
    Left,
    Right,
}

#[derive(Clone)]
struct Well {
    depth: i32,
    raw_height: i32,
    fold_direction: Option<FoldDirection>,
    fold_depth: i32,
    build_height: i32,
}

fn main() {
    let args = Cli::parse();
    let n = args.n;
    let r = args.r;
    let f = args.f;
    let w = args.w;

    let num_wells = n - 1;

    let mut wells: Vec<Well> = vec![
        Well {
            depth: 0,
            raw_height: 0,
            fold_direction: None,
            fold_depth: 0,
            build_height: 0,
        };
        num_wells
    ];

    let mut max_depth = 0;
    for i in 0..num_wells {
        let mut result = 1; // Start with 1, as any number to the power of 0 is 1
        let mut base = r;
        let mut exp = i as u32;
        let modulus = n as i32;

        // Perform modular exponentiation
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus; // Multiply result with base when exp is odd
            }
            base = (base * base) % modulus; // Square the base
            exp /= 2;
        }

        wells[i].depth = result;

        if wells[i].depth > max_depth {
            max_depth = wells[i].depth;
        }
    }

    for i in 0..num_wells {
        wells[i].raw_height = max_depth - wells[i].depth;
    }

    for i in 0..num_wells {
        let required_fold_depth = (wells[i].raw_height - f).clamp(-(n as i32), 0).abs();
        wells[i].build_height = wells[i].raw_height - f + required_fold_depth;
        if required_fold_depth > 0 {
            if required_fold_depth >= w {
                panic!(
                    "Well {}: Required fold depth {} is greater than the well width {}.\n\
                    This can happen if the difference between raw height and fold factor (f) is too large.",
                    i, required_fold_depth, w
                );
            }

            let is_leftmost = i == 0;
            let is_rightmost = i == num_wells - 1;

            if is_leftmost {
                if wells[1].fold_direction.is_some() {
                    panic!(
                        "Well {}: Cannot fold the leftmost well behind its right neighbor because the neighbor (1) already has a fold.",
                        i
                    );
                }
                if wells[1].raw_height < w {
                    panic!(
                        "Well {}: Cannot fold the leftmost well behind its right neighbor (1) because the neighbor's raw height of {} is less than the well width {}.",
                        i, wells[1].raw_height, w
                    );
                }
                wells[1].fold_direction = Some(FoldDirection::Left);
                wells[1].fold_depth = required_fold_depth;
            } else if is_rightmost {
                if wells[num_wells - 2].fold_direction.is_some() {
                    panic!(
                        "Well {}: Cannot fold the rightmost well behind its left neighbor because the neighbor (index {}) already has a fold.",
                        i,
                        num_wells - 2
                    );
                }
                if wells[num_wells - 2].raw_height < w {
                    panic!(
                        "Well {}: Cannot fold the rightmost well behind its left neighbor (index {}) because the neighbor's raw height of {} is less than the well width {}.",
                        i,
                        num_wells - 2,
                        wells[num_wells - 2].raw_height,
                        w
                    );
                }
                wells[num_wells - 2].fold_direction = Some(FoldDirection::Right);
                wells[num_wells - 2].fold_depth = required_fold_depth;
            } else {
                let left_neighbor_folded = match wells[i - 1].fold_direction {
                    Some(FoldDirection::Right) => true,
                    _ => false,
                };
                let right_neighbor_folded = match wells[i + 1].fold_direction {
                    Some(FoldDirection::Left) => true,
                    _ => false,
                };
                let left_neighbor_has_space = wells[i - 1].raw_height >= w;
                let right_neighbor_has_space = wells[i + 1].raw_height >= w;

                let can_fold_behind_left = !left_neighbor_folded && left_neighbor_has_space;
                let can_fold_behind_right = !right_neighbor_folded && right_neighbor_has_space;

                if !(can_fold_behind_left || can_fold_behind_right) {
                    let mut reasons = Vec::new();
                    if left_neighbor_folded {
                        reasons.push(format!(
                            "Left neighbor (index {}) already has a fold.",
                            i - 1
                        ));
                    }
                    if !left_neighbor_has_space {
                        reasons.push(format!(
                            "Left neighbor (index {}) is too short to fit a fold (height = {}, required width = {}).",
                            i - 1, wells[i - 1].raw_height, w
                        ));
                    }
                    if right_neighbor_folded {
                        reasons.push(format!(
                            "Right neighbor (index {}) already has a fold.",
                            i + 1
                        ));
                    }
                    if !right_neighbor_has_space {
                        reasons.push(format!(
                            "Right neighbor (index {}) is too short to fit a fold (height = {}, required width = {}).",
                            i + 1, wells[i + 1].raw_height, w
                        ));
                    }
                    panic!(
                        "Well {}: Cannot fold behind either neighbor. Reasons: {}",
                        i,
                        reasons.join(" ")
                    );
                } else if !can_fold_behind_left {
                    wells[i + 1].fold_direction = Some(FoldDirection::Left);
                    wells[i + 1].fold_depth = required_fold_depth;
                } else if !can_fold_behind_right {
                    wells[i - 1].fold_direction = Some(FoldDirection::Right);
                    wells[i - 1].fold_depth = required_fold_depth;
                } else {
                    // Prefer folding behind left neighbor
                    wells[i - 1].fold_direction = Some(FoldDirection::Right);
                    wells[i - 1].fold_depth = required_fold_depth;
                }
            }
        }
    }

    println!(
        "depths:           {:?}",
        wells.iter().map(|w| w.depth).collect::<Vec<i32>>()
    );
    println!(
        "heights:          {:?}",
        wells.iter().map(|w| w.raw_height).collect::<Vec<i32>>()
    );
    println!(
        "build heights:    {:?}",
        wells.iter().map(|w| w.build_height).collect::<Vec<i32>>()
    );
    println!(
        "fold directions:  {:?}",
        wells
            .iter()
            .map(|w| {
                match &w.fold_direction {
                    Some(FoldDirection::Left) => "L",
                    Some(FoldDirection::Right) => "R",
                    None => "-",
                }
            })
            .collect::<Vec<&str>>()
    );
    println!(
        "fold depths:      {:?}",
        wells.iter().map(|w| w.fold_depth).collect::<Vec<i32>>()
    );
    println!(
        "number of wells with build_height 0: {}",
        wells.iter().filter(|w| w.build_height == 0).count()
    );
    println!(
        "percentage of wells with build_height0: {:.2}%",
        (wells.iter().filter(|w| w.build_height == 0).count() as f64 / num_wells as f64) * 100.0
    );
}
