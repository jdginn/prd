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

#[derive(Debug, Clone)]
struct Solution {
    n: usize,
    r: i32,
    f: i32,
    w: f64,
    max_build_height: i32,
    zero_percentage: f64,
}

#[derive(Debug)]
struct CombinedSolution {
    solutions: Vec<Solution>,
    combined_max_height: i32,
    total_width: f64,
    combined_zero_percentage: f64,
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

fn find_combined_solutions(valid_solutions: &[Solution]) -> Vec<CombinedSolution> {
    let mut combined_solutions = Vec::new();
    
    // Filter solutions with w >= 6
    let combinable_solutions: Vec<&Solution> = valid_solutions
        .iter()
        .filter(|s| s.w >= 6.0)
        .collect();
    
    if combinable_solutions.is_empty() {
        return combined_solutions;
    }
    
    // Try combinations of 2-20 solutions (to reach width of 120+)
    // We'll use a greedy approach: pick the best solutions and keep adding until we reach 120-140
    
    // For efficiency, we'll try combinations of the same solution repeated
    for sol in &combinable_solutions {
        // Calculate how many copies we need
        let min_copies = (120.0 / sol.w).ceil() as usize;
        let max_copies = (140.0 / sol.w).floor() as usize;
        
        for count in min_copies..=max_copies {
            let total_width = sol.w * count as f64;
            if total_width >= 120.0 && total_width <= 140.0 {
                let combined_max_height = sol.max_build_height;
                let combined_zero_percentage = sol.zero_percentage;
                
                combined_solutions.push(CombinedSolution {
                    solutions: vec![(*sol).clone(); count],
                    combined_max_height,
                    total_width,
                    combined_zero_percentage,
                });
            }
        }
    }
    
    // Also try combinations of 2 different solutions repeated
    for i in 0..combinable_solutions.len() {
        for j in i+1..combinable_solutions.len() {
            let sol1 = combinable_solutions[i];
            let sol2 = combinable_solutions[j];
            
            // Try different counts of each
            for count1 in 0..=20 {
                for count2 in 0..=20 {
                    if count1 + count2 < 2 {
                        continue; // Need at least 2 total
                    }
                    
                    let total_width = sol1.w * count1 as f64 + sol2.w * count2 as f64;
                    
                    // Early termination if we're already over the limit
                    if total_width > 140.0 {
                        break;
                    }
                    
                    if total_width >= 120.0 && total_width <= 140.0 {
                        let combined_max_height = sol1.max_build_height.max(sol2.max_build_height);
                        
                        let total_wells = (sol1.n - 1) * count1 + (sol2.n - 1) * count2;
                        let total_zero_wells = 
                            ((sol1.zero_percentage / 100.0) * (sol1.n - 1) as f64 * count1 as f64).round() as usize
                            + ((sol2.zero_percentage / 100.0) * (sol2.n - 1) as f64 * count2 as f64).round() as usize;
                        let combined_zero_percentage = (total_zero_wells as f64 / total_wells as f64) * 100.0;
                        
                        let mut solutions = vec![sol1.clone(); count1];
                        solutions.extend(vec![sol2.clone(); count2]);
                        
                        combined_solutions.push(CombinedSolution {
                            solutions,
                            combined_max_height,
                            total_width,
                            combined_zero_percentage,
                        });
                    }
                }
            }
        }
    }
    
    // Sort by combined_max_height (lower is better), then by how close to 130 the width is
    combined_solutions.sort_by(|a, b| {
        a.combined_max_height.cmp(&b.combined_max_height)
            .then((a.total_width - 130.0).abs().partial_cmp(&(b.total_width - 130.0).abs()).unwrap_or(std::cmp::Ordering::Equal))
    });
    
    combined_solutions
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
    primitive_roots.insert(31, vec![3, 11, 12, 13, 17, 21, 22, 24]);
    
    let f_values = vec![0, 1, 2, 3, 4, 5, 6, 7];
    
    let mut valid_solutions = Vec::new();
    let mut all_successful_solutions = Vec::new();  // Track ALL successful solutions
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
                            
                            let solution = Solution {
                                n,
                                r,
                                f,
                                w,
                                max_build_height,
                                zero_percentage,
                            };
                            
                            // Store ALL successful solutions for combination purposes
                            all_successful_solutions.push(solution.clone());
                            
                            // Filter: percentage of wells with build_height = 0 must be within 25-50%
                            if zero_percentage >= 25.0 && zero_percentage <= 50.0 {
                                valid_solutions.push(solution);
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
            .then(a.zero_percentage.partial_cmp(&b.zero_percentage).unwrap_or(std::cmp::Ordering::Equal))
    });
    
    println!("Valid Solutions (25-50% wells with build_height = 0):");
    println!("Total valid solutions: {}", valid_solutions.len());
    println!("Total successful solutions (all): {}", all_successful_solutions.len());
    println!("Total errors encountered: {}", error_count);
    
