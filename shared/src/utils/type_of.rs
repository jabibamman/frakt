use std::any::type_name;

/// Returns the name of a type.
///
/// This function utilizes Rust's reflection capabilities to obtain the name of a type at runtime.
/// It's a generic function that can accept any type `T`.
///
/// # Generics
///
/// * `T`: The type for which the name will be obtained. This can be any Rust type.
///
/// # Arguments
///
/// * `_`: A parameter of type `T`. The value of this parameter is not used; it's only for type inference.
///
/// # Returns
///
/// Returns a string slice (`&'static str`) that represents the name of the type `T`.
///
/// # Examples
///
/// ```rust
/// use shared::utils::type_of::type_of;
/// 
/// assert_eq!(type_of(42), "i32");
/// assert_eq!(type_of("hello"), "&str");
/// ```
///
/// # Note
///
/// The returned type name is the one Rust uses internally. For common types, it's straightforward (like `i32` for integers),
/// but for more complex types, especially those involving generics, the returned name might be mangled or less readable.

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
