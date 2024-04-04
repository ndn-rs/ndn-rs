use darling::ast;
use darling::util;
use darling::Error;
use darling::FromDeriveInput;
use darling::FromField;
use darling::FromMeta;
use darling::FromVariant;

use darling::export::syn;

mod r#enum;

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
#[darling(
    attributes(tlv),
    forward_attrs(repr),
    supports(struct_any, enum_newtype, enum_unit)
)]
struct TlvDerive {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    data: ast::Data<PayloadVariant, PayloadItem>,
    r#type: syn::Path,
    error: syn::Path,
    #[darling(default = "Crates::default_crates")]
    crates: Crates,
}

#[derive(Debug, FromVariant)]
struct PayloadVariant {
    ident: syn::Ident,
    // discriminant: Option<syn::Expr>,
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
}

impl Default for Crates {
    fn default() -> Self {
        Self::default_crates()
    }
}

impl Crates {
    fn default_crates() -> Self {
        Self {
            tlv_core: Self::default_tlv_core(),
        }
    }

    fn default_tlv_core() -> syn::Path {
        syn::parse_quote!(::ndn_tlv_core)
    }

    fn tlv_core(&self) -> &syn::Path {
        &self.tlv_core
    }

    fn bytes_mut(&self) -> syn::Path {
        let tlv = self.tlv_core();
        syn::parse_quote!(#tlv::export::BytesMut)
    }

    fn result(&self) -> syn::Path {
        let tlv = self.tlv_core();
        syn::parse_quote!(#tlv::export::Result)
    }

    fn ok(&self) -> syn::Path {
        let tlv = self.tlv_core();
        syn::parse_quote!(#tlv::export::Ok)
    }
}

impl quote::ToTokens for TlvDerive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ident: name,
            attrs,
            data,
            r#type,
            error,
            crates,
        } = self;

        let tlv = crates.tlv_core();
        let bytes_mut = crates.bytes_mut();
        let result = crates.result();

        let (length, encode, decode) = match data {
            ast::Data::Enum(variants) => r#enum::handle_deprecated(attrs, tlv, variants),
            ast::Data::Struct(fields) => handle_struct(crates, fields),
        };

        let quoted = quote::quote!(
            #[automatically_derived]
            impl #tlv::Tlv for #name {
                type Error = #error;
                const TYPE: #tlv::Type = #r#type;

                // fn r#type(&self) -> #tlv::Type {
                //     #r#type
                // }

                fn length(&self) -> usize {
                    use #tlv::TlvCodec;
                    #length
                }

                fn encode_value(&self, dst: &mut #bytes_mut) {
                    use #tlv::TlvCodec;
                    #encode
                }

                fn decode_value(r#type: #tlv::Type, length: usize, src: &mut #bytes_mut) -> #result<Self, Self::Error> {
                    use #tlv::TlvCodec;
                    #decode
                }
            }
        );
        tokens.extend(quoted)
    }
}

fn handle_struct(
    crates: &Crates,
    fields: &ast::Fields<PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let (style, fields) = fields.as_ref().split();
    match style {
        ast::Style::Tuple => handle_tuple_struct(crates, fields),
        ast::Style::Struct => handle_regular_struct(crates, fields),
        ast::Style::Unit => handle_unit_struct(crates, fields),
    }
}

fn handle_tuple_struct(
    crates: &Crates,
    fields: Vec<&PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    (
        tuple_length(&fields),
        tuple_encode(&fields),
        tuple_decode(crates, &fields),
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
        .map(|idx| quote::quote!( self.#idx.encode(dst); ));
    quote::quote!(
            #(#fields;)*
    )
}

fn tuple_decode(crates: &Crates, fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let tlv = crates.tlv_core();
    let ok = crates.ok();
    let fields = fields
        .iter()
        .enumerate()
        .map(|(n, item)| (syn::Index::from(n), &item.ty))
        .map(|(_idx, _ty)| quote::quote!( #tlv::TlvCodec::decode(src)? ));
    quote::quote!(
        #ok(Self(#(#fields,)*))
    )
}

fn handle_regular_struct(
    crates: &Crates,
    fields: Vec<&PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    (
        struct_length(&fields),
        struct_encode(&fields),
        struct_decode(crates, &fields),
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
        .map(|field| quote::quote!( self.#field.encode(dst); ));
    quote::quote!(
        #(#fields;)*
    )
}

fn struct_decode(crates: &Crates, fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    let tlv = crates.tlv_core();
    let ok = crates.ok();
    let fields = fields
        .iter()
        .map(|field| (&field.ident, &field.ty))
        .map(|(name, _ty)| quote::quote!( #name: #tlv::TlvCodec::decode(src)? ));
    quote::quote!(
        #ok(Self { #(#fields,)* })
    )
}

fn handle_unit_struct(
    crates: &Crates,
    fields: Vec<&PayloadItem>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    (
        unit_length(&fields),
        unit_encode(&fields),
        unit_decode(crates, &fields),
    )
}

fn unit_length(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    assert!(fields.is_empty());
    quote::quote!(0)
}

fn unit_encode(fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    assert!(fields.is_empty());
    quote::quote!()
}

fn unit_decode(crates: &Crates, fields: &[&PayloadItem]) -> proc_macro2::TokenStream {
    assert!(fields.is_empty());
    let ok = crates.ok();
    quote::quote!(#ok(Self))
}
