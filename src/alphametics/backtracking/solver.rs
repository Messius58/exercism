use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::alphametics::backtracking::{Action, Contexte, Expression, Liste, SommeLng};

use super::{Consigne, Donnees};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let items = input.split("==").map(|sl| sl.trim()).collect::<Vec<&str>>();
    if items.len() != 2 {
        return None; // signe égale non présent
    }

    let termes = items[0].split("+")
                         .map(|sl| sl.trim())
                         .map(|s2| s2.chars().rev().collect())
                         .collect::<Vec<Vec<char>>>();
                        let ln = termes.iter().max().unwrap().len() as i32;
                        let ln2 = items[1].len() as i32;
    let som_lng = match ln - ln2 {
        e if e < 0 => SommeLng::Superieure,
        0 => SommeLng::Egale,
        _ => SommeLng::Inferieure,
    };
    if som_lng == SommeLng::Inferieure {
        return None; // terme plus grand que la somme (ex. 10 + 1 = 1)
    }

    // créer les expressions mathématiques
    let mut iter_som = items[1].chars().rev().enumerate();
    let (pos, val) = iter_som.next().unwrap();
    let mut prec_expr = Expression::create_noeud(val, pos, &termes);
    let mut lst = Liste::new(&prec_expr);
    while let Some((pos, val)) = iter_som.next() {
        let cou_expr = Expression::create_noeud(val, pos, &termes);
        prec_expr.as_ref().borrow_mut().suivant = Some(Rc::clone(&cou_expr));
        cou_expr.as_ref().borrow_mut().precedent = Some(Rc::clone(&prec_expr));
        prec_expr = cou_expr;
    }
    lst.queue = Rc::clone(&prec_expr);

    let (action, prm_nd) = match som_lng {
        SommeLng::Superieure => (Action::Precedent(0), &lst.queue),
        SommeLng::Egale => (Action::Suivant(0), &lst.tete),
        _ => return None,
    };
    let mut ctx = Contexte::new(input, action);
    // commencer avec backtracking qui continuera en récursive avec next_operation
    if backtracking(&lst, prm_nd, &mut ctx, &mut Donnees::new()) == Action::Accepter {
        Some(ctx.hash)
    } else {
        None
    }
}

