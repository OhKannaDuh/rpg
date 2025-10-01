#[macro_export]
macro_rules! use_mod {
    ( $( $m:ident ),+ $(,)? ) => {
        $( mod $m; pub use $m::*; )+
    }
}
