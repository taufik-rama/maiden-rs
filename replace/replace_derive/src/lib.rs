use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Replace)]
pub fn replace_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_replace(&ast)
}

fn impl_replace(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let is_ty_option = |f: &syn::Field| {
        if let syn::Type::Path(ref p) = f.ty {
            return p.path.segments.len() == 1 && p.path.segments[0].ident == "Option";
        }
        false
    };

    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        syn::Data::Enum(_) => panic!("can only derive struct"),
        syn::Data::Union(_) => panic!("can only derive struct"),
    };
    let fields = fields.iter().map(|field| {
        let name = &field.ident;
        println!("{:#?}", field);
        if is_ty_option(field) {
            quote! {
                if other.#name.is_some() {
                    self.#name = other.#name;
                }
            }
        } else {
            quote! {
                self.#name = other.#name
            }
        }
    });

    let gen = quote! {
        impl Replace for #name {
            fn replace_with(&mut self, other: Self) {
                #(#fields;)*
            }
        }
    };
    gen.into()
}
