extern crate morser;

use morser::Decoder;
use morser::Encoder;

#[test]
fn encode_empty() {
    assert_eq!(Encoder::new().encode(""), "");
}

#[test]
fn encode_letter() {
    assert_eq!(Encoder::new().encode("a"), ".- ");
}

#[test]
fn encode_sos() {
    assert_eq!(Encoder::new().encode("sos"), "... --- ... ");
}

#[test]
fn encode_two_words() {
    assert_eq!(Encoder::new().encode("morse code"), "-- --- .-. ... .       -.-. --- -.. . ");
}

#[test]
fn encode_all_letters() {
    let all = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --.. ";
    assert_eq!(Encoder::new().encode("abcdefghijklmnopqrstuvwxyz"), all);
}

#[test]
fn encode_all_numbers() {
    let all = "----- .---- ..--- ...-- ....- ..... -.... --... ---.. ----. ";
    assert_eq!(Encoder::new().encode("0123456789"), all);
}

#[test]
fn decode_empty() {
    assert_eq!(Decoder::new().decode(""),"");
}

#[test]
fn decode_to_letter() {
    assert_eq!(Decoder::new().decode(".-"), "a");
}

#[test]
fn decode_to_all_letters() {
    let all = "abcdefghijklmnopqrstuvwxyz";
    let m = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- \
             ...- .-- -..- -.-- --.. ";
    assert_eq!(Decoder::new().decode(m), all);
}

#[test]
fn decode_all_numbers() {
    let all = "0123456789";
    let m = "----- .---- ..--- ...-- ....- ..... -.... --... ---.. ----. ";
    assert_eq!(Decoder::new().decode(m), all);
}

#[test]
fn decode_two_words() {
    //todo: make space aware
    assert_eq!(Decoder::new().decode("-- --- .-. ... .       -.-. --- -.. . "), "morsecode");
}