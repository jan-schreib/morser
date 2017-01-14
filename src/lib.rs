use std::collections::HashMap;

/// Encoder for International Standard morsecode
pub struct Encoder<'a> {
    it: HashMap<char, &'a str>,
}

/// Decoder for International Standard morsecode
pub struct Decoder<'b> {
    it: HashMap<&'b str, char>,
}

impl<'a> Encoder<'a> {
    /// Constructs a new `Encoder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate morser;
    /// use morser::Encoder;
    ///
    /// fn main() {
    ///     let encode = Encoder::new();
    /// }
    /// ```
    pub fn new() -> Encoder<'a> {
        let mut en = Encoder {
            it: HashMap::with_capacity(36),
        };
        en.it.insert('a', ".- ");
        en.it.insert('b', "-... ");
        en.it.insert('c', "-.-. ");
        en.it.insert('d', "-.. ");
        en.it.insert('e', ". ");
        en.it.insert('f', "..-. ");
        en.it.insert('g', "--. ");
        en.it.insert('h', ".... ");
        en.it.insert('i', ".. ");
        en.it.insert('j', ".--- ");
        en.it.insert('k', "-.- ");
        en.it.insert('l', ".-.. ");
        en.it.insert('m', "-- ");
        en.it.insert('n', "-. ");
        en.it.insert('o', "--- ");
        en.it.insert('p', ".--. ");
        en.it.insert('q', "--.- ");
        en.it.insert('r', ".-. ");
        en.it.insert('s', "... ");
        en.it.insert('t', "- ");
        en.it.insert('u', "..- ");
        en.it.insert('v', "...- ");
        en.it.insert('w', ".-- ");
        en.it.insert('x', "-..- ");
        en.it.insert('y', "-.-- ");
        en.it.insert('z', "--.. ");

        en.it.insert('0', "----- ");
        en.it.insert('1', ".---- ");
        en.it.insert('2', "..--- ");
        en.it.insert('3', "...-- ");
        en.it.insert('4', "....- ");
        en.it.insert('5', "..... ");
        en.it.insert('6', "-.... ");
        en.it.insert('7', "--... ");
        en.it.insert('8', "---.. ");
        en.it.insert('9', "----. ");
        en.it.insert(' ', "      ");

        en.it.shrink_to_fit();
        en
    }
    /// Encodes ascii into morse code.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate morser;
    /// use morser::Encoder;
    ///
    /// fn main() {
    ///     let encoder = Encoder::new();
    ///     let plain = "sos";
    ///     let morse = encoder.encode(plain);
    /// }
    /// ```
    pub fn encode(&self, plain: &str) -> String {
        let mut ret = String::new();
        for c in plain.to_lowercase().chars() {
            ret.push_str(self.it.get(&c).unwrap_or(&""));

        }

        ret.shrink_to_fit();
        ret
    }
}

impl<'b> Decoder<'b> {
    /// Constructs a new `Decoder`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate morser;
    /// use morser::Decoder;
    ///
    /// fn main() {
    ///     let decoder = Decoder::new();
    /// }
    /// ```
    pub fn new() -> Decoder<'b> {
        let mut de = Decoder {
            it: HashMap::with_capacity(36),
        };
        de.it.insert(".-", 'a');
        de.it.insert("-...", 'b');
        de.it.insert("-.-.", 'c');
        de.it.insert("-..", 'd');
        de.it.insert(".", 'e');
        de.it.insert("..-.", 'f');
        de.it.insert("--.", 'g');
        de.it.insert("....", 'h');
        de.it.insert("..", 'i');
        de.it.insert(".---", 'j');
        de.it.insert("-.-", 'k');
        de.it.insert(".-..", 'l');
        de.it.insert("--", 'm');
        de.it.insert("-.", 'n');
        de.it.insert("---", 'o');
        de.it.insert(".--.", 'p');
        de.it.insert("--.-", 'q');
        de.it.insert(".-.", 'r');
        de.it.insert("...", 's');
        de.it.insert("-", 't');
        de.it.insert("..-", 'u');
        de.it.insert("...-", 'v');
        de.it.insert(".--", 'w');
        de.it.insert("-..-", 'x');
        de.it.insert("-.--", 'y');
        de.it.insert("--..", 'z');

        de.it.insert("-----", '0');
        de.it.insert(".----", '1');
        de.it.insert("..---", '2');
        de.it.insert("...--", '3');
        de.it.insert("....-", '4');
        de.it.insert(".....", '5');
        de.it.insert("-....", '6');
        de.it.insert("--...", '7');
        de.it.insert("---..", '8');
        de.it.insert("----.", '9');
        de.it.insert("     ", ' ');
        de.it.shrink_to_fit();
        de
    }
    /// Decodes morse into ascii.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate morser;
    /// use morser::Decoder;
    ///
    /// fn main() {
    ///     let decoder = Decoder::new();
    ///     let morse = "... --- ...";
    ///     let plain = decoder.decode(morse);
    /// }
    /// ```
    pub fn decode(&self, morse: &str) -> String {
        let mut ret = String::new();

        for s in morse.split_whitespace() {
            ret.push(*self.it.get(s).unwrap_or(&' '));
        }
        ret
    }
}