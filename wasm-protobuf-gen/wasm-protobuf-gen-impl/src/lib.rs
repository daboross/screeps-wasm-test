#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate quote;
extern crate syn;

mod arguments;

use failure::{Error, ResultExt};

use arguments::KnownArgumentType;

#[derive(Debug, Fail)]
enum MacroError {
    #[fail(display = "expected function, found invalid item '{:?}'", kind)]
    InvalidItemKind { kind: syn::ItemKind },
    #[fail(display = "expected regular non-self function parameter, found '{:?}'", arg)]
    InvalidArgument { arg: syn::FnArg },
    #[fail(display = "expected one of the known argument types (&[u8], &mut [u8]), found '{:?}",
           ty)]
    UnhandledArgumentType { ty: syn::Ty },
}

proc_macro_item_impl! {
    pub fn __js_fn_impl(input: &str) -> String {
        process_all_functions(input).expect("expected macro to succeed")
    }
}

fn process_all_functions(input: &str) -> Result<String, Error> {
    let ast = syn::parse_items(input).map_err(|e| {
        format_err!("failed to parse macro input as an item: '{}'", e)
    })?;

    let mut full_out = quote::Tokens::new();
    for item in &ast {
        let output = process_item(item).with_context(|e| {
            format!("failed to process function '{:?}': {}", item, e)
        })?;
        // let ast_debug_str = format!("{:?}", item);

        // let ident = &item.ident;

        // let output = quote! (
        //     fn #ident() -> String {
        //         (#ast_debug_str).to_owned()
        //     }
        // );

        full_out.append(output);
    }
    Ok(full_out.to_string())
}

fn process_item(item: &syn::Item) -> Result<quote::Tokens, Error> {
    match item.node {
        syn::ItemKind::Fn(ref decleration, _, _, _, _, ref block) => {
            generate_function_wrapper(item, decleration, block)
        }
        ref kind => Err(MacroError::InvalidItemKind { kind: kind.clone() })?,
    }
}

fn generate_function_wrapper(
    item: &syn::Item,
    decl: &syn::FnDecl,
    code: &syn::Block,
) -> Result<quote::Tokens, Error> {
    let callable_body = generate_callable_body(item, decl, code)?;

    let argument_types = get_argument_types(decl)?;

    let argument_names = (0..argument_types.len())
        .map(|index| syn::Ident::new(format!("__arg{}", index)))
        .collect::<Vec<_>>();

    let mut function_body = quote::Tokens::new();

    for (ty, arg_name) in argument_types.iter().zip(&argument_names) {
        function_body.append(setup_for_argument(&arg_name, ty)?);
    }

    let mut arg_names_as_argument_list = quote::Tokens::new();
    for arg_name in &argument_names {
        arg_names_as_argument_list.append(quote! { #arg_name, });
    }

    function_body.append(quote! {
        // TODO: handle results as well...
        let __result: () = (#callable_body)(#arg_names_as_argument_list);
    });

    let identifier = syn::Ident::new(format!("__jsf_{}", item.ident));

    let mut real_arguments_list = quote::Tokens::new();
    for (ty, arg_name) in argument_types.iter().zip(&argument_names) {
        expand_argument_into(arg_name, ty, &mut real_arguments_list)?;
    }

    let full_definition = quote! {
        extern "C" fn #identifier (#real_arguments_list) {
            #function_body
        }
    };

    // Ok(full_definition)
    let temp_ident = &item.ident;
    let temp_str = full_definition.to_string();
    Ok(quote! {
        fn #temp_ident() -> &'static str {
            #temp_str
        }
    })
}

fn expand_argument_into(
    arg_name: &syn::Ident,
    ty: &syn::Ty,
    tokens: &mut quote::Tokens,
) -> Result<(), Error> {
    let type_type = KnownArgumentType::try_from(ty)?;

    match type_type {
        KnownArgumentType::U8SliceRef => {
            let ptr_arg_name = syn::Ident::from(format!("{}_ptr", arg_name));
            let length_arg_name = syn::Ident::from(format!("{}_len", arg_name));
            tokens.append(quote! {
                #ptr_arg_name: *const u8,
                #length_arg_name: usize,
            });
        }
        KnownArgumentType::U8SliceMutRef => {
            let ptr_arg_name = syn::Ident::from(format!("{}_ptr", arg_name));
            let length_arg_name = syn::Ident::from(format!("{}_len", arg_name));
            tokens.append(quote! {
                #ptr_arg_name: *mut u8,
                #length_arg_name: usize,
            });
        }
    }

    Ok(())
}

fn setup_for_argument(arg_name: &syn::Ident, ty: &syn::Ty) -> Result<quote::Tokens, Error> {
    let type_type = KnownArgumentType::try_from(ty)?;

    let tokens = match type_type {
        KnownArgumentType::U8SliceRef => {
            // TODO: a way to not duplicate this code.
            // (maybe a struct for arg names which implement ToTokens?)
            let ptr_arg_name = syn::Ident::from(format!("{}_ptr", arg_name));
            let length_arg_name = syn::Ident::from(format!("{}_len", arg_name));
            quote! {
                let #arg_name = unsafe {
                    ::std::slice::from_raw_parts(#ptr_arg_name, #length_arg_name)
                };
            }
        }
        KnownArgumentType::U8SliceMutRef => {
            let ptr_arg_name = syn::Ident::from(format!("{}_ptr", arg_name));
            let length_arg_name = syn::Ident::from(format!("{}_len", arg_name));
            quote! {
                let #arg_name = unsafe {
                    ::std::slice::from_raw_parts_mut(#ptr_arg_name, #length_arg_name)
                };
            }
        }
    };

    Ok(tokens)
}

fn get_argument_types(decl: &syn::FnDecl) -> Result<Vec<syn::Ty>, Error> {
    Ok(decl.inputs
        .iter()
        .map(|input| match *input {
            syn::FnArg::SelfRef(_, _) | syn::FnArg::SelfValue(_) | syn::FnArg::Ignored(_) => {
                Err(MacroError::InvalidArgument { arg: input.clone() })
            }
            syn::FnArg::Captured(_, ref ty) => Ok(ty.clone()),
        })
        .collect::<Result<_, _>>()?)
}

fn generate_callable_body(
    _item: &syn::Item,
    decl: &syn::FnDecl,
    code: &syn::Block,
) -> Result<quote::Tokens, Error> {
    // we'll see what works best here.
    // This set of if statements is for if we've been given a path to the implementing function.
    //
    // In this case, we want to just call the function at that path with the same arguments the
    // function declaration takes.
    if let Some(statement) = code.stmts.first() {
        if let syn::Stmt::Expr(ref inner_expr) = *statement {
            if let syn::ExprKind::Path(_, _) = inner_expr.node {
                return Ok(quote! {
                    // output the path alone so that it can be called like (path::to::func)(args)
                    (#inner_expr)
                });
            }
        }
    }

    // if it isn't our special case of a path, we can assume the full code
    // to call the inner function has been written out. We'll give the code
    // then a copy of the inputs and call it
    let mut arguments = quote::Tokens::new();
    for input in &decl.inputs {
        arguments.append(quote! {
            #input,
        });
    }
    Ok(quote! {
        (|#arguments| { #code })
    })
}