fn backtracking(liste: &Liste, candidate: &Rc<RefCell<Expression>>, contexte: &mut Contexte, donnees: &mut Donnees) -> Action {
    let candidat = candidate.as_ref().borrow();
    if candidat.termes.len() == 0 {
        let debut = if candidat.suivant.is_some() { 0 } else { 1 };
        let val_dispo = contexte.valeurs_disponibles(debut);
        let mut valcons = Consigne::new();
        for choix in val_dispo {
            // demande de retenue obligatoire car pas de terme pour définir somme
            donnees.retenu_a_fournir = choix;
            //donnees.doit_fournir = true;
            _ = contexte.set_variable(candidat.somme, valcons.with_value(choix));
            match next_operation(liste, candidate, contexte, donnees) {
                Action::Suivant(retenu) | Action::Precedent(retenu) | Action::Poursuivre(retenu) => continue,
                action => return action,
            }
        }
    } else {
        // Evaluer le nbre de terme restant à découvrir et calculer l'opération avec les termes déjà définis
        let (terme_restant, mut terme_somme): (u8, u8) = candidat.termes.as_slice().iter()
        .filter(|term| contexte.hash[&term.0] != u8::MAX)
        .fold((candidat.termes.len() as u8, 0u8), |(terme_restant, terme_somme), term| (terme_restant - 1, terme_somme + contexte.hash[&term.0] * term.1 + donnees.retenu_a_integrer));
        match (terme_restant, contexte.hash[&candidat.somme].ne(&u8::MAX)) {
            (0, s) => {
                // valeur déjà réservée par une autre lettre, pas possible de la prendre sans forcer
                if !s && contexte.set_variable(candidat.somme, Consigne::new().with_value(terme_somme)).is_err() {
                    return Action::Poursuivre(terme_restant / 10);
                }
                return next_operation(liste, candidate, contexte, donnees);
            }
            (1, true) => {  // Regrouper les cas 1, true et 1, false pour simplifier le code
                // La somme est déjà définie, reste terme qui n'a pas le choix de prendre la valeur déduite si une seule occurrence sinon boucle
                let terme_restant = candidat.termes.iter().filter(|f| contexte.hash[&f.0] == u8::MAX).collect::<Vec<&(char, u8)>>()[0];
                if terme_restant.1 > 1 {
                    let val_dispo = contexte.valeurs_disponibles(1);
                    let mut valconsterm = Consigne::new();
                    for ch_term in 0..val_dispo.len() {
                        let tmp_sum = terme_somme + val_dispo[ch_term] * terme_restant.1 + donnees.retenu_a_integrer;
                        if donnees.doit_fournir {
                            if tmp_sum < 10 || tmp_sum % 10 != contexte.hash[&candidat.somme] {
                                continue;
                            }
                        }
                        else if tmp_sum != contexte.hash[&candidat.somme] {
                            continue;
                        }
                        // Ne devrait normalement pas arriver car demande plus tôt des valeurs restantes disponible...
                        if contexte.set_variable(terme_restant.0, valconsterm.with_value(val_dispo[ch_term])).is_err() {
                            continue;
                        }
                        match next_operation(liste, candidate, contexte, donnees) {
                            Action::Poursuivre(retenu) => {
                                donnees.retenu_a_integrer = retenu;
                                continue
                            },
                            Action::Accepter => return Action::Accepter,
                            a => return a
                        }
                    }
                } else {
                    terme_somme -= contexte.hash[&candidat.somme];
                    if contexte.set_variable(terme_restant.0, Consigne::new().with_value(terme_somme.div_euclid(terme_restant.1))).is_err() {
                        return Action::Poursuivre(terme_somme / 10);
                    }
                    match next_operation(liste, candidate, contexte, donnees) {
                        Action::Accepter => return Action::Accepter,
                        a => return a
                    }
                }
            },
            (1, false) => { // Somme + terme non définie
                let terme_restant = candidat.termes.iter().filter(|f| contexte.hash[&f.0] == u8::MAX).collect::<Vec<&(char, u8)>>()[0];
                let val_dispo = contexte.valeurs_disponibles(0);
                let mut valcons_som = Consigne::new();
                let mut valconsterm = Consigne::new();
                if candidat.somme != terme_restant.0 && terme_somme == 0 {
                    donnees.retenu_a_integrer += 1;
                }
                for ch_som in 0..val_dispo.len() {
                    if candidat.somme != terme_restant.0 {
                        for ch_term in 0..val_dispo.len() {
                            if ch_term == ch_som { continue; }
                            let tmp_sum = terme_somme + val_dispo[ch_term] * terme_restant.1 + donnees.retenu_a_integrer;
                            if donnees.doit_fournir {
                                if tmp_sum < 10 || tmp_sum % 10 != val_dispo[ch_som] {
                                    continue;
                                }
                            }
                            else if tmp_sum != val_dispo[ch_som] {
                                continue;
                            }
                            // Ne devrait normalement pas arriver car demande plus tôt des valeurs restantes disponible...
                            if contexte.set_variable(candidat.somme, valcons_som.with_value(val_dispo[ch_som])).is_err() ||
                               contexte.set_variable(terme_restant.0, valconsterm.with_value(val_dispo[ch_term])).is_err() {
                                continue;
                            }
                            match next_operation(liste, candidate, contexte, donnees) {
                                Action::Poursuivre(retenu) => {
                                    donnees.retenu_a_integrer = retenu;
                                    continue
                                },
                                Action::Accepter => return Action::Accepter,
                                a => return a
                            }
                        }
                    } else {
                         if (terme_somme + val_dispo[ch_som] * terme_restant.1 + donnees.retenu_a_integrer) % 10 != val_dispo[ch_som] ||
                               contexte.set_variable(candidat.somme, valcons_som.with_value(val_dispo[ch_som])).is_err() {
                            continue;
                        }
                        match next_operation(liste, candidate, contexte, donnees) {
                            Action::Poursuivre(retenu) => {
                                donnees.retenu_a_integrer = retenu;
                                continue
                            },
                            Action::Accepter => return Action::Accepter,
                            a => return a
                        }
                    }
                }
                // on retente en demandant une retenue
                if donnees.retenu_a_integrer == 0 {
                    donnees.retenu_a_integrer += 1;
                    contexte.reset_operation(&candidat);
                    return backtracking(liste, candidate, contexte, donnees);
                }
            },
            _ => return Action::Poursuivre(0)
            // TODO : Reste le cas de plusieurs termes sans valeur dans une même opération
        }
    }

    Action::Poursuivre(0)
}

fn next_operation(liste: &Liste, candidate: &Rc<RefCell<Expression>>, contexte: &mut Contexte, donnees: &mut Donnees) -> Action {
    let candidat = candidate.as_ref().borrow();
    if contexte.hash.values().all(|p| *p != u8::MAX) {
        return evaluer(&contexte.hash, &liste.tete, 0);
    } else {
        let cand_backtrack = match contexte.action {
            Action::Precedent(_) => &candidat.precedent,
            _ => &candidat.suivant
        };
        if cand_backtrack.is_some() { // revoir les retenues
            // donnees.retenu_a_fournir = false;
            // donnees.retenu_a_integrer = 0;
        }
        // clonage donnees et contexte ici pour que backtracking puisse avoir la possibilité de faire de la récursivité
        // sur un même jeu sinon peut reset son jeu (non implémenter)
        // clonage du contexte ici permet aussi gérer la remontée de la solution finale, d'exclure les termes déjà gérés
        let clone_ctx = &mut contexte.clone();
        clone_ctx.reserver_termes(&candidat);
        match backtracking(liste, cand_backtrack.as_ref().unwrap(), clone_ctx, &mut donnees.clone()) {
            Action::Accepter => {
                contexte.hash = clone_ctx.hash.clone();
                return Action::Accepter;
            },
            a => return a
        }
    }
}

/// évaluer l'ensemble des opérations, une fois que toutes les variables ont été définies avec une valeur.
fn evaluer(hash: &HashMap<char, u8>, expr: &Rc<RefCell<Expression>>, prec_val: u8) -> Action {
    let resultat = expr.as_ref().borrow().calculer(hash, prec_val);
    if resultat.1 {
        if expr.as_ref().borrow().suivant.is_some(){
            evaluer(hash, expr.as_ref().borrow().suivant.as_ref().unwrap(), resultat.0);
        }
    }

    if resultat.1 { Action::Accepter }
    else { Action::Poursuivre(0) }
}
