fn main() {

    println!("Primitive values");

    let number_8bits: i8 = 127;
    let number_8bits_unsigned: u8 = 255;
    let number_16bits: i16 = 32767;
    let number_16bits_unsigned: u16 = 65535;
    let number_32_bits: i32 = 2147483647;
    let number_32_bits_unsigned: u32 = 4294967295;
    let number_64_bits: i64 = 9223372036854775807;
    let number_64_bits_unsigned: u64 = 18446744073709551615;
    let number_128_bits: i128 = 170141183460469231731687303715884105727;
    let number_128_bits_unsigned: u128 = 340282366920938463463374607431768211455;
    println!("Values (8, 16,32,64,128 signed and unsigned) integers i max values  is: \n{0}, {1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}",
             number_8bits, number_8bits_unsigned, number_16bits, number_16bits_unsigned, number_32_bits,
             number_32_bits_unsigned, number_64_bits, number_64_bits_unsigned, number_128_bits, number_128_bits_unsigned
    );
    let float_number_32bits: f32 = 45.4;
    let float_number_64bits: f64 = 128.1;
    println!("Float (32 and 64 bits) numbers sample: {0}, {1}", float_number_32bits, float_number_64bits);
    let bool_value: bool = true;
    println!("Boolean value: {0}", bool_value);
    let char_value: char = 'a';
    println!("Char value: {0}", char_value);
    let string_value: &'static str = "Hola que pasas";
    let string_ptr: &str = "Hola manito!";
    println!("{}", string_ptr);
    println!("String value: {0}", string_value);
    let sample_i_size: isize = 9223372036854775807; //system architecture
    println!("i size value: {0}", sample_i_size);
    let sample: usize = 18446744073709551615; //system architecture
    println!("u size value: {0}", sample);
}
