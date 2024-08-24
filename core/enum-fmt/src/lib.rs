
#[macro_export]
macro_rules! str_able {
    (
        $(#[$attr:meta])*
        $vis:vis enum $e_ident:ident {
            $($v_ident:ident),* $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $e_ident {
            $($v_ident),*
        }

        impl $e_ident {
            #[inline]
            $vis const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$v_ident => stringify!($v_ident)),*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    str_able! {
        pub enum Example {
            One,
            Two,
            Three
        }
    }

    #[test]
    fn smoke() {
        assert_eq!(Example::One.as_str(), "One");
        assert_eq!(Example::Two.as_str(), "Two");
        assert_eq!(Example::Three.as_str(), "Three");
    }
}
