#[proc_macro]
pub fn inject(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();
    let mut format_str = String::new();
    let mut format_args = String::new();

    let mut depth = 0usize;
    let mut entering_inner = false;
    let mut inner = false;

    let mut it = input.chars().peekable();
    while let Some((c, next)) = it.next().map(|c| (c, it.peek().copied())) {
        if let Some(next) = next {
            if c == '#' && next == '{' {
                entering_inner = true;
                continue;
            }
        }

        if c == '{' && (inner || entering_inner) {
            depth += 1;
        }

        if c != '{' && entering_inner {
            inner = true;
            entering_inner = false;
        }

        if c == '}' && inner {
            depth -= 1;
            if depth == 0 {
                inner = false;
                format_args.push_str(", ");
            }
        }

        if inner {
            format_args.push(c);
        } else {
            format_str.push(c);
        }
    }

    format!(
        r#"format!({})"#,
        if format_args.is_empty() {
            format_str
        } else {
            format!("{format_str}, {format_args}")
        }
    )
    .parse()
    .unwrap()
}
