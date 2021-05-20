extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

#[proc_macro_derive(OrderEventVisitee)]
pub fn order_event_visitee_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_order_event_visitee(&ast)
}

fn impl_order_event_visitee(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    let re = regex::Regex::new(r"[A-Z][a-z]*").unwrap();
    let name_caps: Vec<_> = re
        .captures_iter(&name_str)
        .map(|m| m.get(0).unwrap().as_str())
        .collect();
    let context = name_caps[0];
    let verb = name_caps[1];
    let input_event_enum_name = format_ident!("{}OrderEvent", context);
    let func_name = format_ident!("visit_{}_order", verb.to_lowercase());
    let verb = format_ident!("{}", verb);

    let gen = quote! {
        impl OrderEventVisitee<#input_event_enum_name> for #name {
            fn accept<V>(&self, v: &V) -> Option<V::Output>
                where
                    V: OrderEventVisitor<#input_event_enum_name>,
            {
                v.#func_name(&#input_event_enum_name::#verb(self.clone()))
            }
        }
    };
    //eprintln!("{:?}", gen.to_string());
    gen.into()
}
