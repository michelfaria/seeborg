// HashMap initialization macro
// Example usage:
// ```
// let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
// ```
// Thanks to:
// https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}