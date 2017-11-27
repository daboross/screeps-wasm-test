extern crate arrayvec;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate quote;
extern crate syn;

mod arguments;
mod processing;
mod source_searching;
mod js_generation;

use std::path::Path;
use std::io::{Read, Write};
use std::fs;

use failure::Error;

#[derive(Debug, Fail)]
enum MacroError {
    #[fail(display = "expected function, found invalid item '{:?}'", kind)]
    InvalidItemKind { kind: syn::ItemKind },
    #[fail(display = "expected regular non-self function parameter, found '{:?}'", arg)]
    InvalidArgument { arg: syn::FnArg },
    #[fail(display = "expected one of the known argument types (&[u8], &mut [u8]), found '{:?}",
           ty)]
    UnhandledArgumentType { ty: syn::Ty },
    #[fail(display = "expected macro to contain a single delimited token tree, found \
                      multiple: {:?}",
           tokens)]
    UnexpectedMultiTokenMacro { tokens: Vec<syn::TokenTree> },
    #[fail(display = "expected multiple tokens in js_fn! macro invocation, found single \
                      token: '{:?}'",
           token)]
    UnexpectedSingleToken { token: syn::Token },
    #[fail(display = "expected all complete `fn a(..) => ..;` or `fn a(..) {{ .. }}` \
                      inside js_fn! macro, found incomplete tokens left: {:?}",
           tokens)]
    UnexpectedEndOfMacroInvocation { tokens: quote::Tokens },
    #[fail(display = "failed to parse processed macro invocation: {:?}", err_msg)]
    UnexpectedReparseFailure { err_msg: String },
}

// TODO: full Options struct for options, not just class name.
pub fn translate_files<P, U>(input_lib: P, output_file: U, js_class_name: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
    U: AsRef<Path>,
{
    let contents = {
        let mut handle = fs::File::open(input_lib)?;

        let mut buffer = String::new();

        handle.read_to_string(&mut buffer)?;

        buffer
    };

    let output = generate_js_from_all_macros_in(&contents, js_class_name)?;

    let mut handle = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)?;

    write!(handle, "{}", output)?;

    Ok(())
}

pub fn generate_js_from_all_macros_in(source: &str, js_class_name: &str) -> Result<String, Error> {
    let func_definition_items = source_searching::walk_crate_for_js_fns(source)?;

    let js_fn_infos = func_definition_items
        .into_iter()
        .map(|item| processing::JsFnInfo::try_from(&item))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(js_generation::generate_javascript(
        js_class_name,
        &js_fn_infos,
    )?)
}
