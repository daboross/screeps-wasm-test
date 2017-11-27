use std::fmt;

use {arrayvec, quote, syn};

use failure::{Error, ResultExt};

use MacroError;

use arguments::KnownArgumentType;

#[derive(Debug, Clone)]
/// Small constructed ident struct supporting up to four suffixes.
struct ConstructedArgIdent {
    base: &'static str,
    number_suffix: u32,
    suffixes: arrayvec::ArrayVec<[&'static str; 4]>,
}

impl ConstructedArgIdent {
    fn new(base: &'static str, number_suffix: u32) -> Self {
        ConstructedArgIdent {
            base,
            number_suffix,
            suffixes: arrayvec::ArrayVec::new(),
        }
    }

    fn with_suffix(&self, suffix: &'static str) -> Self {
        let mut cloned = self.clone(); // this is a cheap copy
        cloned.suffixes.push(suffix);
        cloned
    }
}

impl quote::ToTokens for ConstructedArgIdent {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        let mut ident = format!("{}{}", self.base, self.number_suffix);
        for suffix in &self.suffixes {
            ident.push_str(suffix);
        }
        tokens.append(ident);
    }
}

#[derive(Debug, Copy, Clone)]
struct ConstructedFuncIdent<T> {
    base: &'static str,
    name: T,
}

impl<T> ConstructedFuncIdent<T> {
    fn new(base: &'static str, name: T) -> ConstructedFuncIdent<T> {
        ConstructedFuncIdent {
            base: base,
            name: name,
        }
    }
}

impl<T: fmt::Display> quote::ToTokens for ConstructedFuncIdent<T> {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        tokens.append(format!("{}{}", self.base, self.name));
    }
}

fn process_all_functions(input: &str) -> Result<String, Error> {
    let ast = syn::parse_items(input).map_err(|e| {
        format_err!("failed to parse macro input as an item: {}", e)
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

    let argument_names = (0..argument_types.len() as u32)
        .map(|index| ConstructedArgIdent::new("__arg", index))
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

    let func_ident = ConstructedFuncIdent::new("__js_fn_", &item.ident);

    let mut real_arguments_list = quote::Tokens::new();
    for (ty, arg_name) in argument_types.iter().zip(&argument_names) {
        expand_argument_into(arg_name, ty, &mut real_arguments_list)?;
    }

    let full_definition = quote! {
        extern "C" fn #func_ident (#real_arguments_list) {
            #function_body
        }
    };

    Ok(full_definition)
    // let temp_ident = &item.ident;
    // let temp_str = full_definition.to_string();
    // Ok(quote! {
    //     fn #temp_ident() -> &'static str {
    //         #temp_str
    //     }
    // })
}

fn expand_argument_into(
    arg_name: &ConstructedArgIdent,
    ty: &syn::Ty,
    tokens: &mut quote::Tokens,
) -> Result<(), Error> {
    let type_type = KnownArgumentType::try_from(ty)?;

    match type_type {
        KnownArgumentType::U8SliceRef => {
            let ptr_arg_name = arg_name.with_suffix("_ptr");
            let length_arg_name = arg_name.with_suffix("_len");
            tokens.append(quote! {
                #ptr_arg_name: *const u8,
                #length_arg_name: usize,
            });
        }
        KnownArgumentType::U8SliceMutRef => {
            let ptr_arg_name = arg_name.with_suffix("_ptr");
            let length_arg_name = arg_name.with_suffix("_len");
            tokens.append(quote! {
                #ptr_arg_name: *mut u8,
                #length_arg_name: usize,
            });
        }
    }

    Ok(())
}

fn setup_for_argument(
    arg_name: &ConstructedArgIdent,
    ty: &syn::Ty,
) -> Result<quote::Tokens, Error> {
    let type_type = KnownArgumentType::try_from(ty)?;

    let tokens = match type_type {
        KnownArgumentType::U8SliceRef => {
            // TODO: coordinate _ptr / _len suffixes
            let ptr_arg_name = arg_name.with_suffix("_ptr");
            let length_arg_name = arg_name.with_suffix("_len");
            quote! {
                let #arg_name = unsafe {
                    ::std::slice::from_raw_parts(#ptr_arg_name, #length_arg_name)
                };
            }
        }
        KnownArgumentType::U8SliceMutRef => {
            let ptr_arg_name = arg_name.with_suffix("_ptr");
            let length_arg_name = arg_name.with_suffix("_len");
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
            syn::FnArg::SelfRef(_, _) | syn::FnArg::SelfValue(_) => {
                Err(MacroError::InvalidArgument { arg: input.clone() })
            }
            syn::FnArg::Captured(_, ref ty) | syn::FnArg::Ignored(ref ty) => Ok(ty.clone()),
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
        // syn::Block ToTokens includes '{}' always already.
        (|#arguments| #code )
    })
}
