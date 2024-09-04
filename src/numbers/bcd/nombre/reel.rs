
use std::{cmp::Ordering, fmt::Display, ops::{Add, Div, Mul, Sub}};
use crate::numbers::bcd::{BcdError, PackedBcd};

use super::{InfosOps, Nombre, NombreOps, NombreType};

mod test;

// Pour représenter un grand nombre ayant une partie entière et décimale.
// Ex : 1234567890000000000000.00000000987654321
// qui sera représenté par entier (+/-) 123456789E+13 et décimal de même signe (+/-) 987654321E-17
//
// REMARQUE : Le signe doit être reporté sur les 2 mantisses (entier et décimal)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Reel {
    entier: Nombre,
    decimal: Nombre,
}

// Autre avancée : Non nécessaire pour l'exercice
// Traiter la conversion des notations scientifiques (Ex. 123.E-12) et calcul exposant sur non base
// Si possible tenir compte de la représentation française
// Modifier le type de retour est traité les cas d'erreurs
impl Reel {
    fn new() -> Self {
        Reel {
            entier: Nombre::new(NombreType::Entier),
            decimal: Nombre::new(NombreType::Decimal),
        }
    }

    // Met le signe négatif sur la partie entière
    fn signed(&mut self) -> &Self {
        self.entier.mantisse.signed();
        self.decimal.mantisse.signed();

        self
    }

    // Met le signe positif sur la partie entière
    fn unsigned(&mut self) -> &Self {
        self.entier.mantisse.unsigned();
        self.decimal.mantisse.unsigned();
        self
    }

    fn try_from(input: &str) -> Option<Reel> {
        if input.is_empty() {
            return None;
        }

        Some(input.into())
    }

    fn selection_termes<'a>(&'a self, rhs: &'a Self) -> Result<(InfosOps<'a>, InfosOps<'a>), BcdError> {
        if let Ok(ent) = self.entier.selection_termes(&rhs.entier) {
            let has_chiffre = self.entier.mantisse.any();
            if ent.order.is_eq() {
                if let Ok(dec) = self.decimal.selection_termes(&rhs.decimal) {
                    match dec.order {
                        Ordering::Equal => (),
                        _ => {
                            // En cas d'égalité ent == self, valeur de retour par défaut
                            if !dec.term_lng.is_lhs {
                                // On inverse le choix de ent pour suivre celui de dec
                                return Ok((InfosOps::new(ent.term_crt, ent.term_lng, dec.order, has_chiffre), dec))
                            }
                        },
                    }
                    return Ok((ent, dec));
                };
            } else {
                let mut exp_self = 0;
                let mut exp_rhs = 0;
                _ = Nombre::bcd_to_bin(&self.decimal.exposant, &mut exp_self);
                _ = Nombre::bcd_to_bin(&rhs.decimal.exposant, &mut exp_rhs);
                let dec = if ent.term_lng.is_lhs {
                    InfosOps::new(NombreOps::new(&self.decimal, exp_self, true), NombreOps::new(&rhs.decimal, exp_rhs, false), ent.order, has_chiffre)
                } else {
                    InfosOps::new(NombreOps::new(&rhs.decimal, exp_rhs, false), NombreOps::new(&self.decimal, exp_self, true), ent.order, has_chiffre)
                };
                return Ok((ent, dec));
            }
        };

        Err(BcdError::AlignementInvalide)
    }
}

impl From<&str> for Reel {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            return Reel::new();
        }

        let mut nbre_reel = Reel::new();
        let splits: Vec<&str> = value.split(".").collect();
        if splits.len() == 2 {
            if let Err(err) = nbre_reel.entier.str_to_bcd(splits[0].trim_start_matches(&[' ','0'])) {
                panic!("Erreur de conversion de l'entier : {:?}", err);
            };
            if let Err(err) = nbre_reel.decimal.str_to_bcd(splits[1].trim_end_matches(&[' ','0'])) {
                panic!("Erreur de conversion du décimal : {:?}", err);
            }
        } else if let Err(err) = nbre_reel.entier.str_to_bcd(value.trim_start_matches(&[' ','0'])) {
            panic!("Erreur de conversion de l'entier : {:?}", err);
        };

        if nbre_reel.entier.mantisse.is_signed() {
            nbre_reel.decimal.mantisse.signed();
        } else {
            nbre_reel.decimal.mantisse.unsigned();
        }

        nbre_reel
    }
}

