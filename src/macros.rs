#[macro_export]
macro_rules! default {
    ($name: ident: $typ: ty = #into $val: expr) => {
        concat_idents::concat_idents!(fn_name = default_, $name {
            fn fn_name() -> $typ {
                $val.into()
            }
        });
    };

    ($name: ident: $typ: ty = $val: expr) => {
        concat_idents::concat_idents!(fn_name = default_, $name {
            fn fn_name() -> $typ {
                $val
            }
        });
    };
}
