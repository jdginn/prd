#[derive(Clone)]
enum FoldDirection {
    Left,
    Right,
}

#[derive(Clone)]
pub struct Well {
    pub depth: i32,
    pub raw_height: i32,
    pub fold_direction: Option<FoldDirection>,
    pub fold_depth: i32,
    pub build_height: i32,
}

fn calculate(n: usize, r: i32, f: i32, w: f64) -> Result<Vec<Well>, String> {
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
            if required_fold_depth as f64 >= w {
                return Err(format!(
                    "Well {}: Required fold depth {} is greater than the well width {}. \
                    This can happen if the difference between raw height and fold factor (f) is too large.",
                    i, required_fold_depth, w
                ));
            }

            let is_leftmost = i == 0;
            let is_rightmost = i == num_wells - 1;

            if is_leftmost {
                if wells[1].fold_direction.is_some() {
                    return Err(format!(
                        "Well {}: Cannot fold the leftmost well behind its right neighbor because the neighbor (1) already has a fold.",
                        i
                    ));
                }
                if (wells[1].raw_height as f64) < w {
                    return Err(format!(
                        "Well {}: Cannot fold the leftmost well behind its right neighbor (1) because the neighbor's raw height of {} is less than the well width {}.",
                        i, wells[1].raw_height, w
                    ));
                }
                wells[1].fold_direction = Some(FoldDirection::Left);
                wells[1].fold_depth = required_fold_depth;
            } else if is_rightmost {
                if wells[num_wells - 2].fold_direction.is_some() {
                    return Err(format!(
                        "Well {}: Cannot fold the rightmost well behind its left neighbor because the neighbor (index {}) already has a fold.",
                        i,
                        num_wells - 2
                    ));
                }
                if (wells[num_wells - 2].raw_height as f64) < w {
                    return Err(format!(
                        "Well {}: Cannot fold the rightmost well behind its left neighbor (index {}) because the neighbor's raw height of {} is less than the well width {}.",
                        i,
                        num_wells - 2,
                        wells[num_wells - 2].raw_height,
                        w
                    ));
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
                let left_neighbor_has_space = wells[i - 1].raw_height as f64 - wells[i].build_height as f64 > w;
                let right_neighbor_has_space = wells[i + 1].raw_height as f64 - wells[i].build_height as f64 > w;

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
                    return Err(format!(
                        "Well {}: Cannot fold behind either neighbor. Reasons: {}",
                        i,
                        reasons.join(" ")
                    ));
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
    
    Ok(wells)
}

fn print_wells_table(wells: &[Well]) {
    let headers = vec![
        "Well",
        "Depth",
        "Height",
        "Build Height",
        "Fold Dir",
        "Fold Depth",
    ];
    let mut rows = Vec::new();

    for (i, well) in wells.iter().enumerate() {
        // Convert each row as string entries
        rows.push(vec![
            i.to_string(),
            well.depth.to_string(),
            well.raw_height.to_string(),
            well.build_height.to_string(),
            match &well.fold_direction {
                Some(FoldDirection::Left) => "L".to_string(),
                Some(FoldDirection::Right) => "R".to_string(),
                None => "-".to_string(),
            },
            well.fold_depth.to_string(),
        ]);
    }

    // Find column widths by taking the maximum length of all elements (headers + rows)
    let mut col_widths = vec![0; headers.len()];
    for (i, header) in headers.iter().enumerate() {
        col_widths[i] = header.len(); // Start with the header length
    }
    for row in &rows {
        for (i, value) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(value.len());
        }
    }

    // Function to right-align and pad a string
    let format_cell =
        |value: &str, width: usize| -> String { format!("{:>width$}", value, width = width) };

    // Print header
    println!(
        "{}",
        headers
            .iter()
            .enumerate()
            .map(|(i, header)| format_cell(header, col_widths[i]))
            .collect::<Vec<_>>()
            .join(" | ")
    );

    // Print a line separator
    println!(
        "{}",
        col_widths
            .iter()
            .map(|&w| "-".repeat(w))
            .collect::<Vec<_>>()
            .join("-|-")
    );

    // Print rows
    for row in rows {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(i, value)| format_cell(value, col_widths[i]))
                .collect::<Vec<_>>()
                .join(" | ")
        );
    }

    // Additional final statistics
    let num_wells = wells.len();
    let zero_build_height_count = wells.iter().filter(|w| w.build_height == 0).count();
    println!();
    println!(
        "Number of wells with build_height = 0: {}",
        zero_build_height_count
    );
    println!(
        "Percentage of wells with build_height = 0: {:.2}%",
        (zero_build_height_count as f64 / num_wells as f64) * 100.0
    );
}

