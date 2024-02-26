/// Returns a bool that indicates whether or not number is even. 
/// 
/// # Examples
/// ```
/// let number = 5;
/// if is_even(number) {
///     println!("{number} is even.")
/// } else {
///     println!("{number} is not even.")
/// }
/// ```
pub fn is_even<T> (number: T) -> bool where 
    T:
        std::ops::Rem<Output = T> +
        std::cmp::PartialEq<T> +
        From<u8>
{
    number % T::from(2) == T::from(0)
}

/// Returns a bool that indicates whether or not number is odd. 
/// 
/// # Examples
/// ```
/// let number = 5;
/// if is_odd(number) {
///     println!("{number} is odd.")
/// } else {
///     println!("{number} is not odd.")
/// }
/// ```
pub fn is_odd<T> (number: T) -> bool where 
    T:
        std::ops::Rem<Output = T> +
        std::cmp::PartialEq<T> +
        From<u8>
{
    number % T::from(2) != T::from(0)
}