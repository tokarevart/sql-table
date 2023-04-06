use proc_macro::{TokenStream, TokenTree};

/// Interpolates the string using any user defined interpolation expressions inside.
/// The only argument is the interpolation string literal.
///
/// Opening and closing sequences of characters of interpolation expressions
/// are `#{` and `}#` respectively.
#[proc_macro]
pub fn inject(input: TokenStream) -> TokenStream {
    let mut it = input.into_iter();
    let input = if let Some(TokenTree::Literal(l)) = it.next() {
        if it.next().is_some() {
            panic!("only one argument is expected")
        }
        l.to_string()
    } else {
        panic!("&str is expected")
    };

    let (front, back) = ("#{", "}#");

    let mut v = Vec::new();

    let (fstr, fargs) = input
        .split(front)
        .flat_map(|s| {
            let parts: Vec<&str> = s.split(back).collect();
            let (last, inner) = parts.split_last().unwrap();
            v.push((inner.join(back), last.to_string()));
            [inner.join(back), last.to_string()].into_iter()
        })
        .skip(1)
        .enumerate()
        .fold((String::new(), String::new()), |(fstr, fargs), (i, x)| {
            if i % 2 == 0 {
                (format!("{fstr}{x}"), fargs)
            } else {
                (format!("{fstr}{{}}"), format!("{fargs}, {x}"))
            }
        });

    // panic!("{:?}", v);
    // panic!("{}", format!(r#"format!({fstr}{fargs})"#));
    format!(r#"format!({fstr}{fargs})"#).parse().unwrap()
}
