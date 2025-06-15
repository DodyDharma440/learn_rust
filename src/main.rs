fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test!");
}

#[test]
fn test_var() {
    let mut name = "John";
    println!("Hello {}", name);

    name = "Doe";
    println!("Hello {}", name);

    let name = 12.0;
    println!("Name Number {}", name);
}

#[test]
fn boolean_op() {
    let absen = 70;
    let nilai_akhir = 80;

    let is_lulus = absen >= 75;
    let is_lulus_nilai = nilai_akhir >= 75;

    let lulus_final = is_lulus && is_lulus_nilai;
    let lulus_final = if lulus_final { "Lulus" } else { "Tidak Lulus" };
    println!("{}", lulus_final);
}

fn stack_a() {
    let a = 10;
    let b = String::from("Doe");
    println!("{} {}", a, b)
}

fn stack_b() {
    let a = 10;
    let b = String::from("John");

    println!("{} {}", a, b)
}

#[test]
fn stack_heap() {
    stack_a();
    stack_b();
}

#[test]
fn string() {
    let name: &str = "   John Doe   ";
    println!("{}", name);

    let trim = name.trim();
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name = String::from("John");
    name.push_str("Doe");
    println!("{}", name);

    let budi = name.replace("John", "Jane");
    println!("{}", budi)
}

// SOAL 1
// Terdapat kumpulan variable dengan data string sebagai berikut
// var first = 'Rust';                                                                                                          var word = var word = 'dart';
// var second = 'is';
// var third = 'awesome';
// var fourth = 'and';
// var fifth = 'I';
// var sixth = 'love';
// var seventh = 'it!'; Buatlah agar kata-kata di atas menjadi satu kalimat . Output: Dart is awesome and I love it!
#[test]
fn soal1() {
    let first = "Rust";
    let second = "is";
    let third = "awesome";
    let fourth = "and";
    let fifth = "I";
    let sixth = "love";
    let seventh = "it!";

    let sentence: [&str; 7] = [first, second, third, fourth, fifth, sixth, seventh];

    println!("{}", sentence.join(" "))
}

// SOAL 2
// Terdapat satu kalimat seperti berikut: var sentence = "I am going to be Rust Developer";

// var exampleFirstWord = sentence[0] ;
// var exampleSecondWord = sentence[2] + sentence[3] ;
// var thirdWord; // lakukan sendiri
// var fourthWord; // lakukan sendiri
// var fifthWord; // lakukan sendiri
// var sixthWord; // lakukan sendiri
// var seventhWord; // lakukan sendiri

// print('First Word: ' + exampleFirstWord);
// print('Second Word: ' + secondWord);
// print('Third Word: ' + thirdWord);
// print('Fourth Word: ' + fourthWord);
// print('Fifth Word: ' + fifthWord);
// print('Sixth Word: ' + sixthWord);
// print('Seventh Word: ' + seventhWord);

// Buat menjadi Output berikut: First word: I
// Second word: am
// Third word: going
// Fourth word: to
// Fifth word: be
// Sixth word: Rust
// Seventh word: Developer

#[test]
fn soal2() {
    let sentence = "I am going to be Rust Developer";

    fn make_word(s: &str, start_index: i32, end_index: i32) -> String {
        let mut word = String::from("");
        for i in start_index..end_index {
            let character = s.chars().nth(i as usize);
            match character {
                Some(c) => word.push(c),
                None => println!("Character not found"),
            }
        }
        return word;
    }

    let first_word = make_word(&sentence, 0, 1);
    let second_word = make_word(&sentence, 2, 4);
    let third_word = make_word(&sentence, 5, 10);
    let fourth_word = make_word(&sentence, 11, 13);
    let fifth_word = make_word(&sentence, 14, 16);
    let sixth_word = make_word(&sentence, 17, 21);
    let seventh_word = make_word(&sentence, 22, 31);

    println!(
        "{} {} {} {} {} {} {}",
        first_word, second_word, third_word, fourth_word, fifth_word, sixth_word, seventh_word
    )
}
