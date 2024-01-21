use super::*;

pub(crate) fn handle_deprecated(
    attrs: &[syn::Attribute],
    tlv: &syn::Path,
    variants: &[PayloadVariant],
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    // println!("------");
    // println!("{attrs:#?}");
    for attr in attrs {
        println!("{:#?}", util::parse_attribute_to_meta_list(attr))
    }
    // println!("------");
    // println!("{tlv:#?}");
    // println!("{variants:#?}");
    // for variant in variants {
    //     println!(
    //         "{}: {:?}/{:?}",
    //         variant.ident, variant.discriminant, variant.fields
    //     );
    // }
    // variants.iter().for_each(|variant| {
    //     println!(
    //         "{}: is_empty()={} is_struct()={} is_tuple()={} is_unit()={}",
    //         variant.ident,
    //         variant.fields.is_empty(),
    //         variant.fields.is_struct(),
    //         variant.fields.is_tuple(),
    //         variant.fields.is_unit(),
    //     )
    // });

    let all_newtype = variants.iter().all(|variant| variant.fields.is_newtype());
    let all_unit = variants.iter().all(|variant| variant.fields.is_unit());

    if all_newtype {
        (
            length(variants),
            r#enum::newtype_encode(variants),
            r#enum::newtype_decode(tlv, variants),
        )
    } else if all_unit {
        (
            length(variants),
            r#enum::unit_encode(variants),
            r#enum::unit_decode(tlv, variants),
        )
    } else {
        (
            Error::custom("Only newtype enum are supported at this time").write_errors(),
            Error::custom("Only newtype enum are supported at this time").write_errors(),
            Error::custom("Only newtype enum are supported at this time").write_errors(),
        )
    }
}

pub(super) fn length(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let all_newtype = variants.iter().all(|variant| variant.fields.is_newtype());
    let all_unit = variants.iter().all(|variant| variant.fields.is_unit());
    if all_newtype {
        newtype_length(variants)
    } else if all_unit {
        unit_length(variants)
    } else {
        Error::unsupported_shape("XXX").write_errors()
    }
}

fn newtype_length(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let variants = variants.iter().map(variant_length);
    quote::quote!(
            match self {
                #(#variants,)*
            }
    )
}

fn newtype_encode(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let variants = variants.iter().map(variant_encode);
    quote::quote!(
            match self {
                #(#variants,)*
            }
    )
}

fn newtype_decode(tlv: &syn::Path, variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let variants = variants.iter().map(|variant| variant_decode(tlv, variant));
    quote::quote!(
            let r#type = #tlv::VarNumber::peek(src)
                .ok_or_else(|| #tlv::DecodeError::invalid("Insufficient bytes to determine TLV-TYPE"))?;
            match r#type {
                #(#variants,)*
            }
    )
}

pub(super) fn unit_length(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let _ = variants;
    todo!("unit_length")
}

pub(super) fn unit_encode(variants: &[PayloadVariant]) -> proc_macro2::TokenStream {
    let _ = variants;
    todo!("unit_encode")
}

pub(super) fn unit_decode(
    tlv: &syn::Path,
    variants: &[PayloadVariant],
) -> proc_macro2::TokenStream {
    let _ = (tlv, variants);
    todo!("unit_decode")
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
