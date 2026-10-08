fn main() {
    println!("Hello, world!");

    println!("");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn variable() {
    let name = "Raihan Daniswara";
    println!("Hello, {}!", name)
}

#[test]
fn immutable() {
    // consider making this binding mutable: `mut `
    let name = "Raihan Daniswara";
    println!("Hello, {}!", name);

    // // cannot assign twice to immutable variable `name`
    // name = "Raihan";
    // println!("Hello, {}!", name);
}

#[test]
fn mutable() {
    let mut name = "Raihan Daniswara";
    println!("Hello, {}!", name);

    name = "Tommy Kemas";
    println!("Hello, {}!", name);
}

#[test]
fn static_typing() {
    // harusnya mut karena di redeclare dibawah, tapi gk dipake karena warning
    let name = "Raihan Daniswara";
    println!("Hello, {}!", name);

    // expected &str, found i32
    // name = 10;
    // println!("Hello, {}!", name);
}

#[test]
fn shadowing() {
    let name = "Raihan Daniswara";
    println!("Hello, {}!", name);

    let name = 10;
    println!("Hello, {}!", name);
}

#[test]
fn explicit() {
    let age: i8 = 17;
    println!("My age is {}", age)
}

#[test]
fn datatype() {
    fn scalar() {
        // --- Integer Signed ---
        // min: -128, max: 127
        let i_8: i8 = 100;
        println!("i8: {}", i_8);

        // min: -32,768, max: 32,767
        let i_16: i16 = 1000;
        println!("i16: {}", i_16);

        // min: -2,147,483,648, max: 2,147,483,647
        let i_32: i32 = 100_000;
        println!("i32: {}", i_32);

        // min: -9,223,372,036,854,775,808, max: 9,223,372,036,854,775,807
        let i_64: i64 = 1_000_000;
        println!("i64: {}", i_64);

        // min: -1.7e+38, max: 1.7e+38
        let i_128: i128 = 10_000_000;
        println!("i128: {}", i_128);

        // min / max: tergantung arsitektur CPU (32-bit atau 64-bit)
        let i_size: isize = 100;
        println!("isize: {}", i_size);

        // --- Integer Unsigned ---
        // min: 0, max: 255
        let u_8: u8 = 250;
        println!("u8: {}", u_8);

        // min: 0, max: 65,535
        let u_16: u16 = 50_000;
        println!("u16: {}", u_16);

        // min: 0, max: 4,294,967,295
        let u_32: u32 = 1_000_000;
        println!("u32: {}", u_32);

        // min: 0, max: 18,446,744,073,709,551,615
        let u_64: u64 = 10_000_000;
        println!("u64: {}", u_64);

        // min: 0, max: 3.4e+38
        let u_128: u128 = 100_000_000;
        println!("u128: {}", u_128);

        // min / max: tergantung arsitektur CPU (32-bit atau 64-bit)
        let u_size: usize = 100;
        println!("usize: {}", u_size);

        // --- Floating Point ---
        // f32: 32-bit float
        let f_32: f32 = 3.14;
        println!("f32: {}", f_32);

        // f64: 64-bit float (default)
        let f_64: f64 = 3.141592653589793;
        println!("f64: {}", f_64);

        // --- Boolean ---
        // nilai: true atau false
        let is_active: bool = true;
        println!("bool: {}", is_active);

        // --- Character ---
        // karakter Unicode 4-byte (e.g. 'A', '1')
        let char_a: char = 'A';
        println!("char: {}", char_a);
    }

    fn compound_type() {
        // Tuple (kumpulan nilai dengan tipe yang bisa berbeda)
        let tuple: (i32, f64, bool) = (10, 3.14, true);
        println!("tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

        // Array (kumpulan nilai dengan tipe yang harus sama & ukuran tetap)
        let array: [i32; 3] = [1, 2, 3];
        println!("array: {:?}", array);
    }

    scalar();
    compound_type();
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("a i8: {}", a);

    let a_i16: i16 = a as i16;
    println!("a i16: {}", a_i16);

    let a_i32: i32 = a_i16 as i32;
    println!("a i32: {}", a_i32);

    // tidak error tapi akan terjadi integer overflow
    let b: i64 = 1000000000;
    println!("b i64: {}", b);
    let b_i8: i8 = b as i8;
    println!("b i8: {}", b_i8);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 20;

    // Penjumlahan (Addition)
    let sum = a + b;
    println!("a + b = {}", sum);

    // Pengurangan (Subtraction)
    let subtr = a - b;
    println!("a - b = {}", subtr);

    // Perkalian (Multiplication)
    let mult = a * b;
    println!("a * b = {}", mult);

    // Pembagian (Division)
    let division: f32 = a as f32 / b as f32;
    println!("a / b = {}", division);

    // Sisa Bagi (Modulo)
    let modulo = a % b;
    println!("a % b = {}", modulo);
}

#[test]
fn augmented_assignment() {
    // harus mutable
    let mut a = 10;
    println!("a : {}", a);

    // Penjumlahan (Addition)
    a += 10;
    println!("a += 10 : {}", a);

    // Pengurangan (Subtraction)
    a -= 10;
    println!("a += 10 : {}", a)
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b)
}

#[test]
fn comparison_operator() {
    let a = 10;
    let b = 20;
    let result: bool = a > b;
    println!("result {} > {} = {}", a, b, result)
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus_final: bool = lulus_absen && lulus_nilai_akhir;
    println!("lulus final = {}", lulus_final)
}

#[test]
fn tuple() {
    // gunakan mut agar tuple bisa diubah datanya
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("Data tuple = {:?}", data); // Harus pakai {:?} (debug) karena () tidak punya Display

    // // mengakses data dari index
    // let data1 = data.0;
    // println!("Data 1 = {}", data1);
    // let data2 = data.1;
    // println!("Data 2 = {}", data2);
    // let data3 = data.2;
    // println!("Data 3 = {}", data3);

    // mengakses data dari destructuring tuple
    // gunakan _ untuk tidak mengambil data dalam tuple
    let (data1, data2, _) = data;
    println!("Data 1 = {}", data1);
    println!("Data 2 = {}", data2);
    // println!("Data 3 = {}", data3);

    data.0 = 20;
    data.1 = 15.25;
    data.2 = false;
    println!("Data tuple hasil mutable = {:?}", data);
}

// Function yang tidak menentukan return type (seperti void di C/Java),
// secara default mengembalikan "unit type" yaitu ()
fn unit() {
    println!("Hello")
}

#[test]
fn test_unit() {
    // Hasil return function tanpa nilai balik adalah ()
    let hasil: () = unit();
    println!("{:?}", hasil);

    // Unit value juga bisa dideklarasikan langsung sebagai tuple kosong ()
    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    // gunakan mut agar array bisa diubah datanya
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Data array = {:?}", array);

    // // mengakses data dari index
    // let array1 = array[0];
    // println!("Array 1 = {}", array1);
    // let array2 = array[1];
    // println!("Array 2 = {}", array2);
    // let array3 = array[2];
    // println!("Array 3 = {}", array3);

    // mengakses isi array dari destructuring array
    // gunakan _ untuk tidak mengambil data dalam array
    let [array1, array2, _, array4, _] = array;
    println!("Array 1 = {}", array1);
    println!("Array 2 = {}", array2);
    println!("Array 4 = {}", array4);

    array[0] = 10;
    array[1] = 20;
    println!("Data array hasil mutable = {:?}", array);

    // mendapatkan jumlah panjang data di array dengan len()
    let array_lenght = array.len();
    println!("Total panjang data array = {}", array_lenght);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("matrix = {:?}", matrix);
    println!("matrix 1 = {:?}", matrix[0]);
    println!("matrix 1 index 1 = {:?}", matrix[0][0]);
    println!("matrix 1 index 2 = {:?}", matrix[0][1]);
    println!("matrix 1 index 3 = {:?}", matrix[0][2]);
    println!("matrix 2 = {:?}", matrix[1]);
    println!("matrix 2 index 1 = {:?}", matrix[1][0]);
    println!("matrix 2 index 2 = {:?}", matrix[1][1]);
    println!("matrix 2 index 3 = {:?}", matrix[1][2]);
}

#[test]
fn constant() {
    // variable immutable yang tidak dapat diubah
    // wajib sebutkan tipe datanya secara explisit
    // penamaan variablenya seharusnya menggunakan SCREAMING_SNAKE_CASE
    // harus langsung di deklarasikan nilainya
    const MINIMUM: i8 = 0;
    const MAXIMUM: i16 = 1000;
    println!("Minimum = {}", MINIMUM);
    println!("Maximum = {}", MAXIMUM);
}

const SCOPE1: i8 = 1;

#[test]
fn variable_scope() {
    println!("scope 1 = {}", SCOPE1);
    let scope2 = 2;
    {
        println!("scope 2 = {}", scope2);
        let scope3 = 3;
        println!("scope = {}, {}, {}", SCOPE1, scope2, scope3)
    }

    // akan error karena tidak dapat mengambil variabel scope 3 yang diluar dari jangkauan scope nya
    // println!("scope 3 = {}", scope3)
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Raihan");
    println!("{} {}", a, b)
}

fn function_b() {
    let a = 20;
    let b = String::from("Daniswara");
    println!("{} {}", a, b)
}

#[test]
fn string() {
    let name: &str = "    Raihan Daniswara    ";
    println!("my name is {}", name);

    let trimmed_name: &str = name.trim();
    println!("my trimmed name is {}", trimmed_name);

    let mut username: &str = "Budi";
    println!("{}", username);

    username = "Tommy";
    println!("{}", username)
}

#[test]
fn string_type() {
    let mut name: String = String::from("Raihan");
    println!("{}", name);

    name.push_str(" Daniswara");
    println!("{}", name);

    let replaced_name = name.replace("Raihan", "Kemas");
    println!("{}", replaced_name);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses disini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai disini

    {
        // b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai disini
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // copy data dari a ke b

    println!("{} {}", a, b)
}

#[test]
fn ownership_movement() {
    // move occurs because `name1` has type `String`, which does not implement the `Copy` trait
    let name1: String = String::from("Raihan");
    println!("{}", name1);

    let name2: String = name1; // ownership pindah ke name2
    println!("{}", name2);

    // borrow of moved value: `name1`
    // value borrowed here after move
    // println!("{}", name1);
}

#[test]
fn clone() {
    // clone digunakan untuk copy isi data dari 1 variabel ke yang lain
    // clone digunakan untuk tipe data yang tidak fixed (disimpan di heap)
    let name1: String = String::from("Raihan");
    let name2: String = name1.clone(); // membuat data tiruan yang isinya mengcopy dari data di variable name1

    println!("{} {}", name1, name2);
}

#[test]
fn if_expesion() {
    let value = 3;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("Result = {}", result)
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter >= 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter = {}", counter)
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };
    println!("Result = {}", result)
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;

            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter = {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration_while_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value = {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value = {}", value)
    }
}

#[test]
fn range_exclude() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let range = 0..5;
    println!("Range start = {}", range.start);
    println!("Range end = {}", range.end);

    for i in range {
        println!("Array value = {}", array[i])
    }
}

#[test]
fn range_include() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let range = 0..=4;
    println!("Range start = {}", range.start());
    println!("Range end = {}", range.end());

    for i in range {
        println!("Array value = {}", array[i])
    }
}

