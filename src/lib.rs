// endian-writer-derive/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Procedural macro to auto derive [`EndianWritableAt`] and [`EndianReadableAt`] traits for structs
/// whose members implement [`HasSize`], [`EndianWritableAt`], and [`EndianReadableAt`].
///
/// # Requirements
///
/// - The macro can only be applied to structs with named fields.
/// - All fields within the struct must implement [`HasSize`], [`EndianWritableAt`], and [`EndianReadableAt`].
/// - The struct should have a deterministic layout, typically enforced using `#[repr(C)]`.
///
/// # Example
///
/// ```rust
/// use endian_writer::{EndianWriter, EndianReadableAt, HasSize};
/// use endian_writer_derive::EndianWritable;
///
/// #[derive(EndianWritable)]
/// #[repr(C)]
/// struct MyStruct {
///     a: u32,
///     b: u16,
///     c: u8,
/// }
/// ```
#[proc_macro_derive(EndianWritable)]
pub fn derive_endian(input: TokenStream) -> TokenStream {
    derive_endian_impl(input)
}

pub(crate) fn derive_endian_impl(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = input.ident;

    // Ensure the input is a struct with named fields
    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => {
                return syn::Error::new_spanned(
                    data_struct.struct_token,
                    "EndianWritable can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "EndianWritable can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Prepare vectors to hold field names and types
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

    for field in fields.iter() {
        let field_ident = match &field.ident {
            Some(ident) => ident.clone(),
            None => {
                return syn::Error::new_spanned(&field.ty, "All fields must have names")
                    .to_compile_error()
                    .into();
            }
        };
        field_names.push(field_ident.clone());
        field_types.push(field.ty.clone());
    }

    // Generate the implementation for `HasSize`
    // Generates: `const SIZE: usize = <u32 as HasSize>::SIZE + <u16 as HasSize>::SIZE + <u8 as HasSize>::SIZE;`
    //            or similar.
    let has_size_impl = {
        let sizes = field_types.iter().map(|ty| {
            quote! {
                <#ty as HasSize>::SIZE
            }
        });

        let sum_sizes = sizes.fold(quote! { 0 }, |acc, size| {
            quote! { #acc + #size }
        });

        quote! {
            impl HasSize for #name {
                const SIZE: usize = #sum_sizes;
            }
        }
    };

    // Generate the implementation for `EndianWritableAt`
    let writable_impl = {
        let mut sum_expr = quote! { 0 };
        let write_fields = field_names
            .iter()
            .zip(field_types.iter())
            .map(|(field, ty)| {
                let current_offset = if sum_expr.to_string() == "0" {
                    quote! { offset }
                } else {
                    quote! { offset + #sum_expr }
                };

                let write_field = quote! {
                    let #field = self.#field;
                    writer.write_at(&#field, #current_offset);
                };

                let new_sum_expr = quote! { #sum_expr + <#ty as HasSize>::SIZE as isize };

                sum_expr = new_sum_expr.clone();

                write_field
            });

        quote! {
            impl EndianWritableAt for #name {
                unsafe fn write_at<W: EndianWriter>(&self, writer: &mut W, offset: isize) {
                    #(
                        #write_fields
                    )*
                }
            }
        }
    };

    // Generate the implementation for `EndianReadableAt`
    let readable_impl = {
        let mut sum_expr = quote! { 0 };
        let read_fields = field_names
            .iter()
            .zip(field_types.iter())
            .map(|(field, ty)| {
                let current_offset = if sum_expr.to_string() == "0" {
                    quote! { offset }
                } else {
                    quote! { offset + #sum_expr }
                };

                let read_field = quote! {
                    let #field = <#ty as EndianReadableAt>::read_at(reader, #current_offset);
                };

                let new_sum_expr = quote! { #sum_expr + <#ty as HasSize>::SIZE as isize };

                sum_expr = new_sum_expr.clone();

                read_field
            });

        let assign_fields = field_names.iter();

        quote! {
            impl EndianReadableAt for #name {
                unsafe fn read_at<R: EndianReader>(reader: &mut R, offset: isize) -> Self {
                    #(
                        #read_fields
                    )*
                    Self {
                        #(
                            #assign_fields,
                        )*
                    }
                }
            }
        }
    };

    // Combine all implementations
    let expanded = quote! {
        use endian_writer::*;
        #has_size_impl
        #writable_impl
        #readable_impl
    };

    // Return the generated impl as a TokenStream
    TokenStream::from(expanded)
}
