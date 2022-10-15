use proc_macro::{TokenStream, TokenTree};

/// Interpolates the string using any user defined interpolation expressions inside.
///
/// Front and back sequence of characters of interpolation expression can be passed as a parameters.
/// By defalt they are "#{" for front sequence and "}" for back sequence (both without quotes).
///
/// Syntax:
///     inject!( $args ) | inject!{ $args } | inject!\[ $args \]
///
/// $args:
///     $(front: $front:literal,)?
///     $(back: $back:literal,)?
///     $literal_for_interpolation:literal
#[proc_macro]
pub fn inject(input: TokenStream) -> TokenStream {
    fn inject_str(input: &str, front: &str, back: &str) -> String {
        let (fstr, fargs) = input
            .split(front)
            .flat_map(|s| {
                let parts: Vec<&str> = s.split_terminator(back).collect();
                let (last, inner) = parts.split_last().unwrap();
                [inner.join(back), last.to_string()].into_iter()
            })
            .filter(|s| !s.is_empty())
            .enumerate()
            .fold((String::new(), String::new()), |(fstr, fargs), (i, x)| {
                if i % 2 == 0 {
                    (format!("{fstr}{x}"), fargs)
                } else {
                    (format!("{fstr}{{}}"), format!("{fargs}, {x}"))
                }
            });

        format!(r#"format!({fstr}{fargs})"#)
    }

    fn unquote_lit(lit: &str) -> String {
        lit.split('"').nth(1).unwrap().to_string()
    }

    let mut it = input.into_iter();
    let mut front = Some("#{".to_string()); // default front of interpolation expr is "#{"
    let mut back = Some("}".to_string()); // default back of interpolation expr is "}"
    for _ in 0..3 {
        match it.next().unwrap() {
            TokenTree::Ident(i) if i.to_string() == "front" => {
                match it.next().unwrap() {
                    TokenTree::Punct(p) if p.to_string() == ":" => {
                        front = unquote_lit(&it.next().unwrap().to_string()).into();
                    }
                    _ => panic!("unexpected arg"),
                }
                assert_eq!(it.next().unwrap().to_string(), ",", "missing comma");
            }
            TokenTree::Ident(i) if i.to_string() == "back" => {
                match it.next().unwrap() {
                    TokenTree::Punct(p) if p.to_string() == ":" => {
                        back = unquote_lit(&it.next().unwrap().to_string()).into();
                    }
                    _ => panic!("unexpected arg"),
                }
                assert_eq!(it.next().unwrap().to_string(), ",", "missing comma");
            }
            TokenTree::Literal(l) => {
                let input = l.to_string();
                return inject_str(&input, &front.unwrap(), &back.unwrap())
                    .parse()
                    .unwrap();
            }
            _ => panic!("unexpected arg"),
        }
    }

    panic!("literal string not found");
}
