
 // https://en.wikipedia.org/wiki/Knapsack_problem#Solving
#[derive(Debug)]
struct Item {
    weight: u32,
    value: u32,
}

fn compute(items: &[Item], memoisation: &mut Vec<Vec<i32>>, curr_weight: usize, curr_item_idx: usize) {
    // La 1ère ligne ne correspond à aucun item, les unwrap_or s'assurera de renvoyer 0
    if curr_item_idx == 0 || curr_weight == 0 {
        memoisation[curr_item_idx][curr_weight] = 0;
       return;
    }
    // Déterminer si l'élément précédent a bien été calculé sinon le faire
    if memoisation[curr_item_idx - 1][curr_weight] == -1 {
        compute(items, memoisation, curr_weight, curr_item_idx - 1);
    }
    // Si le poids de l'élément actuel dépasse le poids courant, on récupère la valeur max précédente
    if items[curr_item_idx - 1].weight > curr_weight as u32 {
        memoisation[curr_item_idx][curr_weight] = memoisation[curr_item_idx - 1][curr_weight];
    }
    // Si l'élément précédent de poids courant - le poids de l'élément actuel n'a pas été calculé on le fait
    else {
        if memoisation[curr_item_idx - 1][curr_weight - items[curr_item_idx - 1].weight as usize] == -1 {
            compute(items, memoisation, curr_weight - items[curr_item_idx - 1].weight as usize, curr_item_idx - 1);
        }
        let v_1 = memoisation[curr_item_idx - 1][curr_weight - items[curr_item_idx - 1].weight as usize] + items[curr_item_idx - 1].value as i32;
        let v_2 = memoisation[curr_item_idx - 1][curr_weight].max(v_1);
        memoisation[curr_item_idx][curr_weight] = v_2;
    }
}

// Possibilité d'utiliser le pgcd présent dans affine.rs pour réduire le nombre et l'incrément du poids.
// L'idéal serait d'utiliser un équivalent de HashMap pour indexer les poids/items mais celui de Rust est 10 * plus lent !!! mais économise pas mal de mémoire sur les itérations proche du résultat
fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if max_weight == 0 || items.len() == 0 {
        return 0;
    }
    // Ajouter un item en plus pour la ligne de 0 objet, 0 poids
    let mut memoisation = vec![vec![-1; max_weight as usize + 1 ]; items.len() + 1];
    compute(&items, &mut memoisation, max_weight as usize, items.len());

    *memoisation.iter().map(|v| v.iter().max().unwrap()).max().unwrap() as u32
}

#[test]
fn test_no_items() {
    let max_weight = 100;
    let items = [];
    let output = maximum_value(max_weight, &items);
    let expected = 0;
    assert_eq!(output, expected);
}
#[test]
fn test_one_item_too_heavy() {
    let max_weight = 10;
    let items = [Item {
        weight: 100,
        value: 1,
    }];
    let output = maximum_value(max_weight, &items);
    let expected = 0;
    assert_eq!(output, expected);
}
#[test]
fn test_five_items_cannot_be_greedy_by_weight() {
    let max_weight = 10;
    let items = [
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 10,
            value: 21,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 21;
    assert_eq!(output, expected);
}
#[test]
fn test_five_items_cannot_be_greedy_by_value() {
    let max_weight = 10;
    let items = [
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 10,
            value: 50,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 80;
    assert_eq!(output, expected);
}
#[test]
fn test_example_knapsack() {
    let max_weight = 10;
    let items = [
        Item {
            weight: 5,
            value: 10,
        },
        Item {
            weight: 4,
            value: 40,
        },
        Item {
            weight: 6,
            value: 30,
        },
        Item {
            weight: 4,
            value: 50,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 90;
    assert_eq!(output, expected);
}
#[test]
fn test_8_items() {
    let max_weight = 104;
    let items = [
        Item {
            weight: 25,
            value: 350,
        },
        Item {
            weight: 35,
            value: 400,
        },
        Item {
            weight: 45,
            value: 450,
        },
        Item {
            weight: 5,
            value: 20,
        },
        Item {
            weight: 25,
            value: 70,
        },
        Item {
            weight: 3,
            value: 8,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 900;
    assert_eq!(output, expected);
}
#[test]
fn test_15_items() {
    let max_weight = 750;
    let items = [
        Item {
            weight: 70,
            value: 135,
        },
        Item {
            weight: 73,
            value: 139,
        },
        Item {
            weight: 77,
            value: 149,
        },
        Item {
            weight: 80,
            value: 150,
        },
        Item {
            weight: 82,
            value: 156,
        },
        Item {
            weight: 87,
            value: 163,
        },
        Item {
            weight: 90,
            value: 173,
        },
        Item {
            weight: 94,
            value: 184,
        },
        Item {
            weight: 98,
            value: 192,
        },
        Item {
            weight: 106,
            value: 201,
        },
        Item {
            weight: 110,
            value: 210,
        },
        Item {
            weight: 113,
            value: 214,
        },
        Item {
            weight: 115,
            value: 221,
        },
        Item {
            weight: 118,
            value: 229,
        },
        Item {
            weight: 120,
            value: 240,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 1458;
    assert_eq!(output, expected);
}