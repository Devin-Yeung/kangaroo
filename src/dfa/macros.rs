#[macro_export]
macro_rules! state {
    ( $( $name:ident ),* $(,)?) => {
        $(
            let $name = $crate::common::core::State::new(stringify!($name));
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

#[macro_export]
macro_rules! dfa {
    (

        $(state {
            $($name:ident),* $(,)?
        })?

        start {
            $start:expr $(,)?
        }

        // TODO: need further refactoring
        $(transition {
            $($from:expr, $via:literal -> $to:expr),* $(,)?
        })?

        $(transitions {
            $($from1:expr, [$($via1:literal)|* $(,)?] -> $to1:expr),* $(,)?
        })?

        accept {
            $($accept:expr),* $(,)?
        }
    ) => {{
        let mut builder = $crate::dfa::builder::DFABuilder::new();

        $(
            $(
                let $name = $crate::common::core::State::new(stringify!($name));
            )*
        )?

        builder.start($start.clone());

        $(
            $(
                builder.transition($from.clone(), $via, $to.clone());
            )*
        )?

        $(
            $(
                $(
                    builder.transition($from1.clone(), $via1, $to1.clone());
                )*
            )*
        )?

        $(
            builder.accept($accept.clone());
        )*

        builder.build()
    }};
}
