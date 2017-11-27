use failure::Error;

use {quote, syn};

use MacroError;

pub fn walk_crate_for_js_fns(source: &str) -> Result<Vec<syn::Item>, Error> {
    use syn::visit::Visitor;

    let ast = syn::parse_crate(source).map_err(|e| {
        format_err!("failed to parse macro input as an item: {}", e)
    })?;

    let mut v = FindMacrosVisitor::find_js_fn();

    v.visit_crate(&ast);

    // flat_map doesn't work well with Result<Vec<T>, E>.
    let mut func_definition_items = Vec::new();

    for found_macro in v.found {
        if found_macro.tts.len() > 1 {
            return Err(MacroError::UnexpectedMultiTokenMacro {
                tokens: found_macro.tts,
            })?;
        }
        match found_macro.tts.into_iter().next() {
            Some(tt) => func_definition_items.extend(macro_tt_to_fns(tt)?),
            None => return Err(MacroError::UnexpectedMultiTokenMacro { tokens: Vec::new() })?,
        }
    }

    Ok(func_definition_items)
}

struct FindMacrosVisitor {
    ident_to_find: syn::Path,
    found: Vec<syn::Mac>,
}

impl FindMacrosVisitor {
    fn new(ident_to_find: syn::Path) -> Self {
        FindMacrosVisitor {
            ident_to_find: ident_to_find,
            found: Vec::new(),
        }
    }

    fn find_js_fn() -> Self {
        FindMacrosVisitor::new(syn::Path {
            global: false,
            segments: vec![
                syn::PathSegment {
                    ident: syn::Ident::new("js_fn"),
                    parameters: syn::PathParameters::none(),
                },
            ],
        })
    }
}

impl syn::visit::Visitor for FindMacrosVisitor {
    fn visit_mac(&mut self, mac: &syn::Mac) {
        // TODO: can macros ever have global paths? This would break if that's
        // the case. Right now we require an exact match on non-global 'js_fn!',
        // if there could be another way to invoke it, we might want to use
        // some fuzzy matching.
        if mac.path == self.ident_to_find {
            self.found.push(mac.clone());
        }
    }
}

fn macro_tt_to_fns(source: syn::TokenTree) -> Result<Vec<syn::Item>, Error> {
    use syn::TokenTree::*;
    use syn::DelimToken;
    use quote::ToTokens;

    match source {
        Delimited(tt) => {
            let mut found_full = Vec::new();

            let mut so_far = quote::Tokens::new();
            let mut iter = tt.tts.into_iter();
            while let Some(token_tree) = iter.next() {
                match token_tree {
                    // This matches a definition like:
                    // ```
                    // fn a() => modname::funcname;
                    // ```
                    Token(syn::Token::FatArrow) => {
                        // gather all tokens until ';'
                        let mut inner_tokens = Vec::new();
                        while let Some(inner_token) = iter.next() {
                            match inner_token {
                                Token(syn::Token::Semi) => break,
                                _ => inner_tokens.push(inner_token),
                            }
                        }
                        Delimited(syn::Delimited {
                            delim: DelimToken::Brace,
                            tts: inner_tokens,
                        }).to_tokens(&mut so_far);
                        found_full.push(so_far);
                        so_far = quote::Tokens::new();
                    }
                    // This matches a definition like:
                    // ```
                    // fn a() {
                    //     // inline code
                    // }
                    // ```
                    Delimited(syn::Delimited {
                        delim: DelimToken::Brace,
                        ..
                    }) => {
                        token_tree.to_tokens(&mut so_far);
                        found_full.push(so_far);
                        so_far = quote::Tokens::new();
                    }
                    ref other => other.to_tokens(&mut so_far),
                }
            }

            if !so_far.as_ref().is_empty() {
                return Err(MacroError::UnexpectedEndOfMacroInvocation {
                    tokens: so_far,
                })?;
            }

            Ok(found_full
                .into_iter()
                .map(|found| {
                    syn::parse_item(found.as_ref()).map_err(|desc| {
                        MacroError::UnexpectedReparseFailure { err_msg: desc }
                    })
                })
                .collect::<Result<Vec<syn::Item>, MacroError>>()?)
        }
        Token(ref t) => Err(MacroError::UnexpectedSingleToken { token: t.clone() })?,
    }
}
