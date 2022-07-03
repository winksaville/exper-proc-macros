use proc_macro_visit::{visit1, valid_tokenstream};

visit1!(
    fn yo() {
        println!("visit1 macro output");
    }
);

fn main() {
    println!("Hi");
    yo();

    // Example of a "valid token stream"
    //
    // And here are the structs of a proc_macro TokenStream:
    //   https://doc.rust-lang.org/nightly/proc_macro/index.html#structs
    //
    // Here is the what the input TokenStream looks like
    //   valid_tokenstream: input=TokenStream [
    //       Ident {
    //           ident: "struct",
    //           span: #0 bytes(520..526),
    //       },
    //       Ident {
    //           ident: "fn",
    //           span: #0 bytes(527..529),
    //       },
    //       Ident {
    //           ident: "macro",
    //           span: #0 bytes(530..535),
    //       },
    //       Ident {
    //           ident: "a",
    //           span: #0 bytes(536..537),
    //       },
    //       Ident {
    //           ident: "ab",
    //           span: #0 bytes(538..540),
    //       },
    //       Ident {
    //           ident: "abc123",
    //           span: #0 bytes(541..547),
    //       },
    //       Punct {
    //           ch: '\'',
    //           spacing: Joint,
    //           span: #0 bytes(565..570),
    //       },
    //       Ident {
    //           ident: "adef",
    //           span: #0 bytes(565..570),
    //       },
    //       Punct {
    //           ch: '+',
    //           spacing: Alone,
    //           span: #0 bytes(626..627),
    //       },
    //       Punct {
    //           ch: '-',
    //           spacing: Alone,
    //           span: #0 bytes(628..629),
    //       },
    //       Punct {
    //           ch: '/',
    //           spacing: Alone,
    //           span: #0 bytes(630..631),
    //       },
    //       Punct {
    //           ch: '*',
    //           spacing: Alone,
    //           span: #0 bytes(632..633),
    //       },
    //       Literal {
    //           kind: Integer,
    //           symbol: "0x12",
    //           suffix: None,
    //           span: #0 bytes(671..675),
    //       },
    //       Literal {
    //           kind: Integer,
    //           symbol: "123",
    //           suffix: None,
    //           span: #0 bytes(676..679),
    //       },
    //       Literal {
    //           kind: Integer,
    //           symbol: "0001",
    //           suffix: None,
    //           span: #0 bytes(680..684),
    //       },
    //       Literal {
    //           kind: Integer,
    //           symbol: "123",
    //           suffix: Some("def"),
    //           span: #0 bytes(685..691),
    //       },
    //       Literal {
    //           kind: Str,
    //           symbol: "",
    //           suffix: None,
    //           span: #0 bytes(725..727),
    //       },
    //       Punct {
    //           ch: '<',
    //           spacing: Joint,
    //           span: #0 bytes(786..788),
    //       },
    //       Punct {
    //           ch: '<',
    //           spacing: Alone,
    //           span: #0 bytes(786..788),
    //       },
    //       Punct {
    //           ch: '$',
    //           spacing: Joint,
    //           span: #0 bytes(789..790),
    //       },
    //       Punct {
    //           ch: '*',
    //           spacing: Alone,
    //           span: #0 bytes(790..791),
    //       },
    //       Punct {
    //           ch: '#',
    //           spacing: Alone,
    //           span: #0 bytes(865..866),
    //       },
    //       Punct {
    //           ch: '>',
    //           spacing: Alone,
    //           span: #0 bytes(867..868),
    //       },
    //       Group {
    //           delimiter: Parenthesis,
    //           stream: TokenStream [
    //               Group {
    //                   delimiter: Brace,
    //                   stream: TokenStream [
    //                       Ident {
    //                           ident: "world",
    //                           span: #0 bytes(914..919),
    //                       },
    //                   ],
    //                   span: #0 bytes(912..921),
    //               },
    //           ],
    //           span: #0 bytes(910..923),
    //       },
    //   ]
    valid_tokenstream!(
        // comment are not in tokenstream
        /* slash asteric
           comment are not in tokenstream
           asteric slash */
        struct fn macro a ab abc123 // Ident
        'adef // Punct with spacing: Joint followed by Ident
        + - / * // Punct with spacing: Alone
        0x12 123 0001 123def // Literal kind: Integer
        "" // Literal kind Str (double quote must be paired)
        << $* // Both are Punct spacing: Joint followed by Punc spacing: Alone
        # > // Both are Punct spacing: Alone
        ( { world } ) // Group delimiter: Parenthesis with stream: TokenStream array
                      // with one item a Group with delimiter: Brace with a stream: TokenStream array
                      // with one item an Ident
    );

    visit1!(
        println!("main output");
    );

}

#[cfg(test)]
mod tests {
    //use proc_macro_fsm1::{fsm1, fsm1_state};

    #[test]
    fn test_x() {
    }
}
