#[macro_export]
macro_rules! state {
    ( $( $name:ident ),* ) => {
        $(
            let $name = $crate::dfa::core::State::new(stringify!($name));
        )*
    };
}
