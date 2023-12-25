use darling::ast;
// use darling::util;
use darling::Error;
use darling::FromDeriveInput;
use darling::FromField;
use darling::FromMeta;
use darling::FromVariant;

use darling::export::syn;

#[proc_macro_derive(Tlv, attributes(tlv))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive2(input.into()).into()
}

fn derive2(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: syn::DeriveInput = match syn::parse2(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    let tlv = match TlvDerive::from_derive_input(&ast) {
        Ok(tlv) => tlv,
        Err(err) => return err.write_errors(),
    };

    quote::quote!(#tlv)
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tlv))]
struct TlvDerive {
    ident: syn::Ident,
    data: ast::Data<PayloadVariant, PayloadItem>,
    r#type: syn::Path,
    error: syn::Path,
    #[darling(default)]
    crates: Crates,
}

#[derive(Debug, FromVariant)]
struct PayloadVariant {
    ident: syn::Ident,
    fields: ast::Fields<PayloadItem>,
}

#[derive(Debug, FromField)]
struct PayloadItem {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(Debug, FromMeta)]
struct Crates {
    #[darling(default = "Self::default_tlv_core")]
    tlv_core: syn::Path,
    #[darling(default = "Self::default_bytes")]
    bytes: syn::Path,
}

impl Default for Crates {
    fn default() -> Self {
        Self {
            tlv_core: Self::default_tlv_core(),
            bytes: Self::default_bytes(),
        }
    }
}

impl Crates {
    fn default_tlv_core() -> syn::Path {
        syn::parse_quote!(::ndn_tlv_core)
    }

    fn default_bytes() -> syn::Path {
        syn::parse_quote!(::bytes)
    }
}

impl quote::ToTokens for TlvDerive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ident: name,
            data,
            r#type,
            error,
            crates: Crates {
                tlv_core: tlv,
                bytes,
            },
        } = self;

        let (length, encode, decode) = match data {
            ast::Data::Enum(r#enum) => handle_enum(tlv, r#enum),
            ast::Data::Struct(r#struct) => handle_struct(tlv, r#struct),
        };

        let quoted = quote::quote!(
            #[automatically_derived]
            impl #tlv::Tlv for #name {
                type Error = #error;

                fn r#type(&self) -> #tlv::Type {
                    #r#type
                }

                fn length(&self) -> usize {
                    use #tlv::TlvCodec;
                    #length
                }

                fn encode_value(&self, dst: &mut #bytes::BytesMut) -> Result<(), Self::Error> {
                    use #tlv::TlvCodec;
                    #encode
                }

                fn decode_value(src: &mut #bytes::BytesMut) -> Result<Self, Self::Error> {
                    use #tlv::TlvCodec;
                    #decode
                }
            }
        );
        tokens.extend(quoted)
    }
}

fn handle_enum(
    tlv: &syn::Path,
    variants: &[PayloadVariant],
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    println!("{tlv:?}");
    println!("{variants:?}");
    for variant in variants {
        println!("{}: {:?}", variant.ident, variant.fields);
    }
    let newtype_only = variants.iter().all(|variant| variant.fields.is_newtype());
    if newtype_only {
        (
            enum_length(variants),
            enum_encode(variants),
            enum_decode(tlv, variants),
        )
    } else {
        (
            Error::custom("Only newtype enum are supported at this time").write_errors(),
            Error::custom("Only newtype enum are supported at this time").write_errors(),
            Error::custom("Only newtype enum are supported at this time").write_errors(),
        )
    }
}

fn handle_struct(
    tlv: &syn::Path,
    fields: &ast::Fields<PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let (style, fields) = fields.as_ref().split();
    match style {
        ast::Style::Tuple => handle_tuple_struct(tlv, fields),
        ast::Style::Struct => handle_regular_struct(tlv, fields),
        ast::Style::Unit => handle_unit_struct(fields),
    }
}

fn handle_tuple_struct(
    tlv: &syn::Path,
    fields: Vec<&PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    (
        tuple_length(&fields),
        tuple_encode(&fields),
        tuple_decode(tlv, &fields),
    )
}

