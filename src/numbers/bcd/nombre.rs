// https://fr.wikipedia.org/wiki/Décimal_codé_binaire
// https://fr.wikipedia.org/wiki/Double_dabble

use std::{cmp::Ordering, fmt::Display, ops::{Add, Div, Mul, Sub}, ptr, str::Chars};

use super::{BcdError, BcdIterator, PackedBcd, NIBBLE_RIGHT_BIT_MASK};

pub mod reel;

#[derive(PartialEq)]
enum NombrePart {
    Mantisse,
    Exposant
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum NombreType {
    Entier,
    Decimal
}

enum OpsTerm {
    Long,
    Court
}

struct NombreOps<'a> {
    operande: &'a Nombre,
    deb_position: u128,
    is_lhs: bool
}

impl<'a> NombreOps<'a> {
    fn new(operande: &'a Nombre, deb_position: u128, is_lhs: bool) -> Self {
        NombreOps { operande, deb_position, is_lhs }
    }

    fn len(&self) -> u128 {
        match self.operande.type_nbr {
            NombreType::Entier => self.deb_position + self.operande.mantisse.total,
            NombreType::Decimal => self.deb_position,
        }
    }

    fn iter(&self) -> BcdIterator {
        self.operande.mantisse.iter()
    }

    fn is_signed(&self) -> bool {
        self.operande.mantisse.is_signed()
    }
}

struct InfosOps<'a> {
    term_lng: NombreOps<'a>,
    term_crt: NombreOps<'a>,
    order: Ordering,
    has_entier: bool
}

impl<'a> InfosOps<'a> {

    fn new(term_lng: NombreOps<'a>, term_crt: NombreOps<'a>, order: Ordering, has_entier: bool) -> Self {
        InfosOps { term_lng, term_crt, order, has_entier }
    }

    fn ordering(&self) -> Ordering {
        self.order
    }

    fn total_chiffres(&self, term: OpsTerm) -> u128 {
        match term {
            OpsTerm::Long => self.term_lng.len(),
            OpsTerm::Court => self.term_crt.len(),
        }
    }

    // TODO : A REVOIR
    fn get_exposant(&self, term: OpsTerm) -> u128 {
        match term {
            OpsTerm::Long =>
                match self.term_lng.operande.type_nbr {
                    NombreType::Entier => if self.term_lng.deb_position < self.term_crt.deb_position { self.term_lng.deb_position }
                                          else { self.term_crt.deb_position },
                    NombreType::Decimal => if self.term_lng.deb_position > self.term_crt.deb_position { self.term_lng.deb_position }
                                           else { self.term_crt.deb_position },
                },
            OpsTerm::Court =>
                match self.term_lng.operande.type_nbr {
                    NombreType::Entier => if self.term_lng.deb_position < self.term_crt.deb_position { self.term_lng.deb_position }
                                          else { self.term_crt.deb_position },
                    NombreType::Decimal => if self.term_lng.deb_position > self.term_crt.deb_position { self.term_lng.deb_position }
                                           else { self.term_crt.deb_position },
                },
        }
    }

    fn get_type(&self) -> NombreType {
        self.term_lng.operande.type_nbr
    }
}

// Traquer les zéros qui pourraient être ajouté dans l'exposant suite au calcul
trait Tracking {
    fn zero_func (n: u8, type_nbre: NombreType, mant: &mut PackedBcd, exp: &mut u128, zéros: &mut i32) {
        if n == 0 && *zéros != -1 { *zéros += 1; }
        else {
            // Intégrer les zéros retenus dans l'exposant
            if *zéros != -1 {
                if type_nbre == NombreType::Entier { // dans ce cas on stop le suivi
                    *exp += *zéros as u128;
                    *zéros = -1;
                }
                else { // Dans le cas du décimal, réintégrer les zéros et continuer le suivi
                    if mant.any() {
                        for _ in 0..*zéros { _ = mant.append(0); }
                    } else {
                        *exp -= *zéros as u128;
                    }
                    *zéros = 0;
                }
            }
            _ = mant.append(n);
        }
    }
}

// Pour représenter un grand entier sans partie décimale ou une grande décimale sans partie entière.
// Pour avoir les deux, utiliser la structure Nombre.
// Ex : (+/-)123456789E⁴⁶ ou (+/-)987654321E⁻⁴⁰
#[derive(Debug, Eq, Ord, Clone)]
pub(crate)
struct Nombre {
    mantisse: PackedBcd,
    exposant: PackedBcd,
    base_exp: u8, // Pour exprimer les grands nombres sur puissance de 10 uniquement Ex : ⋅10E³°
    type_nbr: NombreType,
    period: bool
}

