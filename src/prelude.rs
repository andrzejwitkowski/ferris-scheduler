#[macro_export]
macro_rules! with_closure {
    // Method call version with dot-like syntax
    ($obj:expr => $method:ident($($args:expr),*) with { $($param:ident),* -> $body:expr }) => {
        $obj.$method($($args),*, |$($param),*| $body)
    };

    // Function call version
    ($func:ident($($args:expr),*) with { $($param:ident),* -> $body:expr }) => {
        $func($($args),*, |$($param),*| $body)
    };
}

