#[cfg(test)]
pub(crate) fn assert_ast_eq(actual: proc_macro2::TokenStream, expected: proc_macro2::TokenStream) {
    let expected_tokens = expected.to_string();
    let actual_tokens = actual.to_string();

    pretty_assertions::assert_eq!(
        expected_tokens,
        actual_tokens,
        "\n\nActual:\n {}",
        pretty_print_item(actual)
    );
}

#[cfg(test)]
fn pretty_print_item(item: proc_macro2::TokenStream) -> String {
    // Maybe just if it actually fails?
    let mod_wrap = quote::quote! {
        mod pp {
            #item
        }
    };

    let as_string = mod_wrap.to_string();

    let item = match syn::parse2(mod_wrap) {
        Ok(item) => item,
        Err(err) => return format!("unable to parse AST (due to {}): {}", err, as_string),
    };

    let file = syn::File {
        attrs: vec![],
        items: vec![item],
        shebang: None,
    };

    prettyplease::unparse(&file)
}