impl Tracking for Nombre {}

impl Nombre {
    fn new(type_nbr: NombreType) -> Self {
        Nombre {
            mantisse: PackedBcd::new(),
            exposant: PackedBcd::new(),
            base_exp: 0,
            type_nbr,
            period: false
        }
    }

    fn with_capacity(type_nbr: NombreType, mant_cap: usize, exp_cap: usize) -> Self {
        Nombre {
            mantisse: PackedBcd::with_capacity(mant_cap),
            exposant: PackedBcd::with_capacity(exp_cap),
            base_exp: 0,
            type_nbr,
            period: false
        }
    }

    #[inline]
    fn meme_type(&self, rhs: Self) -> bool {
        self.type_nbr != rhs.type_nbr
    }

    // Déduire l'exposant de simplification du nombre selon si les
    // zéros signficatifs sont à droite (entier) ou à gauche (décimal).
    //
    // L'itérateur étant partagé, retourne le dernier caractère lu non zéro
    // ainsi que l'exposant qui devra être complété avec le nombre des autres chiffres
    // dans le cadre d'un décimal puis être converti en BCD
    fn extract_exp(&mut self, chars: &mut Chars<'_>) -> (char, u128) {
        let mut exposant: u128 = 0;
        let mut opt;
        let ch;
        loop {
            opt = match self.type_nbr {
                NombreType::Entier => chars.next_back(),
                NombreType::Decimal => chars.next(),
            };
            match opt {
                Some(c) =>
                    if c == '0' {
                        exposant += 1;
                    } else {
                        ch = c; break;
                    },
                None => { // Possible fin de parcours et pas de chiffre autre que zéro (cas 0.0)
                    exposant = 0; ch = '0';
                    break;
                }
            }
        }
        if ch == '+' || ch == '-' {
            exposant = 0; // pas de chiffre significatif autre que zéro (cas -0.0)
        }
        if exposant > 0 {
            self.base_exp = 10; // puisque zéro successif base = ⋅10E
        }

        (ch, exposant)
    }

