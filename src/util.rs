use pest::iterators::{Pair, Pairs};

use crate::pest::Rule;

#[macro_export]
macro_rules! enum_token {
    (
        $(#[$m:meta])*
        pub enum $token:ident {
            $(
                $(#[$doc:meta])*
                $variant:ident = $name:literal,
            )*
        }
    ) => {
        $(#[$m])*
        pub enum $token {
            $($(#[$doc])* $variant),*
        }

        impl $token {
            #[inline]
            pub const fn variants() -> &'static [Self] {
                &[ $(Self::$variant,)* ]
            }

            #[inline]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }

            #[inline]
            pub fn from_name(s: &str) -> Option<Self> {
                match s {
                    $($name => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

pub trait PairsExt<'i> {
    fn next_if_rule(&mut self, rule: Rule) -> Option<Pair<'i, Rule>>;
}

impl<'i> PairsExt<'i> for Pairs<'i, Rule> {
    fn next_if_rule(&mut self, rule: Rule) -> Option<Pair<'i, Rule>> {
        let next = self.peek()?;
        if next.as_rule() == rule {
            self.next()
        } else {
            None
        }
    }
}
