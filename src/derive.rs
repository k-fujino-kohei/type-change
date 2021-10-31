use quote::quote;
use syn::Attribute;

pub fn derive(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let arg_name = syn::Ident::new("pre_state", proc_macro2::Span::call_site());
    let (from_type, inits) = match ast.data {
        syn::Data::Struct(ref s) => match s.fields {
            syn::Fields::Named(ref named) => {
                let from_type = parse_attributes(&ast.attrs).0;
                let fields: Vec<_> = named
                    .named
                    .iter()
                    .map(|f| {
                        let field = f.ident.clone().expect("unexpected field");
                        quote!(#field: #arg_name.#field.into())
                    })
                    .collect();
                let inits = quote!({ #(#fields), * });
                (from_type, inits)
            }
            syn::Fields::Unnamed(ref unnamed) => {
                let types: Vec<_> = unnamed.unnamed.iter().map(|f| f.ty.clone()).collect();
                if types.len() > 1 {
                    panic!("tuple length is overed")
                }
                let from_type = types.first().unwrap().clone();
                let inits = quote!((#arg_name));
                (from_type, inits)
            }
            syn::Fields::Unit => unimplemented!(),
        },
        _ => panic!("support for `struct` only."),
    };

    quote!(
        impl From<#from_type> for #name {
            fn from(#arg_name: #from_type) -> #name {
                #name #inits
            }
        }
    )
}

fn parse_attributes(attributes: &[Attribute]) -> FromAttributes {
    let mut attrs: Vec<syn::Result<FromAttributes>> =
        attributes.iter().map(|attr| attr.parse_args()).collect();
    match attrs.pop() {
        Some(Ok(attr)) => attr,
        Some(Err(err)) => panic!("{}", err),
        _ => panic!("must be specified attribute. for example `#[from(State1)]`"),
    }
}

struct FromAttributes(syn::Type);
impl syn::parse::Parse for FromAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<FromAttributes> {
        let ty = input.parse()?;
        Ok(FromAttributes(ty))
    }
}