    /*
    Sélection se fait en considérant les nombres sans leurs signes.
    Important dans les cas d'opérations (+/-) sur 2 entiers ou 2 décimaux
        Sélectionner les termes : CAS DU DECIMAL => mantisse long > mantisse court (12345 > 123) ET/OU exposant court > exposant long
                                  CAS DE ENTIER  => mantisse long > mantisse court (12345 > 123) ET/OU exposant long  > exposant court
        Réutilisation d'une partie de l'implémentation du trait PartialOrd (Cf. cmp) mais sans la vérification du signe
    */
    fn selection_termes<'a>(&'a self, rhs: &'a Self) -> Result<InfosOps<'a>, BcdError> {
        let mut iter_self = self.mantisse.iter();
        let mut iter_other = rhs.mantisse.iter();

        let mut exp_self = 0;
        let mut exp_rhs = 0;
        _ = Nombre::bcd_to_bin(&self.exposant, &mut exp_self);
        _ = Nombre::bcd_to_bin(&rhs.exposant, &mut exp_rhs);

        // Traitement du zéro, seul élément dans l'un des 2 termes
        match (self.mantisse.any(), rhs.mantisse.any()) {
            (true, true) => (),
            (true, false) => return Ok(InfosOps::new(NombreOps::new(self, exp_self, true), NombreOps::new(rhs, exp_rhs, false), Ordering::Greater, false)),
            (false, true) => return Ok(InfosOps::new(NombreOps::new(rhs, exp_rhs, false), NombreOps::new(self, exp_self, true), Ordering::Greater, false)),
            (false, false) => return Ok(InfosOps::new(NombreOps::new(self, exp_self, true), NombreOps::new(rhs, exp_rhs, false), Ordering::Equal, false)),
        }
         // Sur (true, true) précédent, on lance une comparaison plus approfondie
        loop {
            let order = match (iter_self.next_back(), iter_other.next_back()) {
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
                (Some(nbre_self), Some(nbre_other)) => {

                    if nbre_self == nbre_other && self.mantisse.total == rhs.mantisse.total && exp_self == exp_rhs {
                        continue;
                    }
                    match self.type_nbr {
                        NombreType::Entier => {
                            match (nbre_self.cmp(&nbre_other), (exp_self + self.mantisse.total).cmp(&(exp_rhs + rhs.mantisse.total))) {
                                (Ordering::Less, Ordering::Less) => Ordering::Less,
                                (Ordering::Less, Ordering::Equal) => Ordering::Less,
                                (Ordering::Less, Ordering::Greater) => Ordering::Greater,
                                (Ordering::Equal, Ordering::Less) => Ordering::Less,
                                (Ordering::Equal, Ordering::Equal) => continue,
                                (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
                                (Ordering::Greater, Ordering::Less) => Ordering::Less,
                                (Ordering::Greater, Ordering::Equal) => Ordering::Greater,
                                (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
                            }
                        },
                        NombreType::Decimal => {
                            match (nbre_self.cmp(&nbre_other), (exp_self - self.mantisse.total).cmp(&(exp_rhs - rhs.mantisse.total))) {
                                (Ordering::Less, Ordering::Less) => Ordering::Greater,
                                (Ordering::Less, Ordering::Equal) => Ordering::Less,
                                (Ordering::Less, Ordering::Greater) => Ordering::Less,
                                (Ordering::Equal, Ordering::Less) => Ordering::Greater,
                                (Ordering::Equal, Ordering::Equal) => continue,
                                (Ordering::Equal, Ordering::Greater) => Ordering::Less,
                                (Ordering::Greater, Ordering::Less) => Ordering::Greater,
                                (Ordering::Greater, Ordering::Equal) => Ordering::Greater,
                                (Ordering::Greater, Ordering::Greater) => Ordering::Less,
                            }
                        },
                    }
                }
            };
            match order {
                Ordering::Less => return Ok(InfosOps::new(NombreOps::new(rhs, exp_rhs, false), NombreOps::new(self, exp_self, true), order, false)),
                _ => return Ok(InfosOps::new(NombreOps::new(self, exp_self, true), NombreOps::new(rhs, exp_rhs, false), order, false)),
            };
        }
    }

    // Converti un nombre entier positif en packed BCD
    // Chaque nibble (quartet) comportant un chiffre, la conversion se fera par modulo de 10
    //
    // On signe par défaut selon le type :
    // - Entier mantisse ou exposant => positif
    // - Décimal mantisse => positif, exposant => négatif
    fn int_to_bcd(&mut self, mut nombre: u128, part: NombrePart) -> Result<(), BcdError> {
        let nbr_part = match part {
            NombrePart::Mantisse => &mut self.mantisse,
            NombrePart::Exposant => &mut self.exposant,
        };
        nbr_part.reset();
        // Traiter l'unité qui recevra le signe
        nbr_part.append((nombre % 10) as u8)?;
        nombre /= 10;
        match self.type_nbr {
            NombreType::Entier => nbr_part.signed(),
            NombreType::Decimal => if part == NombrePart::Exposant { nbr_part.signed(); } else { nbr_part.unsigned(); },
        }
        while nombre > 0 {
            nbr_part.append((nombre % 10) as u8)?;
            nombre /= 10;
        }

        Ok(())
    }

    // Converti un nombre sous forme ASCII en packed BCD
    fn str_to_bcd(&mut self, str: &str) -> Result<(), BcdError> {
        let mut chars = str.chars();
        let mut last_ch;
        let mut exposant;

        self.mantisse.reset();
        // Le chiffre renvoyé par l'exposant est le chiffre d'unité, significatif, signe ou espace blanc.
        (last_ch, exposant) = self.extract_exp(&mut chars);

        // Dans le cadre de l'entier, on traite de suite le signe et l'éventuel unité renvoyé par l'exposant.
        if self.type_nbr == NombreType::Entier {
            if last_ch.is_digit(10) {
                self.mantisse.append(last_ch as u8 & NIBBLE_RIGHT_BIT_MASK)?;
                last_ch = if let Some(signe) = chars.next() {
                    signe
                }
                else { '+' }; // Plus de caractère à traiter on met + par défaut (cas : 0.0)
            } else {
                self.mantisse.append(0)?; // pas de chiffre renvoyé (cas -0.0)
            }
            if !last_ch.is_digit(10) {
                match last_ch {
                    '-' => self.mantisse.signed(),
                    '+' => self.mantisse.unsigned(),
                    _  => return Err(BcdError::CaractereInvalide)
                };
            }
            else { self.mantisse.unsigned(); } // signe + par défaut
        }

        // Entier ou décimal on traite la mantisse de droite à gauche
        while let Some(c) = chars.next_back() {
            if c.is_digit(10) {
                self.mantisse.append(c as u8 & NIBBLE_RIGHT_BIT_MASK)?;
                // Dans le cas du décimal, on continue d'incrémenter l'exposant car représenter sous une forme sans virgule 123E⁻⁴⁰
                if self.type_nbr == NombreType::Decimal {
                    exposant += 1;
                }
            }
        }

        // On traite le chiffre significatif renvoyé par l'exposant ou récupéré dans le traitement du signe
        if last_ch.is_digit(10) {
            if self.type_nbr == NombreType::Decimal
            { exposant += 1; }
            self.mantisse.append(last_ch as u8 & NIBBLE_RIGHT_BIT_MASK)?;
        }

        // La valeur finale de l'exposant peut être converti
        if exposant > 0 {
            self.int_to_bcd(exposant, NombrePart::Exposant)?;
        }

        if !self.mantisse.has_sign() {
            self.mantisse.unsigned();
        }

        Ok(())
    }

    // Converti un nombre BCD en entier et retourne si celui-ci est signé (-)
    // Pour chaque nibble (quartet), récupérer le chiffre en respectant le positionnement (dizaine, unité)
    // multiplier par 100 l'accumulateur pour faire de la place au nibble
    // Signe dans la partie droite de la valeur d'unité
    fn bcd_to_bin(bcd: &PackedBcd, out: &mut u128) -> bool {
        if bcd.is_empty() {
            return false;
        }
        let iter = bcd.iter();
        *out = iter.rfold(0, |acc, f| acc * 10 + f as u128);

        bcd.is_signed()
    }
}

impl Display for Nombre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut mant = 0;
        Nombre::bcd_to_bin(&self.mantisse, &mut mant);
        let mut exp = 0;
        Nombre::bcd_to_bin(&self.exposant, &mut exp);

        let mut out = mant.to_string();
        if self.mantisse.is_signed() { out = "-".to_owned() + &out; }
        if exp > 0 {
            out = if self.exposant.is_signed() {
                format!("{out}E-{exp}")
            } else {
                format!("{out}E{exp}")
            };
        }

        write!(f, "{}", out)
    }
}

