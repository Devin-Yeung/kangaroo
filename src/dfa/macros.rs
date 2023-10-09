#[macro_export]
macro_rules! state {
    ( $( $name:ident ),* ) => {
        $(
            let $name = $crate::dfa::core::State::new(stringify!($name));
        )*
    };
}

#[macro_export]
macro_rules! transition {
    ($builder:expr, $($from:expr, $via:literal -> $to:expr),* $(,)?) => {
        $(
            $builder.transition($from.clone(), $via, $to.clone());
        )*
    };
}

#[macro_export]
macro_rules! accept {
    ($builder:expr, $($state:expr),* $(,)?) => {
        $(
            $builder.accept($state.clone());
        )*
    };
}

#[macro_export]
macro_rules! start {
    ($builder:expr, $state:expr, $(,)?) => {
        $builder.start($state.clone());
    };
}