    // Debug: Show w values in all successful solutions with w >= 6
    let combinable_count = all_successful_solutions.iter().filter(|s| s.w >= 6.0).count();
    println!("Solutions with w >= 6: {}", combinable_count);
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
    
    // Find and display combined solutions
    println!("\n{}", "=".repeat(80));
    println!("COMBINED SOLUTIONS (w >= 6, total_width >= 120)");
    println!("{}", "=".repeat(80));
    
    let combined_solutions = find_combined_solutions(&all_successful_solutions);
    
    if combined_solutions.is_empty() {
        println!("No valid combined solutions found.");
    } else {
        println!("Total combined solutions: {}\n", combined_solutions.len());
        
        // Print table header for combined solutions
        println!("{:>30} | {:>11} | {:>16} | {:>14}", 
                 "Solution Combination", "Total Width", "Max Build Height", "Zero % (0 bh)");
        println!("{}", "-".repeat(80));
        
        // Print top 20 combined solutions
        for (idx, combo) in combined_solutions.iter().take(20).enumerate() {
            // Count occurrences of each unique solution
            let mut solution_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for sol in &combo.solutions {
                let key = format!("N{}(r={},f={})", sol.n, sol.r, sol.f);
                *solution_counts.entry(key).or_insert(0) += 1;
            }
            
            // Format as "3×N7(r=3,f=1) + 2×N11(r=2,f=0)"
            let combo_str: Vec<String> = solution_counts.iter()
                .map(|(key, count)| {
                    if *count > 1 {
                        format!("{}×{}", count, key)
                    } else {
                        key.clone()
                    }
                })
                .collect();
            let combo_str = combo_str.join(" + ");
            
            println!("{:>30} | {:>11.2} | {:>16} | {:>13.2}%",
                     combo_str, combo.total_width, combo.combined_max_height, combo.combined_zero_percentage);
            
            // Show more details for the top 3
            if idx < 3 {
                println!("   Details:");
                let mut counted_solutions: std::collections::HashMap<String, (usize, &Solution)> = std::collections::HashMap::new();
                for sol in &combo.solutions {
                    let key = format!("N{}(r={},f={})", sol.n, sol.r, sol.f);
                    counted_solutions.entry(key.clone())
                        .and_modify(|(count, _)| *count += 1)
                        .or_insert((1, sol));
                }
                
                for (_key, (count, sol)) in counted_solutions.iter() {
                    if *count > 1 {
                        println!("     - {}× N={}, r={}, f={}, w={:.2}, max_height={}, zero%={:.2}%",
                                 count, sol.n, sol.r, sol.f, sol.w, sol.max_build_height, sol.zero_percentage);
                    } else {
                        println!("     - N={}, r={}, f={}, w={:.2}, max_height={}, zero%={:.2}%",
                                 sol.n, sol.r, sol.f, sol.w, sol.max_build_height, sol.zero_percentage);
                    }
                }
            }
        }
        
        // Show the best combined solution
        if !combined_solutions.is_empty() {
            println!();
            println!("Best combined solution:");
            let best_combo = &combined_solutions[0];
            
            // Count occurrences
            let mut solution_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for sol in &best_combo.solutions {
                let key = format!("N{}(r={},f={})", sol.n, sol.r, sol.f);
                *solution_counts.entry(key).or_insert(0) += 1;
            }
            
            let combo_str: Vec<String> = solution_counts.iter()
                .map(|(key, count)| {
                    if *count > 1 {
                        format!("{}×{}", count, key)
                    } else {
                        key.clone()
                    }
                })
                .collect();
            
            println!("  Combination: {}", combo_str.join(" + "));
            println!("  Total Width: {:.2}", best_combo.total_width);
            println!("  Combined Max Build Height: {}", best_combo.combined_max_height);
            println!("  Combined Zero Percentage: {:.2}%", best_combo.combined_zero_percentage);
            println!("  Individual solutions:");
            
            let mut counted_solutions: std::collections::HashMap<String, (usize, &Solution)> = std::collections::HashMap::new();
            for sol in &best_combo.solutions {
                let key = format!("N{}(r={},f={})", sol.n, sol.r, sol.f);
                counted_solutions.entry(key.clone())
                    .and_modify(|(count, _)| *count += 1)
                    .or_insert((1, sol));
            }
            
            for (_key, (count, sol)) in counted_solutions.iter() {
                if *count > 1 {
                    println!("    - {}× N={}, r={}, f={}, w={:.2}, max_height={}, zero%={:.2}%",
                             count, sol.n, sol.r, sol.f, sol.w, sol.max_build_height, sol.zero_percentage);
                } else {
                    println!("    - N={}, r={}, f={}, w={:.2}, max_height={}, zero%={:.2}%",
                             sol.n, sol.r, sol.f, sol.w, sol.max_build_height, sol.zero_percentage);
                }
            }
        }
    }
}
