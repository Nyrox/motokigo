macro_rules! implement_func {
	( $func:ident, $impl:expr, $name:ident, $ret:ident ) => {
        paste::item! {
            #[generate_builtin_fn("{}", [<$func:lower>])]
            fn [<$name $func>](a: $name) -> $ret {
                $impl
            }

            #[generate_glsl_impl_inline("{}{}", $name, $func)]
            fn generate(a: &str) -> String {
                format!(concat!(stringify!([<$func:lower>]), "({})"), a)
            }
        }
    };
    ( $func:ident, $impl:expr, $name1:ident, $name2:ident, $ret:ident ) => {
        paste::item! {
            #[generate_builtin_fn("{}", [<$func:lower>])]
            fn [<$name1 $name2 $func>](a: $name1, b: $name2) -> $ret {
                $impl
            }

            #[generate_glsl_impl_inline("{}{}{}", $name1, $name2, $func)]
            fn generate(a: &str, b: &str) -> String {
                format!(concat!(stringify!([<$func:lower>]), "({}, {})"), a, b)
            }
        }
    };
    ( $func:ident, $glfunc:ident, $impl:expr, $glimpl:literal, $name:ident, $ret:ident ) => {
        paste::item! {
            #[generate_builtin_fn("{}", [<$glfunc>])]
            fn [<$name $func>](a: $name) -> $ret {
                $impl
            }

            #[generate_glsl_impl_inline("{}{}", $name, $func)]
            fn generate(a: &str) -> String {
                format!($glimpl, a)
            }
        }
    };
    ( $func:ident, $glfunc:ident, $impl:expr, $glimpl:literal, $name1:ident, $name2:ident, $ret:ident ) => {
        paste::item! {
            #[generate_builtin_fn("{}", [<$glfunc>])]
            fn [<$name1 $name2 $func>](a: $name1, b: $name2) -> $ret {
                $impl
            }

            #[generate_glsl_impl_inline("{}{}{}", $name1, $name2, $func)]
            fn generate(a: &str, b: &str) -> String {
                format!($glimpl, a, b)
            }
        }
    };
}