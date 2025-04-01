pub use embassy_futures::join::join as join2;

#[macro_export]
macro_rules! join {
    ( $future:expr ) => {
        $future
    };
    ( $fut1:expr, $($futN:expr),+ $(,)? ) => {
        $crate::futures::join2($fut1, $crate::join!( $($futN),+ ))
    };
}

