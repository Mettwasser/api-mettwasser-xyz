macro_rules! error_codes {
    ( $( $name:ident = $value:expr ),+$(,)? ) => {
        $(
            pub static $name: u16 = $value;
        )+
    };

    ( $( $category:literal: $( $name:ident ),* );+ $(;)? ) => {
        $(
            $(
                pub static $name: u16 = $category;
            )*
        )+
    };
}

error_codes! {
    200:
        OK;
    400:
        INVALID_URL,
        BAD_ARGUMENTS;
}
