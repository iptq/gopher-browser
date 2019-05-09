use std::error::Error as StdError;
use std::fmt;

macro_rules! define_error {
    ($($vis:vis $name:ident { $( $vname:ident($vinto:path), )* })*) => {
        $(
            #[derive(Debug)]
            $vis enum $name {
                $(
                    $vname($vinto),
                )*
            }

            $(
                impl From<$vinto> for $name {
                    fn from(err: $vinto) -> Self {
                        $name::$vname(err)
                    }
                }
            )*

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        $(
                            $name::$vname(err) => write!(f, "{}", err),
                        )*
                    }
                }
            }
        )*
    }
}

define_error! {
    pub Error {
        IO(::std::io::Error),
    }
}

impl StdError for Error {}
