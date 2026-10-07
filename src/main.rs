fn main() {
    println!("Hello, world!");

    println!("Hello, Raihan!");
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