fn say_hello() {
    println!("Hello!")
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}!", first_name, last_name)
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_function() {
    say_goodbye("Raihan", "Daniswara");
    say_goodbye("Kayana", "Hafidz");
    say_goodbye("Tommy", "Kemas");

    let result = factorial_loop(5);
    println!("Result = {}", result);

    let result = factorial_loop(-10);
    println!("Result = {}", result)
}

fn print_text(value: String, times: i32) {
    if times <= 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

fn factorial_recursive(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    return n * factorial_recursive(n - 1);
}

#[test]
fn test_recursive() {
    print_text(String::from("Raihan Daniswara"), 10);

    let factorial_result = factorial_recursive(10);
    println!("Factorial Result = {}", factorial_result)
}

fn print_number(number: i32) {
    println!("number: {}", number)
}

fn hi(name: String) {
    println!("Hi, {}", name)
}

fn full_name(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    // kirim first_name dan juga last_name agar tetap bisa digunakan setelah berpindah ownership
    (first_name, last_name, full_name)
}

#[test]
fn test_function_ownership() {
    let number = 10;
    print_number(number);
    // tetap bisa karena disimpan di stack
    println!("Number = {}", number);

    let name = String::from("Raihan Daniswara");
    hi(name);
    // borrow of moved value: name
    // tidak bisa karena ownershipnya sudah berpindah ke name
    // println!("Name = {}", name);

    let first_name = String::from("Raihan");
    let last_name = String::from("Daniswara");
    let (first_name, last_name, full_name) = full_name(first_name, last_name);
    println!("Full name = {}", full_name);
    println!("First name = {}", first_name);
    println!("Last name = {}", last_name);
}

fn full_name_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

fn change_value(value: &String) {
    // cannot borrow `*value` as mutable, as it is behind a `&` reference
    // `value` is a `&` reference, so it cannot be borrowed as mutable
    //
    // Secara default data reference tidak bisa di ubah meskipun variabel aslinya mutable,
    // karena function ini hanya meminjam data tersebut dan nanti akan dikembalikan lagi tanpa modifikasi
    // value.push_str("Test");
}

fn change_value_mutable(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_reference() {
    let first_name = String::from("Raihan");
    let last_name = String::from("Daniswara");
    let full_name = full_name_reference(&first_name, &last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);

    let mut value = String::from("Raihan");
    change_value(&value);
    println!("{}", value);

    change_value_mutable(&mut value);
    println!("{}", value);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Raihan Daniswara");
    let first_name: &str = &name[..6];
    println!("First name = {}", first_name);

    let last_name: &str = &name[7..];
    println!("Last name = {}", last_name)
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("First name = {}", person.first_name);
    println!("Last name = {}", person.last_name);
    println!("Age = {}", person.age);
}

#[test]
fn struct_person() {
    // bisa gunakan shorthand untuk pengisian data struct, tetapi harus sama nama variable nya dengan field struct nya
    // ownership dari data first_name juga akan berpindah ke person, jadi sudah tidak bisa diakses melalui variable first_name
    let first_name = String::from("Raihan");

    let person: Person = Person {
        age: 17,
        first_name,
        last_name: String::from("Daniswara"),
    };

    print_person(&person);
    // borrow of moved value: first_name
    // println!("First name = {}", first_name);

    // hati hati, jika ada field di data person yang disimpan di heap, maka akan pindah ownership
    // jadi sebaiknya gunakan .clone() khusus untuk di tipe data field yang disimpan di heap
    let person2: Person = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person2);
    println!("{}", person.first_name);
    println!("{}", person.last_name);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point: GeoPoint = GeoPoint(-6.12353, 100.23453);
    println!("Latitude = {}", geo_point.0);
    println!("Longitude = {}", geo_point.1);
}

struct Nothing;

#[test]
fn no_field_struct() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing {};
}
