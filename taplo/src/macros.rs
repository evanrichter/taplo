macro_rules! dom_node_from {
    ($($inner:ty => $name:ident),*) => {
        $(
            impl From<$inner> for Node {
                fn from(inner: $inner) -> Self {
                    Node::$name(inner)
                }
            }
        )*
    };
}

macro_rules! dom_primitives {
    ($($($kind:ident)|* => $ast:ident),*) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct $ast(SyntaxToken);
            impl Cast for $ast {
                #[allow(unused)]
                fn cast(elem: SyntaxElement) -> Option<Self> {
                    match elem {
                        rowan::NodeOrToken::Token(t) => {
                            match t.kind() {
                                $($kind)|* => {
                                    Some(Self(t))
                                }
                                _ => {
                                    None
                                }
                            }
                        },
                        _ => {
                            None
                        }
                    }
                }
            }

            impl $ast {
                pub fn text_range(&self) -> TextRange {
                    self.0.text_range()
                }

                pub fn kind(&self) -> SyntaxKind {
                    self.0.kind()
                }
            }

            impl core::fmt::Display for $ast {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
                }
            }
        )*
    };
}

macro_rules! dom_common {
    ($($ast:ident),*) => {
        $(
            impl $ast {
                pub fn text_range(&self) -> TextRange {
                    self.syntax.text_range()
                }

                pub fn kind(&self) -> SyntaxKind {
                    self.syntax.kind()
                }
            }

            impl core::fmt::Display for $ast {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                   self.syntax.fmt(f)
                }
            }
        )*
    };
}