impl Display for Reel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.entier, self.decimal)
    }
}

impl Add for Reel {
    type Output = Self;
/*
    NE PAS UTILISER les opérateurs de nombre, car reel étant découpé en 2 parties ent + dec
*/
    // Attention aux retenues et décalages de valeurs
    fn add(self, rhs: Self) -> Self::Output {
        let mut reel_ajout = Reel::new();

        let (oper_ent, oper_dec) = self.selection_termes(&rhs).unwrap();
        let (sign_lng, sign_crt) = (oper_ent.term_lng.is_signed(), oper_ent.term_crt.is_signed());
        let (res_ent, res_dec) = match (sign_lng, sign_crt) {
            (true, false) => (Nombre::soustraction_positive(&oper_ent), Nombre::soustraction_positive(&oper_dec)),
            (false, true) => (Nombre::soustraction_positive(&oper_ent), Nombre::soustraction_positive(&oper_dec)),
            _ => (Nombre::addition_positive(&oper_ent), Nombre::addition_positive(&oper_dec)),
        };

        if let Err(BcdError::Depassement(nbre, _retenu)) = res_dec {
            if let Ok(ent) = res_ent {
                let retenu = match (sign_lng, sign_crt) {
                    (true, false) => Nombre::from("1"),
                    (false, true) => Nombre::from("-1"),
                    _ => Nombre::from("1"),
                };
                reel_ajout.entier = (ent + retenu).unwrap();
            }
            reel_ajout.decimal = nbre;
        } else {
            reel_ajout.entier = res_ent.unwrap();
            reel_ajout.decimal = res_dec.unwrap();
        }
        // Reporter le signe
        if sign_lng { reel_ajout.signed(); }
        else { reel_ajout.unsigned(); }

        reel_ajout
    }
}

impl Sub for Reel {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut reel_ajout = Reel::new();

        let (oper_ent, oper_dec) = self.selection_termes(&rhs).unwrap();
        let (sign_lng, sign_crt) = (oper_ent.term_lng.is_signed(), oper_ent.term_crt.is_signed());
        let (res_ent, res_dec) = match (sign_lng, sign_crt) {
            (true, false) => (Nombre::addition_positive(&oper_ent), Nombre::addition_positive(&oper_dec)),
            (false, true) => (Nombre::addition_positive(&oper_ent), Nombre::addition_positive(&oper_dec)),
            _ => (Nombre::soustraction_positive(&oper_ent), Nombre::soustraction_positive(&oper_dec)),
        };

        if let Err(BcdError::Depassement(nbre, _retenu)) = res_dec {
            if let Ok(ent) = res_ent {
                let retenu = match (sign_lng, sign_crt) {
                    (false, true) => Nombre::from("1"),
                    (true, false) => Nombre::from("-1"),
                    _ => Nombre::from("-1")
                };
                reel_ajout.entier = (ent + retenu).unwrap();
            }
            reel_ajout.decimal = nbre;
        } else {
            reel_ajout.entier = res_ent.unwrap();
            reel_ajout.decimal = res_dec.unwrap();
        }
        // Reporter le signe
        match (self.entier.mantisse.is_signed(), rhs.entier.mantisse.is_signed()) {
            (false, true) => {reel_ajout.unsigned();},
            (false, false) => if !oper_ent.term_lng.is_lhs { reel_ajout.signed(); }
                            else { reel_ajout.unsigned(); },
            (true, false) => {reel_ajout.signed();},
            (true, true) => if !oper_ent.term_lng.is_lhs { reel_ajout.unsigned(); }
                            else { reel_ajout.signed(); },
        }

