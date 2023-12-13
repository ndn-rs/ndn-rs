use darling::ast;
use darling::util;
use darling::FromDeriveInput;
use darling::FromField;
use darling::FromMeta;

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
    data: ast::Data<util::Ignored, PayloadItem>,
    r#type: syn::Path,
    #[darling(default)]
    crates: Crates,
}

#[derive(Debug, FromField)]
struct PayloadItem {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(Debug, FromMeta)]
struct Crates {
    #[darling(default = "Self::default_tlv")]
    tlv: syn::Path,
    #[darling(default = "Self::default_bytes")]
    bytes: syn::Path,
}

impl Default for Crates {
    fn default() -> Self {
        Self {
            tlv: Self::default_tlv(),
            bytes: Self::default_bytes(),
        }
    }
}

impl Crates {
    fn default_tlv() -> syn::Path {
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
            crates: Crates { tlv, bytes },
        } = self;

        // let name = &self.ident;
        // let r#type = &self.r#type;
        // let data = &self.data;
        // let tlv = &self.crates.tlv;
        // let bytes = &self.crates.bytes;

        if data.is_struct() {
            println!("{data:?}");
        }

        if let Some(fields) = data.as_ref().take_struct() {
            println!("{fields:?}");
            if fields.is_empty() {
                println!("{name} is unit type");
            }

            let (style, fields) = fields.split();
            println!("{style:?}");
            println!("{fields:?}");
            fields
                .into_iter()
                .enumerate()
                .for_each(|(n, f)| println!("{n}: \"{:?}: {:?}\"", f.ident, f.ty))
        }

        let quoted = quote::quote!(
            #[automatically_derived]
            impl #tlv::Tlv for #name {
                fn r#type(&self) -> #tlv::Type {
                    #r#type
                }

                fn value(&self) -> Option<#bytes::Bytes> {
                    None
                }

                fn payload_size(&self) -> usize {
                    0
                }
            }
        );
        tokens.extend(quoted)
    }
}

// pub trait Tlv: fmt::Debug {
//     // /// Each TLV type has its assigned TLV-TYPE number defined as a constant of type u64
//     // const TYPE: Type;

//     /// Report this TLV-TYPE as `Type`
//     fn r#type(&self) -> Type;

//     /// report this TLV-TYPE as a `VarNumber`
//     fn type_as_varnumber(&self) -> VarNumber {
//         self.r#type().to_varnumber()
//     }

//     /// Report TLV-LENGTH as a `VarNumber`
//     fn length(&self) -> VarNumber {
//         self.payload_size().into()
//     }

//     /// Report TLV-VALUE as `Bytes` buffer (if value is present)
//     fn value(&self) -> Option<Bytes>;

//     /// Report the total size of the packet or TLV element, including the TLV-TYPE and TLV-LENGTH
//     fn size(&self) -> usize {
//         self.payload_size() + self.type_as_varnumber().len() + self.length().len()
//     }

//     /// Report the size of the payload if any
//     fn payload_size(&self) -> usize;

//     /// Convert this TLV to `Bytes`
//     fn bytes(&self) -> Bytes {
//         let mut bytes = BytesMut::new();
//         self.write(&mut bytes);
//         bytes.freeze()
//     }

//     /// Write this TLV to `BytesMut`
//     fn write(&self, dst: &mut BytesMut) {
//         let r#type = self.type_as_varnumber().bytes();
//         let length = self.length().bytes();
//         let value = self.value().unwrap_or_default();
//         let additional = r#type.len() + length.len() + value.len();
//         dst.reserve(additional);
//         dst.extend([r#type, length, value]);
//     }
// }