fn main() {
    use std::collections::HashMap;
    
    // Define parameter ranges
    let n_values = vec![7, 11, 13, 17, 19, 23, 29, 31];
    
    let mut primitive_roots: HashMap<usize, Vec<i32>> = HashMap::new();
    primitive_roots.insert(7, vec![3, 5]);
    primitive_roots.insert(11, vec![2, 6, 7, 8]);
    primitive_roots.insert(13, vec![2, 6, 7, 11]);
    primitive_roots.insert(17, vec![3, 5, 6, 7, 10, 11, 12, 14]);
    primitive_roots.insert(19, vec![2, 3, 10, 13, 14, 15]);
    primitive_roots.insert(23, vec![5, 7, 10, 11, 14, 15, 17, 19, 20, 21]);
    primitive_roots.insert(29, vec![2, 3, 8, 10, 11, 14, 15, 18, 19, 21, 26, 27]);
    primitive_roots.insert(31, vec![3]); // Placeholder, at least one primitive root
    
    let f_values = vec![0, 1, 2, 3, 4, 5, 6, 7];
    
    #[derive(Debug)]
    struct Solution {
        n: usize,
        r: i32,
        f: i32,
        w: f64,
        max_build_height: i32,
        zero_percentage: f64,
    }
    
    let mut valid_solutions = Vec::new();
    let mut error_count = 0;
    
    println!("Testing all combinations...\n");
    
    for &n in &n_values {
        let w = (130.0 / n as f64).max(2.5);
        
        if let Some(roots) = primitive_roots.get(&n) {
            for &r in roots {
                for &f in &f_values {
                    match calculate(n, r, f, w) {
                        Ok(wells) => {
                            let max_build_height = wells.iter().map(|w| w.build_height).max().unwrap_or(0);
                            let zero_count = wells.iter().filter(|w| w.build_height == 0).count();
                            let zero_percentage = (zero_count as f64 / wells.len() as f64) * 100.0;
                            
                            // Filter: percentage of wells with build_height = 0 must be within 25-50%
                            if zero_percentage >= 25.0 && zero_percentage <= 50.0 {
                                valid_solutions.push(Solution {
                                    n,
                                    r,
                                    f,
                                    w,
                                    max_build_height,
                                    zero_percentage,
                                });
                            }
                        }
                        Err(_e) => {
                            error_count += 1;
                            // Uncomment to see errors:
                            // eprintln!("Error for n={}, r={}, f={}, w={:.2}: {}", n, r, f, w, e);
                        }
                    }
                }
            }
        }
    }
    
    // Sort by max_build_height (lower is better), then by zero_percentage
    valid_solutions.sort_by(|a, b| {
        a.max_build_height.cmp(&b.max_build_height)
            .then(a.zero_percentage.partial_cmp(&b.zero_percentage).unwrap())
    });
    
    println!("Valid Solutions (25-50% wells with build_height = 0):");
    println!("Total valid solutions: {}", valid_solutions.len());
    println!("Total errors encountered: {}", error_count);
    println!();
    
    // Print table header
    println!("{:>3} | {:>3} | {:>3} | {:>6} | {:>16} | {:>14}", 
             "N", "r", "f", "w", "Max Build Height", "Zero % (0 bh)");
    println!("{}", "-".repeat(70));
    
    // Print top 20 solutions or all if less than 20
    for solution in valid_solutions.iter().take(20) {
        println!("{:>3} | {:>3} | {:>3} | {:>6.2} | {:>16} | {:>13.2}%",
                 solution.n, solution.r, solution.f, solution.w, 
                 solution.max_build_height, solution.zero_percentage);
    }
    
    if valid_solutions.is_empty() {
        println!("No valid solutions found within the criteria.");
    } else {
        println!();
        println!("Best solution:");
        let best = &valid_solutions[0];
        println!("  N={}, r={}, f={}, w={:.2}", best.n, best.r, best.f, best.w);
        println!("  Max Build Height: {}", best.max_build_height);
        println!("  Percentage of wells with build_height = 0: {:.2}%", best.zero_percentage);
        
        // Show the detailed output for the best solution
        println!();
        println!("Detailed output for best solution:");
        if let Ok(wells) = calculate(best.n, best.r, best.f, best.w) {
            print_wells_table(&wells);
        }
    }
}
