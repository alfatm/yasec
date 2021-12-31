//! Provides a derive macro that implements `Yasec` trait.
//! For complete documentation please see [yasec](https://docs.rs/yasec).

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, NestedMeta};

#[proc_macro_derive(Yasec, attributes(yasec))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_yasec(&derive_input);
    gen.into()
}

fn impl_yasec(input: &DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;
    let struct_name = &input.ident;

    let inner_impl = match input.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => impl_yasec_for_struct(struct_name, &fields.named),
            _ => panic!("yasec supports only named fields"),
        },
        _ => panic!("yasec only supports non-tuple structs"),
    };

    quote!(#inner_impl)
}

fn impl_yasec_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let field_assigns = fields.iter().map(gen_field_assign);
    let usage_assigns = fields.iter().map(gen_field_usage);

    quote! {
        impl Yasec for #struct_name {
            fn with_context(context: ::yasec::Context) -> ::std::result::Result<Self, ::yasec::Error> {
                let config = Self {
                    #(#field_assigns,)*
                };
                Ok(config)
            }

            fn usage_with_context(context: ::yasec::Context) -> ::std::result::Result<Vec< ::yasec::Context>, ::yasec::Error> {
                let output = vec![
                    #(#usage_assigns,)*
                ].into_iter().flatten().collect::<Vec< ::yasec::Context>>();
                Ok(output)
            }
        }
    }
}

fn gen_field_assign(field: &Field) -> proc_macro2::TokenStream {
    match fetch_yasec_attr_from_field(field) {
        Some(attr) => {
            let list = fetch_list_from_attr(field, attr);
            let env_name = find_item_in_list(field, &list, "env");
            let opt_default = find_item_in_list(field, &list, "default");
            gen_field_assign_for_struct_type(field, env_name, opt_default)
        }
        None => gen_field_assign_for_struct_type(field, None, None),
    }
}

// converts Option<T> to Option::<T>
fn norm_path(path: &mut syn::TypePath) {
    path.path.segments.iter_mut().for_each(|segment| {
        let args = &mut segment.arguments;
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
                            .with_default_value(#default)
                    )?
                },
                None => quote! {
                    #ident: #path :: with_context(
                        context
                            .with_var_name(#var_name)
                            .push_prefix(#ident_str.to_owned())
                    )?
                },
            }
        }
        _ => panic!("Expected field type to be a path: {:?}", ident),
    }
}

fn gen_field_usage(field: &Field) -> proc_macro2::TokenStream {
    match fetch_yasec_attr_from_field(field) {
        Some(attr) => {
            let list = fetch_list_from_attr(field, attr);
            let from_value = find_item_in_list(field, &list, "from");
            let opt_default = find_item_in_list(field, &list, "default");
            gen_field_usage_for_struct_type(field, from_value, opt_default)
        }
        None => gen_field_usage_for_struct_type(field, None, None),
    }
}

fn gen_field_usage_for_struct_type(
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
                    #path :: usage_with_context(
                        context
                            .with_var_name(#var_name)
                            .push_prefix(#ident_str.to_owned())
                            .with_default_value(#default)
                    )?
                },
                None => quote! {
                    #path :: usage_with_context(
                        context
                            .with_var_name(#var_name)
                            .push_prefix(#ident_str.to_owned())
                    )?
                },
            }
        }
        _ => panic!("Expected field type to be a path: {:?}", ident),
    }
}

fn fetch_yasec_attr_from_field(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
        let path = &a.path;
        let name = quote!(#path).to_string();
        name == "yasec"
    })
}

fn fetch_list_from_attr(field: &Field, attr: &Attribute) -> Punctuated<NestedMeta, Comma> {
    let opt_meta = attr.parse_meta().unwrap_or_else(|e| {
        panic!(
            "Can not interpret meta of `yasec` attribute on field `{}`, {}",
            field_name(field),
            e
        )
    });

    match opt_meta {
        Meta::List(l) => l.nested,
        _ => panic!(
            "`yasec` attribute on field `{}` must contain a list",
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
                    "`yasec` attribute on field `{}` must contain name/value item",
                    field_name(field)
                ),
            },
            _ => panic!(
                "Failed to process `yasec` attribute on field `{}`",
                field_name(field)
            ),
        })
        .find(|name_value| {
            let ident = &name_value.path;
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
