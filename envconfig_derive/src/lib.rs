//! Provides a derive macro that implements `Envconfig` trait.
//! For complete documentation please see [envconfig](https://docs.rs/envconfig).

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, NestedMeta};

#[proc_macro_derive(Envconfig, attributes(envconfig))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_envconfig(&derive_input);
    gen.into()
}

fn impl_envconfig(input: &DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;
    let struct_name = &input.ident;

    let inner_impl = match input.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => impl_envconfig_for_struct(struct_name, &fields.named),
            _ => panic!("envconfig supports only named fields"),
        },
        _ => panic!("envconfig only supports non-tuple structs"),
    };

    quote!(#inner_impl)
}

fn impl_envconfig_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let field_assigns = fields.iter().map(gen_field_assign);

    quote! {
        impl Envconfig for #struct_name {
            fn with_context(context: ::envconfig::Context<Self>) -> ::std::result::Result<Self, ::envconfig::Error> {
                let config = Self {
                    #(#field_assigns,)*
                };
                Ok(config)
            }
        }
    }
}

fn gen_field_assign(field: &Field) -> proc_macro2::TokenStream {
    match fetch_envconfig_attr_from_field(field) {
        Some(attr) => {
            let list = fetch_list_from_attr(field, attr);
            let from_value = find_item_in_list(field, &list, "from");
            let opt_default = find_item_in_list(field, &list, "default");
            gen_field_assign_for_struct_type(field, from_value, opt_default)
        }
        None => gen_field_assign_for_struct_type(field, None, None),
    }
}

// converts Option<T> to Option::<T>
fn norm_path(path: &mut syn::TypePath) {
    path.path.segments.iter_mut().for_each(|segment| {
        let ref mut args = segment.arguments;
        if args.is_empty() {
            return;
        }

        match args {
            syn::PathArguments::AngleBracketed(ref mut x) if x.colon2_token.is_none() => {
                x.colon2_token = Some(syn::Token!(::)([
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ]))
            }
            _ => (),
        }
    });
}

fn remove_quotes(val: &str) -> String {
    val.chars().skip(1).take(val.len() - 2).collect()
}

fn gen_field_assign_for_struct_type(
    field: &Field,
    name: Option<&Lit>,
    default: Option<&Lit>,
) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let var_name = name.map(|x| remove_quotes(&to_s(x))).unwrap_or_default();
    let ident_str = to_s(ident).to_uppercase();
    match &field.ty {
        syn::Type::Path(path) => {
            let mut path = path.clone();
            norm_path(&mut path);
            match default {
                Some(_) => quote! {
                    #ident: #path :: with_context(
                        context
                            .with_var_name(#var_name)
                            .push_prefix(#ident_str.to_owned())
                            .with_default_var_value(Some(#default))
                    )?
                },
                None => quote! {
                    #ident: #path :: with_context(
                        context
                            .with_var_name(#var_name)
                            .push_prefix(#ident_str.to_owned())
                            .with_default_var_value(None)
                    )?
                },
            }
        }
        _ => panic!(format!("Expected field type to be a path: {:?}", ident)),
    }
}

fn fetch_envconfig_attr_from_field(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
        let path = &a.path;
        let name = quote!(#path).to_string();
        name == "envconfig"
    })
}

fn fetch_list_from_attr(field: &Field, attr: &Attribute) -> Punctuated<NestedMeta, Comma> {
    let opt_meta = attr.interpret_meta().unwrap_or_else(|| {
        panic!(
            "Can not interpret meta of `envconfig` attribute on field `{}`",
            field_name(field)
        )
    });

    match opt_meta {
        Meta::List(l) => l.nested,
        _ => panic!(
            "`envconfig` attribute on field `{}` must contain a list",
            field_name(field)
        ),
    }
}

fn find_item_in_list<'l, 'n>(
    field: &Field,
    list: &'l Punctuated<NestedMeta, Comma>,
    item_name: &'n str,
) -> Option<&'l Lit> {
    list.iter()
        .map(|item| match item {
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(name_value) => name_value,
                _ => panic!(
                    "`envconfig` attribute on field `{}` must contain name/value item",
                    field_name(field)
                ),
            },
            _ => panic!(
                "Failed to process `envconfig` attribute on field `{}`",
                field_name(field)
            ),
        })
        .find(|name_value| {
            let ident = &name_value.ident;
            let name = quote!(#ident).to_string();
            name == item_name
        })
        .map(|item| &item.lit)
}

fn field_name(field: &Field) -> String {
    to_s(&field.ident)
}

fn to_s<T: quote::ToTokens>(node: &T) -> String {
    quote!(#node).to_string()
}