fn tuple_length(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let fields = fields
        .iter()
        .enumerate()
        .map(|(n, _)| syn::Index::from(n))
        .map(|idx| quote::quote!( self.#idx.total_size() ));
    quote::quote!([#(#fields,)*].into_iter().sum())
}

fn tuple_encode(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let fields = fields
        .iter()
        .enumerate()
        .map(|(n, _)| syn::Index::from(n))
        .map(|idx| quote::quote!( self.#idx.encode(dst)? ));
    quote::quote!(
            #(#fields;)*
            Ok(())
    )
}

fn tuple_decode(tlv: &syn::Path, fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let fields = fields
        .iter()
        .enumerate()
        .map(|(n, item)| (syn::Index::from(n), &item.ty))
        .map(|(_idx, _ty)| quote::quote!( #tlv::TlvCodec::decode(src)? ));
    quote::quote!(
        Ok(Self(#(#fields,)*))
    )
}

fn handle_regular_struct(
    tlv: &syn::Path,
    fields: Vec<&PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    (
        struct_length(&fields),
        struct_encode(&fields),
        struct_decode(tlv, &fields),
    )
}

fn struct_length(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let fields = fields
        .iter()
        .map(|field| &field.ident)
        .map(|field| quote::quote!( self.#field.total_size() ));
    quote::quote!([#(#fields,)*].into_iter().sum())
}

fn struct_encode(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let fields = fields
        .iter()
        .map(|field| &field.ident)
        .map(|field| quote::quote!( self.#field.encode(dst)? ));
    quote::quote!(
        #(#fields;)*
        Ok(())
    )
}

fn struct_decode(tlv: &syn::Path, fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let fields = fields
        .iter()
        .map(|field| (&field.ident, &field.ty))
        .map(|(name, _ty)| quote::quote!( #name: #tlv::TlvCodec::decode(src)? ));
    quote::quote!(
        Ok(Self { #(#fields,)* })
    )
}

fn handle_unit_struct(
    fields: Vec<&PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    (
        unit_length(&fields),
        unit_encode(&fields),
        unit_decode(&fields),
    )
}

fn unit_length(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    assert!(fields.is_empty());
    quote::quote!(0)
}

fn unit_encode(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    assert!(fields.is_empty());
    quote::quote!(Ok(()))
}

fn unit_decode(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    assert!(fields.is_empty());
    quote::quote!(Ok(Self))
}

fn enum_length(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let variants = variants.iter().map(variant_length);
    quote::quote!(
            match self {
                #(#variants,)*
            }
    )
}

fn enum_encode(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let variants = variants.iter().map(variant_encode);
    quote::quote!(
            match self {
                #(#variants,)*
            }
    )
}

fn enum_decode(tlv: &syn::Path, variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let variants = variants.iter().map(|variant| variant_decode(tlv, variant));
    quote::quote!(
            let r#type = #tlv::VarNumber::peek(src)
                .ok_or_else(|| #tlv::DecodeError::invalid("Insufficient bytes to determine TLV-TYPE"))?;
            match r#type {
                #(#variants,)*
            }
    )
}

fn variant_length(variant: &PayloadVariant) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    quote::quote!(Self::#ident(payload) => payload.total_size())
}

fn variant_encode(variant: &PayloadVariant) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    quote::quote!(Self::#ident(payload) => payload.encode(dst))
}

fn variant_decode(tlv: &syn::Path, variant: &PayloadVariant) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let (_style, items) = variant.fields.as_ref().split();
    // let item = items[0];
    let ty = &items[0].ty;
    quote::quote!(
        #ty::r#type() => {
            #tlv::TlvCodec::decode(src).map(Self::#ident)
        }
    )
    //     if variant.f  #name: #tlv::TlvCodec::decode(src)? );
    // quote::quote!(Self::#ident(payload) => #name: #tlv::TlvCodec::decode(src)?)
}
