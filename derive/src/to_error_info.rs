use darling::ast::{Data, Fields, Style};
use darling::{util, FromDeriveInput, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_info))]
struct ToErrorInfoEnum {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<ToErrorInfoVariants, ()>,
    ty: syn::Type,
    prefix: String,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(error_info))]
struct ToErrorInfoVariants {
    ident: syn::Ident,
    fields: Fields<util::Ignored>,
    code: String,
    inner_code: String,
    client_msg: Option<String>,
}

pub(crate) fn process_to_error_info(input: DeriveInput) -> TokenStream {
    let ToErrorInfoEnum {
        ident: enum_ident,
        generics,
        data: Data::Enum(data),
        ty,
        prefix,
    } = ToErrorInfoEnum::from_derive_input(&input).expect("Can not parse input.")
    else {
        panic!("ToError only works on enum.")
    };

    // for each variant, generate a match arm
    // #name::#ident(_) => {
    //     code to new error info
    // }
    let variant_impls = data
        .iter()
        .map(|variant| {
            let ToErrorInfoVariants {
                ident,
                fields,
                code,
                inner_code,
                client_msg,
            } = variant;

            let inner_code = format!("{}{}", prefix, inner_code);
            let client_msg = client_msg.clone().unwrap_or_default();
            let style_impls = match fields.style {
                Style::Struct => quote! { #enum_ident::#ident { .. } },
                Style::Tuple => quote! { #enum_ident::#ident(_) },
                Style::Unit => quote! { #enum_ident::#ident },
            };

            quote! {
                #style_impls => {
                    ErrorInfo::new(
                        #code,
                        #inner_code,
                        #client_msg,
                        self,
                    )
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        use code::{ErrorInfo, ToErrorInfo as _};

        impl #generics ToErrorInfo for #enum_ident #generics {
            type T = #ty;

            fn to_error_info(&self) -> ErrorInfo<Self::T> {
                match self {
                    #(#variant_impls),*
                }
            }
        }
    }
}
