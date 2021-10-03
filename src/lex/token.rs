macro_rules! tokens {
    (
        $($(#[$($docs_meta:meta)+])* pub struct $token_name:ident;)*
    ) => {
        // token structs
        $(
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        $(#[$($docs_meta)+])*
        pub struct $token_name<'a> {
            pub(super) inner: std::borrow::Cow<'a, str>,
            pub(super) span: crate::Span,
        })*

        /// Language tokens.
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        pub enum Token<'a> {
            $($(#[$($docs_meta)+])* $token_name ($token_name<'a>),)*
        }

        // span trait
        $(impl crate::Spanned for $token_name<'_> {})*
        impl crate::Spanned for Token<'_> {}
    }
}

tokens! {
    // literals, identifiers, and mist

    /// `EOF`
    pub struct EOF;
    pub struct Identifier;
    pub struct Number;
    /// `"Hello, world!"`
    pub struct Str;

    // keywords

    /// `addr`
    pub struct Addr;
    /// `break`
    pub struct Break;
    /// `const`
    pub struct Const;
    /// `continue`
    pub struct Continue;
    /// `deref`
    pub struct Deref;
    /// `else`
    pub struct Else;
    /// `if`
    pub struct If;
    /// `let`
    pub struct Let;
    /// `loop`
    pub struct Loop;
    /// `ptr`
    pub struct Ptr;
    /// `static`
    pub struct Static;
    /// `struct`
    pub struct Struct;
    /// `union`
    pub struct Union;
    /// `while`
    pub struct While;

    // two chars

    /// `&=`
    pub struct AndEquals;
    /// `::`
    pub struct ColonColon;
    /// `==`
    pub struct EqualsEquals;
    /// `/=`
    pub struct ForwardSlashEquals;
    /// `>=`
    pub struct GreaterEqualsThan;
    /// `<=`
    pub struct LessEqualsThan;
    /// `-=`
    pub struct MinusEquals;
    /// `~=`
    pub struct NotEquals;
    /// `|=`
    pub struct OrEquals;
    /// `+=`
    pub struct PlusEquals;
    /// `*=`
    pub struct StarEquals;
    /// `^=`
    pub struct XorEquals;

    // one char

    /// `&`
    pub  struct And;
    /// `@`
    pub struct At;
    /// `:`
    pub struct Colon;
    /// `{`
    pub struct CurlyLeft;
    /// `}`
    pub struct CurlyRight;
    /// `=`
    pub struct Equals;
    /// `/`
    pub struct ForwardSlash;
    /// `>`
    pub struct GreaterThan;
    /// `<`
    pub struct LessThan;
    /// `-`
    pub struct Minus;
    /// `~`
    pub struct Not;
    /// `|`
    pub struct Or;
    /// `(`
    pub struct ParLeft;
    /// `)`
    pub struct ParRight;
    /// `+`
    pub struct Plus;
    /// `;`
    pub struct SemiColon;
    /// `[`
    pub struct SquareLeft;
    /// `]`
    pub struct SquareRight;
    /// `*`
    pub struct Star;
    /// `^`
    pub  struct Xor;
}