        reel_ajout
    }
}
/*
    CAS :
    Entier * entier => entier (utiliser nombre)
    Décimal * décimal => décimal (utiliser nombre)
    Entier * décimal => décimal : utilise nombre en convertissant en entier puis reconvertir le résultat en 2 parties
    Décimal * entier => idem

    Puis additionner les 4 sous-résultats pour obtenir le résultat final.
*/
impl Mul for Reel {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut rhs_dec_to_ent = Nombre::new(NombreType::Entier);
        let mut self_dec_to_ent = Nombre::new(NombreType::Entier);
        rhs_dec_to_ent.mantisse = rhs.decimal.mantisse.clone();
        self_dec_to_ent.mantisse = self.decimal.mantisse.clone();
        let mut exp_lhs = 0;
        let mut exp_rhs = 0;

        let mut reel_1 = Reel::new();
        _ = Nombre::bcd_to_bin(&self.entier.exposant, &mut exp_lhs);
        _ = Nombre::bcd_to_bin(&rhs.entier.exposant, &mut exp_rhs);
        reel_1.entier = Nombre::multiplication_entier(&InfosOps::new(NombreOps::new(&self.entier, exp_lhs, true), NombreOps::new(&rhs.entier, exp_rhs, false), Ordering::Greater, true)).unwrap();

        let mut exp_dec_lhs = 0;
        let mut exp_dec_rhs = 0;
        _ = Nombre::bcd_to_bin(&self.decimal.exposant, &mut exp_dec_lhs);
        _ = Nombre::bcd_to_bin(&rhs.decimal.exposant, &mut exp_dec_rhs);
        reel_1.decimal = Nombre::multiplication_entier(&InfosOps::new(NombreOps::new(&self.decimal, exp_dec_lhs, true), NombreOps::new(&rhs.decimal, exp_dec_rhs, false), Ordering::Greater, true)).unwrap();

        // Faire le décalage et séparation entier - décimal en recréant les reels avant additionner
        let mut reel_2 = Reel::new();
        let mut res_ent_dec = (self.entier * rhs_dec_to_ent).unwrap();
        if exp_lhs >= exp_dec_rhs {
            reel_2.entier = res_ent_dec;
            _ = reel_2.entier.int_to_bcd(exp_lhs, super::NombrePart::Exposant);
        } else if exp_dec_rhs - exp_lhs >= res_ent_dec.mantisse.total {
            reel_2.decimal.mantisse = res_ent_dec.mantisse;
            _ = reel_2.decimal.int_to_bcd(exp_dec_rhs, super::NombrePart::Exposant);
        } else {
            let mut tmp = PackedBcd::new();
            for _ in 0..res_ent_dec.mantisse.total - (exp_dec_rhs - exp_lhs) {
                _ = tmp.append(res_ent_dec.mantisse.pop().unwrap());
             }
             while let Some(n) = tmp.pop() {
                _ = reel_2.entier.mantisse.append(n);
             }
             reel_2.decimal.mantisse = res_ent_dec.mantisse;
             _ = reel_2.decimal.int_to_bcd(exp_dec_rhs - exp_lhs, super::NombrePart::Exposant);
        }

        let mut reel_3 = Reel::new();
        let mut res_dec_ent = (self_dec_to_ent * rhs.entier).unwrap();
        if exp_rhs >= exp_dec_lhs {
            reel_3.entier = res_dec_ent;
            _ = reel_3.entier.int_to_bcd(exp_rhs, super::NombrePart::Exposant);
        } else if exp_dec_lhs - exp_rhs >= res_dec_ent.mantisse.total {
            reel_3.decimal.mantisse = res_dec_ent.mantisse;
            _ = reel_3.decimal.int_to_bcd(exp_dec_lhs, super::NombrePart::Exposant);
        } else {
            for _ in 0..res_dec_ent.mantisse.total - (exp_dec_lhs - exp_rhs) {
                _ = reel_3.entier.mantisse.append(res_dec_ent.mantisse.pop().unwrap());
             }
             _ = reel_3.entier.int_to_bcd(res_dec_ent.mantisse.total - (exp_dec_lhs - exp_rhs), super::NombrePart::Exposant);
             _ = reel_3.decimal.int_to_bcd(exp_dec_lhs - exp_rhs, super::NombrePart::Exposant);
        }

        let test = reel_1 + reel_2;
        let test2 = test + reel_3;

        test2
    }
}

impl Div for Reel {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut reel_ajout = Reel::new();

        reel_ajout
    }
}
