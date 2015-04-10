#![crate_type = "dylib"]
#![feature(plugin_registrar, quote)]
#![feature(slice_patterns)]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate maud;

use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use rustc::plugin::Registry;

mod parse;
mod render;

fn expand_html<'cx>(cx: &'cx mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'cx> {
    MacEager::expr(parse::parse(cx, args, sp))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("html", expand_html);
}
