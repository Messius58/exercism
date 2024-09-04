/*
   Forme des rails (Ex. rang 6 avec intervalle de 10) :
⇩ Point de départ, on compte le nombre d'éléments entre 2 points de même intervalle.
_         _         _
 _       _ _       _ _
  _     _   _     _   _
   _   _     _   _     _
    _ _       _ _       X
     _         _         X
*/
type Diagonal = usize;
trait ZigZag {
    fn is_zag(&self) -> bool;
    fn is_zig(&self) -> bool;
}

impl ZigZag for Diagonal {
    #[inline]
    fn is_zag(&self) -> bool {
        *self & 1 == 0
    }

    #[inline]
    fn is_zig(&self) -> bool {
        !self.is_zag()
    }
}

#[test]
fn cypher_zag() {
    let mut n: Diagonal = 2;
    assert!(n.is_zag());
    n *= 2;
    assert!(n.is_zag());
    n *= 9;
    assert!(n.is_zag());
}

#[test]
fn cypher_zig() {
    let mut n: Diagonal = 3;
    assert!(!n.is_zag());
    n *= 3;
    assert!(!n.is_zag());
    n *= 9;
    assert!(!n.is_zag());
}

// https://en.wikipedia.org/wiki/Rail_fence_cipher
struct RailFence {
    count: u32
}

impl RailFence {
    fn new(rails: u32) -> Self {
        Self {
            count: rails
        }
    }

    fn count(&self) -> usize {
        self.count as usize
    }

    // distance maximale entre 2 points d'un même rail (zig - > zig ou zag -> zag)
    fn interval(&self) -> usize {
        2 * (self.count() - 1)
    }

    // Pour déterminer les "pas" entre chaque lettres d' un rail donné en fonction du nombre de rail.
    // il y a une symmétrie des pas étant donné la nature du triangle.
    // Ex : pour un rail de 4 : (en code - 1 pour index base 0)
    // rail tête : 6-6-6 ...
    // rail intermédiaire 1 : 4-2-4-2 ... Intermédiaires on a (zig -> zag, zag -> zig )
    // rail intermédiaire 2 : 2-4-2-4 ...
    // rail de queue : 6-6-6 ...
    //
    // Pour utiliser les pas, il suffit d'inclure ce vecteur en l'inversant dans un zip de ce même vecteur (cf. encode)
    fn steps(&self) -> Vec<u32> {
        let limit = self.interval() as u32;
        let mut steps = (2..=limit).rev().step_by(2).collect::<Vec<u32>>();
        steps.push(limit);

        steps
    }

    fn encode(&self, text: &str) -> String {
        let steps = self.steps();
        steps.iter().zip(steps.iter().rev()).enumerate().map(|(idx, (zig, zag))| {
            let mut iter = text.chars().skip(idx);
            let mut string = String::new();
            string.push(iter.next().unwrap());
            loop {
                // Il est nécessaire de décrémenter car le next précédent et nth décale la position courante.
                match (iter.nth((*zig - 1) as usize), iter.nth((*zag - 1) as usize)) {
                    (Some(a), Some(b)) => { string.push(a); string.push(b); },
                    (None, Some(b)) => string.push(b),
                    (Some(a), None) => string.push(a),
                    _ => break
                }
            }
        string
        }).collect::<String>()
    }

    fn decode(&self, cipher: &str) -> String {
        let rails = self.rails(cipher);
        let steps = self.steps();
        let mut string: Vec<char> = Vec::new();
        string.push(' ');
        string = string.repeat(cipher.len());
        _ = rails.iter().zip(steps.iter().zip(steps.iter().rev()))
            .enumerate().map(|(pos,(str, (zig, zag)))| {
            let mut iter = str.chars();
            let mut position = pos;
            string[position] = iter.next().unwrap();
            loop {
                match (iter.next(), iter.next()) {
                    (Some(a), Some(b)) => {
                        position += *zig as usize;
                        string[position] = a;
                        position += *zag as usize;
                        string[position] = b;
                    },
                    (None, Some(b)) => {
                        position += *zag as usize;
                        string[position] = b;
                    },
                    (Some(a), None) => {
                        position += *zig as usize;
                        string[position] = a;
                    },
                    _ => break
                }
            }
            ""
        }).collect::<String>();

        string.iter().collect::<String>()
    }

