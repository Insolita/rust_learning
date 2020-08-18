fn type_name<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}
fn primitives() {
    let num1: i32 = 10;
    let num2: i16 = 5;
    let sum = num1 + i32::from(num2);
    let is_greater = sum > 5;
    println!("Sum is {} ({})", sum, type_name(&sum));
    println!("{} ({})", is_greater, type_name(&is_greater));
    let div = sum / num1; // By default division for integers is integer
    let div2 :f32 = sum as f32 / num1 as f32; // Force type casting required
    let rem = sum % num1;
    println!("{}, {}, {}", div, div2, rem);
    println!("{}, {}, {}", type_name(&div), type_name(&div2), type_name(&rem));
}

pub fn main() {
    primitives();
}