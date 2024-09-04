pub mod nombre;
use self::nombre::Nombre;

const SIGNE_POSITIVE:u8 = 0x0C;
const SIGNE_NEGATIVE:u8 = 0x0D;
const SIGNE_BIT_MASK:u8 = 0x0F; // Position du signe sur les 4 bits de poids faible du dernier chiffre
//const SIGNE_DEL_MASK:u8 = 0xF0; // Pour supprimer le signe de l'unité

const NIBBLE_RIGHT_BIT_MASK:u8 = 0x0F; // Pour la conversion du chiffre sur le nibble de droite d'un packed BCD et pour convertir ASCII chiffre en binaire chiffre (retire les bits 5 et 6)
const NIBBLE_LEFT_BIT_MASK:u8 = 0xF0;

const NOMBRE_FIN: u8 = 0x0B; // Marqueur de fin du nombre présent dans le nibble de droite quand on a une longueur pair de chiffres, si impair alors dernier chiffre significatif

#[derive(Debug)]
pub enum BcdError {
    HorsRang,
    TypeDifferent,
    Depassement(Nombre, u8),
    CaractereInvalide,
    AlignementInvalide,
    CalculErrone
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PackedBcd {
    chiffres: Vec<u8>,
    total: u128,
    index: usize,
    shift: bool
}

impl PackedBcd {
    fn new() -> Self {
        let mut p = PackedBcd {
            chiffres: Vec::new(),
            total: 0,
            index: 0,
            shift: false
        };

        p.chiffres.push(0);
        p
    }

    fn with_capacity(capacity: usize) -> Self {
        let mut p = PackedBcd {
            chiffres: Vec::with_capacity(capacity),
            total: 0,
            index: 0,
            shift: false
        };

        p.chiffres.push(0);
        p
    }

    // Met le signe négatif au nombre sur la partie chiffres
    //
    // Remarque : le signe sur la partie exposant est déterminé par le code et le type
    fn signed(&mut self) {
        if !self.chiffres.is_empty()
        { self.chiffres[0] = self.chiffres[0] & NIBBLE_LEFT_BIT_MASK | SIGNE_NEGATIVE; }
    }

    // Met le signe positif au nombre sur la partie chiffres
    //
    // Remarque : Toujours signer le nombre avec au moins le signe +
    fn unsigned(&mut self) {
        if !self.chiffres.is_empty()
        { self.chiffres[0] = self.chiffres[0] & NIBBLE_LEFT_BIT_MASK | SIGNE_POSITIVE; }
    }

    // Signal un signe positif par défaut
    fn is_signed(&self) -> bool {
        if !self.chiffres.is_empty() {
            self.chiffres[0] & SIGNE_BIT_MASK == SIGNE_NEGATIVE
        }
        else { false }
    }

    fn has_sign(&self) -> bool {
        if !self.chiffres.is_empty() {
            self.chiffres[0] & SIGNE_BIT_MASK == SIGNE_NEGATIVE ||
            self.chiffres[0] & SIGNE_BIT_MASK == SIGNE_POSITIVE
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.total == 0
    }

    fn iter(&self) -> BcdIterator {
        BcdIterator::new(&self)
    }

    fn append(&mut self, chiffre: u8) -> Result<(), BcdError> {
        if chiffre > 9 {
            return Err(BcdError::HorsRang);
        }
        self.shift = !self.shift;
        if self.shift {
            self.chiffres[self.index] = chiffre << 4 | (self.chiffres[self.index] & NIBBLE_RIGHT_BIT_MASK);
        } else {
            self.chiffres.push((NOMBRE_FIN << 4) | chiffre);
            self.index += 1;
        }
        self.total += 1;

        Ok(())
    }

    fn len(&self) -> usize {
        self.chiffres.len()
    }

    fn any(&self) -> bool {
        let mut it = self.iter();
        while let Some(n) = it.next() {
            if n != 0 {
                return true;
            }
        }

        false
    }

    fn reset(&mut self) {
        self.total = 0;
        self.index = 0;
        self.shift = false;
        self.chiffres.clear();
        self.chiffres.push(0);
    }

    fn pop(&mut self) -> Option<u8> {
        let val;
        if self.shift {
            val = self.chiffres[self.index] >> 4;
            self.chiffres[self.index] = NOMBRE_FIN << 4 | (self.chiffres[self.index] & NIBBLE_RIGHT_BIT_MASK);
        } else {
            val = self.chiffres[self.index] &NIBBLE_RIGHT_BIT_MASK;
            self.chiffres.pop();
        }
        if !self.shift && self.index == 0 {
            None
        } else {
            self.total -= 1;
            self.shift = !self.shift;
            if self.shift && self.index > 0 { self.index -= 1; }
            Some(val)
        }
    }
}

struct BcdIterator<'a> {
    shift_suiv: bool,
    shift_prec: bool,
    index_suiv: usize,
    index_prec: usize,
    bcd: &'a PackedBcd,
}

impl<'a> BcdIterator<'a> {
    fn new(bcd: &'a PackedBcd) -> Self {
        BcdIterator {
            shift_suiv: true,
            shift_prec: true,
            index_suiv: 0,
            index_prec: bcd.len() - 1,
            bcd
        }
    }

    fn len(&self) -> u128 {
        self.bcd.total
    }
}

impl<'a> Iterator for BcdIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_suiv < self.bcd.chiffres.len() {
            let c = if self.shift_suiv {
                self.index_suiv += 1;
                self.bcd.chiffres[self.index_suiv - 1] >> 4
            } else {
                self.bcd.chiffres[self.index_suiv] & NIBBLE_RIGHT_BIT_MASK
            };
            if c == NOMBRE_FIN {
                return None;
            } else {
                self.shift_suiv = !self.shift_suiv;
                return Some(c);
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for BcdIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.bcd.chiffres[self.index_prec] >> 4 == NOMBRE_FIN {
            self.shift_prec = false;
        }
        let c = if self.shift_prec {
            self.bcd.chiffres[self.index_prec] >> 4
        } else {
            self.bcd.chiffres[self.index_prec] & NIBBLE_RIGHT_BIT_MASK
        };

        if !self.shift_prec && self.index_prec == 0 {
            None
        } else {
            self.shift_prec = !self.shift_prec;

            if self.shift_prec && self.index_prec > 0 { self.index_prec -= 1; }
            Some(c)
        }
    }
}