use crate::Sudoku;
use crate::strategy::*;
use crate::strategy::deduction::Deduction;
use crate::board::*;

#[test]
fn trace_solver_workflow() {
    // 简单的例子：第一行只缺一个数字
    let puzzle = "12345678.........................................................................";
    let sudoku = Sudoku::from_str_line(puzzle).unwrap();
    
    let mut solver = StrategySolver::from_sudoku(sudoku);
    
    println!("=== 初始状态 ===");
    println!("deduced_entries: {} 条", solver.deduced_entries.len());
    println!("n_solved: {}", solver.n_solved);
    println!("next_deduced: {}", solver.cell_poss_digits.next_deduced);
    
    // 手动展示第一行最后一个单元格的可能数字
    println!("\n=== 处理前 Cell(8) 的状态 ===");
    let _ = solver._update_cell_poss_house_solved(false, false);
    let cell_8 = Cell::new(8);
    println!("Cell(8) 可能数字: {:?}", solver.cell_poss_digits.state[cell_8]);
    
    println!("\n=== 调用 NakedSingles ===");
    let result = solver.find_naked_singles(false);
    println!("Result: {:?}", result);
    println!("找到推导: {}", solver.deductions.len());
    println!("deduced_entries: {} 条", solver.deduced_entries.len());
    println!("n_solved: {}", solver.n_solved);
    
    // 查看找到的推导
    if let Some(last_deduction) = solver.deductions.last() {
        println!("最后的推导: {:?}", last_deduction);
    }
    
    // 查看 Cell(8) 现在的状态
    println!("\n=== 处理后 Cell(8) 的状态 ===");
    println!("Cell(8) 可能数字: {:?}", solver.cell_poss_digits.state[cell_8]);
    
    // 更新网格看结果
    solver.update_grid();
    println!("\n=== 最终网格 ===");
    for row in 0..9 {
        for col in 0..9 {
            let idx = row * 9 + col;
            let val = solver.grid.state.0[idx];
            if val == 0 {
                print!(". ");
            } else {
                print!("{} ", val);
            }
        }
        println!();
    }
}

#[test] 
fn trace_hidden_single_workflow() {
    // 更复杂的例子，需要 HiddenSingle
    // 第一行：1-8都有，缺9
    // 第一列：第一个是1，其他位置某些不能是9（通过其他clues限制）
    let puzzle = "12345678.9.......................................................................";
    
    let sudoku = Sudoku::from_str_line(puzzle).unwrap();
    let mut solver = StrategySolver::from_sudoku(sudoku);
    
    println!("=== 初始状态 ===");
    println!("初始 clues: {}", solver.deduced_entries.len());
    
    // 先处理 NakedSingles
    println!("\n=== NakedSingles ===");
    let _ = solver.find_naked_singles(false);
    println!("找到 {} 个 NakedSingles", 
             solver.deductions.iter().filter(|d| matches!(d, Deduction::NakedSingles(_))).count());
    
    // 再处理 HiddenSingles
    println!("\n=== HiddenSingles ===");
    let _ = solver.find_hidden_singles(false);
    println!("找到 {} 个 HiddenSingles",
             solver.deductions.iter().filter(|d| matches!(d, Deduction::HiddenSingles(_, _))).count());
    
    println!("\n总共 n_solved: {}", solver.n_solved);
}