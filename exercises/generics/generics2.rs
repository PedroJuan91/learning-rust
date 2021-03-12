// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.


// struct Wrapper{
//     value: u32,
    // value: Y
// }
struct Wrapper<T> {
    value: T, 
}

impl<T> Wrapper<T> {
    pub fn new(val: T) -> Self {
        // Wrapper{value: value};
        // &self.value
        Wrapper {
            value: val
        }
    }
}
// impl<T> GenWrapper<T> {
//     pub fn new(&self)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
