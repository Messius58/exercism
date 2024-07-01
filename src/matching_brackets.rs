use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut c_open = Vec::<char>::new();
    let corr_chars = HashMap::<char, char>::from([('[',']'), ('{','}'), ('(', ')')]);
    // for access performance
    let opening_chars  = ['[', '{', '('];
    let ending_chars   = [']', '}', ')'];

    for c in string.chars() {
        if opening_chars.contains(&c) {
            c_open.push(c);
        } 
        else if ending_chars.contains(&c) {
            if let Some(o) = c_open.pop() {
                if corr_chars[&o] == c {
                    continue;
                }
            }
            return false;
        }
    }
    c_open.is_empty()
}

#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
}

#[test]
fn empty_string() {
    assert!(brackets_are_balanced(""));
}

#[test]
fn unpaired_brackets() {
    assert!(!brackets_are_balanced("[["));
}

#[test]
fn wrong_ordered_brackets() {
    assert!(!brackets_are_balanced("}{"));
}

#[test]
fn wrong_closing_bracket() {
    assert!(!brackets_are_balanced("{]"));
}

#[test]
fn paired_with_whitespace() {
    assert!(brackets_are_balanced("{ }"));
}

#[test]
fn partially_paired_brackets() {
    assert!(!brackets_are_balanced("{[])"));
}

#[test]
fn simple_nested_brackets() {
    assert!(brackets_are_balanced("{[]}"));
}

#[test]
fn several_paired_brackets() {
    assert!(brackets_are_balanced("{}[]"));
}

#[test]
fn paired_and_nested_brackets() {
    assert!(brackets_are_balanced("([{}({}[])])"));
}

#[test]
fn unopened_closing_brackets() {
    assert!(!brackets_are_balanced("{[)][]}"));
}

#[test]
fn unpaired_and_nested_brackets() {
    assert!(!brackets_are_balanced("([{])"));
}

#[test]
fn paired_and_wrong_nested_brackets() {
    assert!(!brackets_are_balanced("[({]})"));
}

#[test]
fn paired_and_incomplete_brackets() {
    assert!(!brackets_are_balanced("{}["));
}

#[test]
fn too_many_closing_brackets() {
    assert!(!brackets_are_balanced("[]]"));
}

#[test]
fn early_incomplete_brackets() {
    assert!(!brackets_are_balanced(")()"));
}

#[test]
fn early_mismatched_brackets() {
    assert!(!brackets_are_balanced("{)()"));
}

#[test]
fn math_expression() {
    assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}

#[test]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    assert!(brackets_are_balanced(input));
}
