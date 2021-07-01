#![feature(
    format_args_capture,
    async_closure,
    map_first_last,
    box_patterns,
    error_iter,
    try_blocks
)]
#![allow(type_alias_bounds)]
#![deny(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

extern crate humantime_serde;

#[macro_use]
pub mod error;
pub use error::Error;

#[macro_use]
pub mod schema;
pub use schema::{Content, Name, Namespace};

pub mod graph;
pub use graph::Graph;

pub mod compile;
pub use compile::{Compile, Compiler};

#[test]
fn create_koto_config() -> Result<(), Box<dyn std::error::Error>> {
    let koto = koto::Koto::default();
    let prelude = koto.prelude();
    bindlang_init(&mut prelude);
    let script = "Namespace::default()";
    let compiled = koto.compile(script)?;
    let result_value = koto.run_chunk(compiled)?;
    let namespace: Namespace = <Namespace as lang_bindings::FromValue>::from_value(&lang_bindings::KeyPath::Index(0, None), &result_value)?;
    Ok(())
}

bindlang::bindlang_main! {
    use crate::graph::{string::Locale, Value};
    use crate::schema::{ArrayContent, BoolContent, FieldContent, NumberContent, ObjectContent, optional, required};
}
