use std::collections::BTreeMap;

fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_map = BTreeMap::<char, i32>::new();

    /** Solution trouvé dans la communauté mais ne gère que les caractères ASCII alors que le
     * but de cette conversion est le multi-langues.
     */
    // h.iter().flat_map(|(&score, letters)| letters.iter().map(move |char| (char.to_ascii_lowercase(), score)))
    // .collect()

    for (key, chars) in h {
        for char in chars {
            if let Some(lowc) = char.to_lowercase().next() {
                new_map.insert(lowc, *key);
            }
        }
    }

    new_map
}

#[test]
fn transform_one_value() {
    let input = input_from(&[(1, vec!['A'])]);
    let expected = expected_from(&[('a', 1)]);
    assert_eq!(expected, transform(&input));
}
#[test]
fn transform_more_values() {
    let input = input_from(&[(1, vec!['A', 'E', 'I', 'O', 'U'])]);
    let expected = expected_from(&[('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]);
    assert_eq!(expected, transform(&input));
}
#[test]
fn more_keys() {
    let input = input_from(&[(1, vec!['A', 'E']), (2, vec!['D', 'G'])]);
    let expected = expected_from(&[('a', 1), ('e', 1), ('d', 2), ('g', 2)]);
    assert_eq!(expected, transform(&input));
}
#[test]
fn full_dataset() {
    let input = input_from(&[
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ]);
    let expected = expected_from(&[
        ('a', 1),
        ('b', 3),
        ('c', 3),
        ('d', 2),
        ('e', 1),
        ('f', 4),
        ('g', 2),
        ('h', 4),
        ('i', 1),
        ('j', 8),
        ('k', 5),
        ('l', 1),
        ('m', 3),
        ('n', 1),
        ('o', 1),
        ('p', 3),
        ('q', 10),
        ('r', 1),
        ('s', 1),
        ('t', 1),
        ('u', 1),
        ('v', 4),
        ('w', 4),
        ('x', 8),
        ('y', 4),
        ('z', 10),
    ]);
    assert_eq!(expected, transform(&input));
}
fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
    v.iter().cloned().collect()
}
fn expected_from(v: &[(char, i32)]) -> BTreeMap<char, i32> {
    v.iter().cloned().collect()
}