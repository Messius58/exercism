use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

pub mod solver;

#[derive(PartialEq)]
enum SommeLng {
    Superieure,
    Egale,
    Inferieure
}

#[derive(Clone, Copy, PartialEq)]
enum Action {
    Suivant(u8),
    Precedent(u8),
    Calculer,
    Rejeter, // Si et Seulement Si toutes les solutions possibles ont été évaluées
    Accepter,
    Poursuivre(u8) // Continuer à tester les valeurs restantes dans chaque étape
}

enum ValeurError {
    Attribuer(char),
    HorsLimite
}

struct Expression {
    termes: Vec<(char, u8)>, // possible ex : xa + b + yc = d (ex : "NO + NO + TOO == LATE")
    somme: char,
    suivant: Option<Rc<RefCell<Expression>>>,
    precedent: Option<Rc<RefCell<Expression>>>
}

struct Liste {
    tete: Rc<RefCell<Expression>>,
    queue: Rc<RefCell<Expression>>
}

#[derive(Clone)]
struct Donnees {
    retenu_a_fournir: u8,
    retenu_a_integrer: u8,
    doit_fournir: bool
}

#[derive(Clone)]
struct Contexte {
    hash: HashMap<char, u8>,
    valeurs_utilises: [(bool, char); 10],
    lettres_reserves: HashSet<char>,
    action: Action
}

struct Consigne {
    valeur: u8,
    force: bool
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            termes: Vec::new(),
            somme: ' ',
            suivant: None,
            precedent: None
        }
    }

    fn create_noeud(val: char, pos: usize, termes: &Vec<Vec<char>>) -> Rc<RefCell<Self>> {
        let mut expr = Self::new();
        let lettres = termes.iter().filter_map(|t| t.get(pos)).collect::<Vec<&char>>();
        expr.somme = val;
        if !lettres.is_empty() {
            let mut let_clo = lettres.clone();
            let_clo.sort();
            let_clo.dedup();
            expr.termes = let_clo.iter()
                .map(|l| (**l, lettres.iter().filter(|f| **f == *l).count() as u8))
                .collect::<Vec<(char, u8)>>();
        }

        Rc::new(RefCell::new(expr))
    }

    /// Calculer une opération sur une expression (unité) et vérifier son égalité.
    /// Retourne la valeur du calcul et un booléen si celui-ci est égale.
    fn calculer(&self, hash: &HashMap<char, u8>, val_prec: u8) -> (u8, bool) {
        let mut sum_t: u8 = val_prec / 10;
        for terme in self.termes.iter() {
            sum_t += hash[&terme.0] * terme.1;
        }
        if sum_t >= 10 {
            (sum_t, sum_t % 10 == hash[&self.somme])
        } else {
            (sum_t, sum_t == hash[&self.somme])
        }
    }
}

impl Liste {
    pub fn new(prem_noeud: &Rc<RefCell<Expression>>) -> Self {
        Liste {
            tete: Rc::clone(prem_noeud),
            queue: Rc::clone(prem_noeud)
        }
    }
}

impl Donnees {
    pub fn new() -> Self {
        Donnees {
            retenu_a_fournir: 0,
            doit_fournir: false,
            retenu_a_integrer: 0
        }
    }
}

impl Contexte {
    pub fn new(input: &str, action: Action) -> Self {
        let mut hash_alpha = input.chars().filter(|c| c.is_ascii_alphabetic()).collect::<Vec<char>>();
        hash_alpha.sort();
        hash_alpha.dedup();
        Contexte {
            valeurs_utilises: [(false, ' '); 10],
            lettres_reserves: HashSet::new(),
            action,
            // tableau des valeurs à tester
            hash: HashMap::from_iter(hash_alpha.into_iter().map(|c| (c, u8::MAX)))
        }
    }

    fn set_variable(&mut self, lettre: char, consigne: &Consigne) -> Result<(), ValeurError> {
        if consigne.valeur >= self.valeurs_utilises.len() as u8 {
            return Err(ValeurError::HorsLimite);
        }
        let (b, c) = self.valeurs_utilises[consigne.valeur as usize];
        if c != lettre {
            match (consigne.force, b) {
                (false, true) => return Err(ValeurError::Attribuer(c)), // si valeur déjà prise, renvoyer une erreur
                (true, true) => _ = self.hash.insert(c, u8::MAX),
                (_, _) => (),
            }
            if self.hash[&lettre] != u8::MAX {
                self.valeurs_utilises[self.hash[&lettre] as usize] = (false, ' '); // libérer l'ancienne valeur
            }
            // réserver et affecter la nouvelle valeur
            self.valeurs_utilises[consigne.valeur as usize] = (true, lettre);
            self.hash.insert(lettre, consigne.valeur);
        }

        Ok(())
    }

    fn valeurs_disponibles(&self, debut: u8) -> Vec<u8> {
        self.valeurs_utilises.iter().enumerate()
        .filter(|p| !p.1.0 && p.0 >= debut as usize)
        .map(|(p, _)| p as u8).collect()
    }

    fn reset_operation(&mut self, expression: &Expression) {
        let mut lambda = |&cara| {
            if self.hash[&cara] != u8::MAX && !self.lettres_reserves.contains(&cara) {
                self.valeurs_utilises[self.hash[&cara] as usize] = (false, ' '); // libérer l'ancienne valeur
                self.hash.insert(cara, u8::MAX);
            }
        };
        for term in expression.termes.iter() {
            lambda(&term.0);
        }
        lambda(&expression.somme);
    }

    fn reserver_termes(&mut self, expression: &Expression) {
        for term in expression.termes.iter() {
            self.lettres_reserves.insert(term.0);
        }
        self.lettres_reserves.insert(expression.somme);
    }
}

impl Consigne {
    pub fn new() -> Self {
        Consigne {
            valeur: 0,
            force: false
        }
    }

    pub fn with_value<'a>(&'a mut self, valeur: u8) -> &'a Self {
        self.valeur = valeur;
        self
    }
}