/*
Les opérations sont de type naive, une amélioration serait peut-être de passer à ce style de découpage pour de grandes mantisses,
attention toutefois au rôle et à la gestion de l'exposant.
exemple pour multiplication :
https://fr.wikipedia.org/wiki/Algorithme_Toom-Cook pour les très grands nombres

ATTENTION : Cas de entier OP décimal ou inverse (dû au choix technique des structures nombre et reel)

    - Le signe de l'exposant étant forcément négative pour une décimale, on peut ignorer le retour de bcd_to_bin

    Préparer toutes les tranches pour plus de clarté
    - Tranche à droite (excédant du plus long) SI MANTISSE NEGATIVE ALORS FAIRE CMPL_A_10 + RETENU
    - Tranche des chiffres alignés
    - Tranche à gauche (le plus court)

*/
    // 1) Comparer les exposants pour déterminer l'alignement des chiffres
    // 2) Recopier les valeurs non aligner à droite qui ne nécessite pas de calcul (long + 0) ou
    //    calculer 0 - court ou 0 + court selon le cas quand la mantisse de court est plus décalée que long
    //     Ex(décimal) : Long |     0.01245 |  0.01245 |  0.01245    |     0.01245    => représentation 1245E-5
    //                  court |+ (-)0.00100 |+ 0.00001 |+ 0.00000001 |+ (-)0.00000001 => représentation 1E-8
    //                        |=    0.01145 |= 0.01246 |= 0.01245001 |=    0.01244999
    //                           Add -> Sub     Add     Add + compl_0  Add -> sub (compl_10)
    // 3) Additionner ou soustraire les valeurs alignés long / court
    // 4) Reporter la retenue tant qu'elle est là
    // 5) Recopier les valeurs non aligner à gauche avec ou sans retenu (Idem que point 2 sauf que court doit s'arrêter à l'étape 3-4)