    // Découper la chaîne cryptée selon le nombre de rail.
    //
    // L + y / (N + x(N - 1)) pour découvrir les lettres manquantes
    // L = longueur du texte
    // N = nombre de rail
    // y = nombre de lettre manquante
    // x = nombre de diagonale complète
    //
    // K = L / 2(N - 1)
    // si les rails sont équilibrés alors rail de tête et queue sont égale à K
    // et rails intermédiaires de 2K. Sinon déterminer si rail incomplet zig ou zag.
    fn rails<'a>(&self, cipher: &'a str) -> Vec<&'a str> {
        let intervalle = self.interval();
        // Calcul du nombre de lettre pour chaque rail si équilibré
        let period_k = cipher.len() / intervalle;
        let mut lettre_par_rail = Vec::<usize>::with_capacity(self.count());
        lettre_par_rail.push(period_k); // 1er
        for _ in 0..self.count - 2 {
            lettre_par_rail.push(2 * period_k); // intermed
        }
        lettre_par_rail.push(period_k); // dernier

        let y = cipher.len() % intervalle;
        if y != 0 { // la diagonale finale est incomplète, vérification et correction éventuel de la longueur d'un rail
            let l = cipher.len() - self.count();
            let n_1 = self.count() - 1;
            let x: Diagonal = l / n_1 + 1; // + 1 la diagonale incomplète suite à l'entrée du if
            let (mut index, inc) : (i32, i32) = if x.is_zag() { (0, 1) } else { (lettre_par_rail.len() as i32 - 1, -1) };
            for _ in 0..y {
                lettre_par_rail[index as usize] += 1;
                index += inc;
            }
        }
        // Découpage des rails de n longueur selon leur place et correction (cf. period_k)
        let mut rails: Vec<&str> = Vec::new();
        let mut splits: (&str, &str) = ("", cipher);
        for rail_lng in lettre_par_rail {
            splits = splits.1.split_at(rail_lng);
            rails.push(splits.0);
        }

        // Si préférence d'utilisation avec pop au lieu d'iter, faut inverser le vecteur avant
        rails
    }
}

#[test]
fn encode_with_two_rails() {
    let input = "XOXOXOXOXOXOXOXOXO";
    let rails = 2;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "XXXXXXXXXOOOOOOOOO";
    assert_eq!(output, expected);
}
#[test]
fn encode_with_three_rails() {
    let input = "WEAREDISCOVEREDFLEEATONCE";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "WECRLTEERDSOEEFEAOCAIVDEN";
    assert_eq!(output, expected);
}
#[test]
fn encode_with_ending_in_the_middle() {
    let input = "EXERCISES";
    let rails = 4;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "ESXIEECSR";
    assert_eq!(output, expected);
}
#[test]
fn decode_with_three_rails() {
    let input = "TEITELHDVLSNHDTISEIIEA";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.decode(input);
    let expected = "THEDEVILISINTHEDETAILS";
    assert_eq!(output, expected);
}
#[test]
fn decode_with_five_rails() {
    let input = "EIEXMSMESAORIWSCE";
    let rails = 5;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.decode(input);
    let expected = "EXERCISMISAWESOME";
    assert_eq!(output, expected);
}
#[test]
fn decode_with_six_rails() {
    let input = "133714114238148966225439541018335470986172518171757571896261";
    let rails = 6;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.decode(input);
    let expected = "112358132134558914423337761098715972584418167651094617711286";
    assert_eq!(output, expected);
}
#[test]
fn encode_wide_characters() {
    let input = "古池蛙飛び込む水の音";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "古びの池飛込水音蛙む";
    assert_eq!(output, expected);
}