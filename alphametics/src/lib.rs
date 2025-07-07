use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse the equation
    let parts: Vec<&str> = input.split(" == ").collect();
    if parts.len() != 2 {
        return None;
    }
    
    let left_side = parts[0];
    let result = parts[1];
    
    // Split left side by " + " - this handles multiple operands
    let operands: Vec<&str> = left_side.split(" + ").collect();
    
    // Collect all unique letters
    let mut letters: Vec<char> = Vec::new();
    for operand in &operands {
        for ch in operand.chars() {
            if ch.is_alphabetic() && !letters.contains(&ch) {
                letters.push(ch);
            }
        }
    }
    for ch in result.chars() {
        if ch.is_alphabetic() && !letters.contains(&ch) {
            letters.push(ch);
        }
    }
    
    // Get leading letters (cannot be zero) - only for multi-digit numbers
    let mut leading_letters: Vec<char> = Vec::new();
    for operand in &operands {
        if operand.len() > 1 {
            if let Some(first_char) = operand.chars().next() {
                if first_char.is_alphabetic() && !leading_letters.contains(&first_char) {
                    leading_letters.push(first_char);
                }
            }
        }
    }
    if result.len() > 1 {
        if let Some(first_char) = result.chars().next() {
            if first_char.is_alphabetic() && !leading_letters.contains(&first_char) {
                leading_letters.push(first_char);
            }
        }
    }
    
    // Try all permutations of digits 0-9 for the letters
    let digits: Vec<u8> = (0..10).collect();
    
    // Generate all permutations of length letters.len() from digits
    if let Some(solution) = try_permutations(&letters, &digits, &leading_letters, &operands, result) {
        return Some(solution);
    }
    
    None
}

fn try_permutations(
    letters: &[char],
    digits: &[u8],
    leading_letters: &[char],
    operands: &[&str],
    result: &str,
) -> Option<HashMap<char, u8>> {
    if letters.len() > digits.len() {
        return None;
    }
    
    // Generate all permutations of digits for the letters
    let mut used_digits = vec![false; 10];
    let mut assignment = HashMap::new();
    
    if backtrack(
        letters,
        digits,
        leading_letters,
        operands,
        result,
        &mut used_digits,
        &mut assignment,
        0,
    ) {
        Some(assignment)
    } else {
        None
    }
}

fn backtrack(
    letters: &[char],
    digits: &[u8],
    leading_letters: &[char],
    operands: &[&str],
    result: &str,
    used_digits: &mut [bool],
    assignment: &mut HashMap<char, u8>,
    index: usize,
) -> bool {
    if index == letters.len() {
        // Check if this assignment is valid
        return is_valid_assignment(assignment, operands, result);
    }
    
    let letter = letters[index];
    
    for digit in 0..10 {
        if used_digits[digit] {
            continue;
        }
        
        // Check if this letter is a leading letter and digit is 0
        if leading_letters.contains(&letter) && digit == 0 {
            continue;
        }
        
        // Try this assignment
        used_digits[digit] = true;
        assignment.insert(letter, digit as u8);
        
        if backtrack(letters, digits, leading_letters, operands, result, used_digits, assignment, index + 1) {
            return true;
        }
        
        // Backtrack
        used_digits[digit] = false;
        assignment.remove(&letter);
    }
    
    false
}

fn is_valid_assignment(assignment: &HashMap<char, u8>, operands: &[&str], result: &str) -> bool {
    let mut sum = 0u64;
    
    // Calculate sum of all operands
    for operand in operands {
        if let Some(value) = word_to_number(operand, assignment) {
            sum += value;
        } else {
            return false;
        }
    }
    
    // Calculate result value
    if let Some(result_value) = word_to_number(result, assignment) {
        sum == result_value
    } else {
        false
    }
}

fn word_to_number(word: &str, assignment: &HashMap<char, u8>) -> Option<u64> {
    let mut number = 0u64;
    
    for ch in word.chars() {
        if let Some(&digit) = assignment.get(&ch) {
            number = number * 10 + digit as u64;
        } else {
            return None;
        }
    }
    
    Some(number)
}