impl Nombre {
    fn addition_positive(operandes: &InfosOps) -> Result<Nombre, BcdError> {
        let (mut iter_lng, mut iter_crt) = (operandes.term_lng.iter(), operandes.term_crt.iter());
        let (mut total_lng, mut total_crt) = (operandes.total_chiffres(OpsTerm::Long), operandes.total_chiffres(OpsTerm::Court));
        let mut nbre = Nombre::with_capacity(operandes.term_lng.operande.type_nbr, operandes.term_lng.operande.mantisse.total as usize, operandes.term_lng.operande.exposant.total as usize);

        let mut retenu = 0;
        let mut zéros: i32 = 0;
        let mut exposant = operandes.get_exposant(OpsTerm::Long);

        // Analyse somme avec zéro chiffre significatif
        if total_lng <= 1 && !operandes.term_lng.operande.mantisse.any() {
            return Ok(operandes.term_crt.operande.clone());
        }
        if total_crt <= 1 && !operandes.term_crt.operande.mantisse.any() {
            return Ok(operandes.term_lng.operande.clone());
        }
        // Traitement BCD excédentaire à droite
        // Cas exemple 1
        if (operandes.get_type() == NombreType::Entier && operandes.term_lng.deb_position < operandes.term_crt.deb_position) ||
            (operandes.get_type() == NombreType::Decimal && operandes.term_lng.deb_position > operandes.term_crt.deb_position) {
            let limit;
            match operandes.get_type() {
                NombreType::Entier => {
                    limit = operandes.term_crt.deb_position - operandes.term_lng.deb_position;
                    total_crt -= limit;
                },
                NombreType::Decimal => {
                    limit = operandes.term_lng.deb_position - operandes.term_crt.deb_position;
                    total_lng -= limit;
                },
            };
            // Traitement de l'unité à part pour supprimer le signe
            nbre.mantisse.append(iter_lng.next().unwrap())?;
            for _ in 0..limit - 1 {
                Nombre::zero_func(iter_lng.next().unwrap(), operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
            }
            if operandes.get_type() == NombreType::Entier { zéros = -1; }
        }
        // Cas exemple 3 (add + compl_0)
        else if (operandes.get_type() == NombreType::Entier && operandes.term_lng.deb_position > operandes.term_crt.deb_position) ||
                (operandes.get_type() == NombreType::Decimal && operandes.term_lng.deb_position < operandes.term_crt.deb_position) {
            let mut pos_crt;
            zéros = -1; // On fait une complétion à zéro, il faut donc informer zero_func de ce fait
            match operandes.get_type() {
                NombreType::Entier => {
                    pos_crt = operandes.term_lng.deb_position - operandes.term_crt.deb_position;
                    total_lng -= pos_crt;
                },
                NombreType::Decimal => {
                    pos_crt = operandes.term_crt.deb_position - operandes.term_lng.deb_position;
                    total_crt -= pos_crt;
                },
            };
            // Traitement de l'unité à part pour supprimer le signe
            nbre.mantisse.append(iter_crt.next().unwrap())?;
            let limit = if pos_crt > iter_crt.len() { iter_crt.len() } else { pos_crt };
            if limit > 0 {
                for _ in 0..limit - 1 {
                    Nombre::zero_func(iter_crt.next().unwrap(), operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                }
                pos_crt -= limit;
            }

            while pos_crt > 0 { // Complétion à 0
                Nombre::zero_func(0, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                pos_crt -= 1;
            }
            if operandes.get_type() == NombreType::Entier { zéros = -1; }
        }

        // Traitement des BCD éventuellement alignés
        while let Some(mut n) = iter_crt.next() {
            match iter_lng.next() {
                Some(m) => {
                    n += m + retenu;
                    if n >= 0x0A { retenu = 1; n -= 0x0A; } else { retenu = 0; }
                    Nombre::zero_func(n, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                    total_lng -= 1;
                },
                None => nbre.mantisse.append(n)?,
            }
            total_crt -= 1;
        }

        // Traitement des BCD restants excédentaire à gauche dans le terme long
        while let Some(mut n) = iter_lng.next() {
            n += retenu;
            total_lng -= 1;
            if n >= 0x0A { retenu = 1; n -= 0x0A; } else { retenu = 0; }
            Nombre::zero_func(n, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
        }
        // Traitement de la retenue finale devenue excédentaire
        if retenu == 1 && (total_lng > 0 || operandes.get_type() == NombreType::Entier) {
            Nombre::zero_func(1, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
            retenu = 0;
        }
        // Traitement somme nul : reporter le zéro retenu dans le suivi lors de calcul
        if zéros > 0 && nbre.mantisse.total == 0 {
            nbre.mantisse.append(0)?;
            exposant = if operandes.get_type() == NombreType::Decimal { 1 } else { 0 };
        }
        // Reporter l'exposant et sa base
        if exposant > 0 {
            nbre.int_to_bcd(exposant, NombrePart::Exposant)?;
            nbre.base_exp = 10;
        }
        // Reporter la retenue excédentaire dans le cas du décimal
        if retenu == 1 { Err(BcdError::Depassement(nbre, retenu)) }
        else { Ok(nbre) }
    }
    /*
        CAS EXTREME : 1.00001 + -0.001 => 0.99901
        CAS : 0.0 - 0.001 => ne pas de calcul, changer de signe
        CAS : 1.0 - 0.001 => complément à neuf
        DILEMNE : connaître la partie entière
        Dans ce cas il y a excédent à gauche, non alignement, complément à 10 ET suppression de l'entier !!!
    */
    fn soustraction_positive(operandes: &InfosOps) -> Result<Nombre, BcdError> {
        let (mut iter_lng, mut iter_crt) = (operandes.term_lng.iter(), operandes.term_crt.iter());
        let (mut total_lng, mut total_crt) = (operandes.total_chiffres(OpsTerm::Long), operandes.total_chiffres(OpsTerm::Court));
        let mut nbre = Nombre::with_capacity(operandes.get_type(), operandes.term_lng.operande.mantisse.total as usize, operandes.term_lng.operande.exposant.total as usize);

        let mut retenu = 0;
        let mut zéros: i32 = 0;
        let mut exposant = operandes.get_exposant(OpsTerm::Long);

        // Analyse soustraction avec zéro chiffre significatif
        if operandes.get_type() == NombreType::Entier || (operandes.get_type() == NombreType::Decimal && !operandes.has_entier) {
            if operandes.term_lng.operande.mantisse.total == 1 && !operandes.term_lng.operande.mantisse.any() {
                return Ok(operandes.term_crt.operande.clone());
            }
            if operandes.term_crt.operande.mantisse.total == 1 && !operandes.term_crt.operande.mantisse.any() {
                return Ok(operandes.term_lng.operande.clone());
            }
        }
        // Traitement BCD excédentaire à droite avec ou sans calcul compl_10
        // Cas exemple 1 sub avec zéro
        if total_lng > 0 && operandes.term_lng.deb_position > operandes.term_crt.deb_position {
            let limit = operandes.term_lng.deb_position - operandes.term_crt.deb_position;
            total_lng -= limit;
            // Traitement de l'unité à part pour supprimer le signe
            nbre.mantisse.append(iter_lng.next().unwrap())?;
            for _ in 0..limit - 1 { nbre.mantisse.append(iter_lng.next().unwrap())?; };
            if operandes.get_type() == NombreType::Entier { zéros = -1; }
        }
        // Cas exemple 4 (sub compl_10)
        else if total_crt > 0 && operandes.term_lng.deb_position < operandes.term_crt.deb_position {
            let mut pos_crt = operandes.term_crt.deb_position - operandes.term_lng.deb_position;
            total_crt -= pos_crt;
            retenu = 1;
            let limit = if pos_crt > iter_crt.len() { iter_crt.len() } else { pos_crt };
            // Traitement de l'unité à part pour supprimer le signe
            let unit = iter_crt.next().unwrap();
            if unit > 0 {
                nbre.mantisse.append(10 - unit)?;
            } else {
                nbre.mantisse.append(9)?;
            }
            for _ in 0..limit - 1 { nbre.mantisse.append(9 - iter_crt.next().unwrap())? };
            pos_crt -= limit;

            while pos_crt > 0 { // Complétion à 9
                nbre.mantisse.append(9)?;
                pos_crt -= 1;
            }
            if operandes.get_type() == NombreType::Entier { zéros = -1; }
        }

        // Traitement des BCD éventuellement alignés
        while let Some(mut n) = iter_crt.next() {
            match iter_lng.next() {
                Some(m) => {
                    n = m + 0x0A - n - retenu;
                    if n >= 0x0A { retenu = 0; n -= 0x0A; } else { retenu = 1; }
                    Nombre::zero_func(n, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                },
                None => {
                    n = 0x0A - n - retenu;
                    if n >= 0x0A { retenu = 0; n -= 0x0A; } else { retenu = 1; }
                    Nombre::zero_func(n, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                },
            }
            if total_lng > 0 {total_lng -= 1;}
        }

        // Traitement des BCD restants excédentaire à gauche dans le terme long
        if let Some(mut n) = iter_lng.next() {
            if total_lng > 0 {total_lng -= 1;}
            n += 0x0A - retenu;
            if n >= 0x0A { retenu = 0; n -= 0x0A; } else { retenu = 1; }
            Nombre::zero_func(n, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
            while let Some(n) = iter_lng.next() {
                Nombre::zero_func(n, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                if total_lng > 0 {total_lng -= 1;}
            }
        }

        // Traitement des compléments à 9 dans le cas de grand exposant décimal > longueur de la mantisse (cas : 1.00001 + -0.001 => 0.99901)
        if retenu == 1 && operandes.get_type() == NombreType::Decimal {
            while total_lng > 0 { // Complétion à 9
                Nombre::zero_func(9, operandes.get_type(), &mut nbre.mantisse, &mut exposant, &mut zéros);
                if total_lng > 0 {total_lng -= 1;}
            }
        }

        // Dernier traitement : reporter le zéro retenu dans le suivi lors de calcul amenant à un résultat nul
        if zéros > 0 && nbre.mantisse.total == 0 {
            nbre.mantisse.append(0)?;
            exposant = if operandes.get_type() == NombreType::Decimal { 1 } else { 0 };
        }
        // Reporter l'exposant et sa base
        if exposant > 0 {
            nbre.int_to_bcd(exposant, NombrePart::Exposant)?;
            nbre.base_exp = 10;
        }

        if retenu == 1 { Err(BcdError::Depassement(nbre, retenu)) }
        else { Ok(nbre) }
    }

    fn multiplication_entier(operandes: &InfosOps) -> Result<Nombre, BcdError> {
        let mut iter_rhs = operandes.term_crt.iter();
        let mut nbre = Nombre::with_capacity(operandes.get_type(), operandes.term_lng.operande.mantisse.total as usize, operandes.term_lng.operande.exposant.total as usize);
        let mut intermed = Nombre::with_capacity(operandes.get_type(), operandes.term_lng.operande.mantisse.total as usize, operandes.term_lng.operande.exposant.total as usize);
        let mut exp_nbre = 0;
        let mut res: u8 = 0;
        let mut zéros: i32 = 0;
        let mut compteur: u128 = 0; // suivre les itérations pour décaler l'addition
        let mut exp_lng = operandes.term_lng.deb_position;
        let exp_crt = operandes.term_crt.deb_position;
        while let Some(coef) = iter_rhs.next() {
            intermed.mantisse.reset();
            zéros = 0;
            let mut iter_self = operandes.term_lng.iter();
            while let Some(c) = iter_self.next() {
                res = c * coef + res;
                Nombre::zero_func(res % 10, operandes.get_type(), &mut intermed.mantisse, &mut exp_lng, &mut zéros);
                res = res / 10;
            }
            // Traitement de la retenue finale devenue excédentaire
            if res > 0 {
                Nombre::zero_func(res, operandes.get_type(), &mut intermed.mantisse, &mut exp_lng, &mut zéros);
                res = 0;
            }
            if nbre.mantisse.is_empty() {
                nbre = intermed.clone();
                exp_nbre = if zéros > 0 { zéros as u128 } else { 0 };
            } else {
                let exp = compteur + if zéros > 0 { zéros as u128 } else { 0 };
                nbre = Nombre::addition_positive(&InfosOps::new(
                    NombreOps::new(&nbre, exp_nbre, true),
                    NombreOps::new(&intermed, exp, false),
                    Ordering::Greater, false))?;
                    _ = Nombre::bcd_to_bin(&nbre.exposant, &mut exp_nbre);
            }
            compteur += 1;
        }
        exp_lng += exp_crt;
        if zéros > 0 {
            if operandes.get_type() == NombreType::Entier { exp_lng += zéros as u128; }
            else { exp_lng -= zéros as u128; }
        }
        // Reporter l'exposant et sa base
        if exp_lng > 0 {
            nbre.int_to_bcd(exp_lng, NombrePart::Exposant)?;
            nbre.base_exp = 10;
        }
        // Traiter le signe
        match (operandes.term_lng.is_signed(), operandes.term_crt.is_signed()) {
            (true, true) => nbre.mantisse.unsigned(),
            (false, false) => nbre.mantisse.unsigned(),
            _ => nbre.mantisse.signed()
        }

        Ok(nbre)
    }
}

/*
    Cas arithmétique addition => (signe à conserver)
    1) (-) long + (+) court => (-) long - court
    2) (-) long + (-) court => (-) long + court
    3) (+) long + (-) court => (+) long - court
    4) (+) long + (+) court => (+) long + court

    Cas long - court : 5 - 2 => 3 ou complément à 10 = 13 (8 + 5) et retrancher 10 (pas de retenu >= 10)
                        2 - 5 => 7 ou complément à 10 = 7 (5 + 2) (retenu < 10)
                        0 - 5 => 5 ou complément à 10 = 5 (5 + 0) (retenu < 10)
    Cas long + court : 5 + 2 => 7 pas besoin de complément
                        5 + 9 => 14 et retrancher 10 (retenu > 10)
                        0 + 9 => 9
*/
impl Add for Nombre {
    type Output = Result<Self, BcdError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.type_nbr != rhs.type_nbr {
            return Err(BcdError::TypeDifferent);
        }

        let operandes = self.selection_termes(&rhs)?;
        let mut nbre = match (operandes.term_lng.is_signed(), operandes.term_crt.is_signed()) {
            (true, false) => Nombre::soustraction_positive(&operandes),
            (false, true) => Nombre::soustraction_positive(&operandes),
            _ => Nombre::addition_positive(&operandes),
        }?;

        match operandes.term_lng.is_signed() {
            true => nbre.mantisse.signed(),
            false => nbre.mantisse.unsigned(),
        }

        Ok(nbre)
    }
}

/*
    Cas arithmétique soustraction => (signe à conserver)
    1) (+) self - (-) rhs => self + rhs (+)
    2) (+) self - (+) rhs => self - rhs (si rhs mantisse > self - sinon +)
    3) (-) self - (+) rhs => self + rhs (-)
    4) (-) self - (-) rhs => self - rhs (si rhs mantisse > self + sinon -)
*/
impl Sub for Nombre {
    type Output = Result<Self, BcdError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.type_nbr != rhs.type_nbr {
            return Err(BcdError::TypeDifferent);
        }

        let operandes = self.selection_termes(&rhs)?;
        let mut nbre = match (operandes.term_lng.is_signed(), operandes.term_crt.is_signed()) {
            (false, true) => Nombre::addition_positive(&operandes),
            (true, false) => Nombre::addition_positive(&operandes),
            _ => Nombre::soustraction_positive(&operandes),
        }?;

        match (self.mantisse.is_signed(), rhs.mantisse.is_signed()) {
            (false, true) => {nbre.mantisse.unsigned();},
            (false, false) => if !operandes.term_lng.is_lhs { nbre.mantisse.signed(); }
                            else { nbre.mantisse.unsigned(); } ,
            (true, false) => {nbre.mantisse.signed();},
            (true, true) => if !operandes.term_lng.is_lhs { nbre.mantisse.unsigned(); }
                            else { nbre.mantisse.signed(); },
        }

        Ok(nbre)
    }
}

impl Mul for Nombre {
    type Output = Result<Self, BcdError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.type_nbr != rhs.type_nbr {
            return Err(BcdError::TypeDifferent);
        }
        let mut exp_self = 0;
        let mut exp_rhs = 0;
        _ = Nombre::bcd_to_bin(&self.exposant, &mut exp_self);
        _ = Nombre::bcd_to_bin(&rhs.exposant, &mut exp_rhs);
        Nombre::multiplication_entier(&InfosOps::new(
                                                NombreOps::new(&self, exp_self, true),
                                                NombreOps::new(&rhs, exp_rhs, false),
                                                Ordering::Greater, true))
    }
}

impl Div for Nombre {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut nbre = Nombre::new(self.type_nbr);

        nbre
    }
}

// Faire son propre comparatif, celui par défaut ne va pas
// D'abord comparer l'adresse mémoire des objets puis leurs valeurs BCD
impl PartialEq for Nombre {
    fn eq(&self, other: &Self) -> bool {
        if !ptr::eq(self, other) {
            self.partial_cmp(&other).unwrap().is_eq()
        } else {
            true
        }
    }
}

impl PartialOrd for Nombre {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut iter_self = self.mantisse.iter();
        let mut iter_other = other.mantisse.iter();

        match (self.type_nbr, other.type_nbr) {
            (NombreType::Entier, NombreType::Decimal) => {
                if iter_self.next()? > 0 {
                    return Some(Ordering::Greater);
                }
            },
            (NombreType::Decimal, NombreType::Entier) => {
                if iter_other.next()? > 0 {
                    return Some(Ordering::Less);
                }
            },
            (_, _) => ()
        }

        match (self.mantisse.is_signed(), other.mantisse.is_signed()) {
            (false, true) => return  Some(Ordering::Greater),
            (true, false) => return  Some(Ordering::Less),
            _ => ()
        }

        // Réutilisation de la comparaison des mantisses sans signe
        let ops = self.selection_termes(other).unwrap();
        let mut ord = ops.ordering();
        // Après la comparaison des nombres en positif, il faut tenir compte du signe et du terme
        // forcément +/+ ou -/- tous les 2 (cf. second match -> signed)
        if self.mantisse.is_signed() {
            ord = match ord {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        };
        // Ensuite faut tenir compte si terme retenu comme supérieur est bien self
        if !ops.term_lng.is_lhs {
            ord = match ord {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        };

        return Some(ord);
    }
}

impl From<&str> for Nombre {
    fn from(value: &str) -> Self {
        let mut nbre = Nombre::new(NombreType::Entier);
        if !value.is_empty() {
            if value.contains(".") {
                let splits: Vec<&str> = value.split(".").collect();
                if splits.len() == 2 {
                    nbre = Nombre::new(NombreType::Decimal);
                    if let Err(err) = nbre.str_to_bcd(splits[1]) {
                        panic!("Erreur de conversion du décimal : {:?}", err);
                    }
                }
            } else {
                if let Err(err) = nbre.str_to_bcd(value) {
                    panic!("Erreur de conversion de l'entier : {:?}", err);
                }
            }
        }

        nbre
    }
}