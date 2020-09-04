//! What a procedural macro does is that: Take the input `TokenStream`, do something with the code,
//! and then return back an output `TokenStream`. In other words, just take some code as input and
//! generate back the new code as the output.
//!
//! More details at [here](https://doc.rust-lang.org/reference/procedural-macros.html)
//!
//! The `syn` crate helps us to analyze the input `TokenStream` and get back the syntax tree which
//! is a `DeriveInput` instance. We can use that instance to get back the meta data of the data
//! type and help us to write our output code.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
    Ident,
};

// /// Example to define the outer attribute macro
// #[proc_macro_attribute]
// pub fn title(attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("attr: \"{}\"", attr.to_string());
    // println!("item: \"{}\"", item.to_string());
// 
    // item
// }

/// The derive macro which will applied on a particular `struct` to implement some important traits.
///
/// - `FlattenTreeNode` is the macro name.
///
/// - `attributes(title, searchable))` enable the helper attributes. 
///
/// But we got a bug here:
///
/// As the `derive macro helper attributes` should be `inert` attribute which can apply to the struct 
/// field like below, but you will get back the error:
///
/// **`cannot find attribute 'title' in this scope`**:
///
/// ```rust
/// #[derive(FlattenTreeNode, Debug)]
/// struct Compnay {
///     #[title]
///     name: String,
///     #[searchable]
///     address: String
/// }
/// ```
///
/// So the workaround at this moment is that apply the helper attribute on the outer part like
/// below:
///
/// ```rust
/// #[title(field_name = "name")]
/// #[searchable(field_names = "address")]
/// #[derive(FlattenTreeNode, Debug)]
/// struct Compnay {
///     name: String,
///     address: String
/// }
/// ```
#[proc_macro_derive(FlattenTreeNode, attributes(title, searchable))]
pub fn derive_flatten_tree_node(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Original struct name (`Ident` instance)
    let struct_name = &ast.ident;

    // Generate the proxy struct based on the original struct name, it's an `Ident` instance.
    // `struct_name`.span()` can help the compiler to track the original error source (if error
    // happens)
    let proxy_struct_ident =
        Ident::new(format!("Inner{}", struct_name).as_str(), struct_name.span());

    // Generate the trait `Ident`
    let trait_name = Ident::new("GenerateTreeNodeHashmapKey", struct_name.span());
    let tree_node_trait_name = Ident::new("SimpleFastTreeNode", struct_name.span());
    // println!("ast {:#?}", ast);

    // Get back all the  `fields` in the original struct, ignore the `Enum` and `Union` cases.
    // As the `Punctuated<Field, Comma>` has the `iter()` which allows us to walk through each
    // field manually.
    let struct_fields: Option<Punctuated<Field, Comma>> = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => Some(named_fields.named),
            _ => None,
        },
        Data::Enum(_) => None,
        Data::Union(_) => None,
    };
    let struct_fields_ref = struct_fields.as_ref().unwrap();

    // println!("struct_fields: {:#?}", struct_fields);

    // Walk though the fields
    // for temp_field in struct_fields_ref.iter() {
    // println!("temp_field: {:#?}", temp_field);
    // }

    // We also can modify the field data type to what we want as well!!!
    // Here is an example: We wrap all original field data types into an `Option` (if it's not).
    // let is_data_type_is_an_option = |f: &Field| -> bool {
    // match f.ty {
    // Type::Path(ref temp_type_path) => {
    // let temp_path: &Path = &temp_type_path.path;
    // return temp_path.segments.len() == 1 && temp_path.segments[0].ident == "Option";
    // }
    // _ => false,
    // }
    // };

    // let wrapped_in_option_type_fields = struct_fields_ref.iter().map(|f: &Field| {
    // let field_ident = &f.ident;
    // let field_type = &f.ty;
    //
    // if is_data_type_is_an_option(f) {
    // // Should expand to `xxx: yyyy`
    // quote! { #field_ident: #field_type }
    // } else {
    // // Should expand to `xxx: std::option::Option<yyyy>`
    // quote! { #field_ident: std::option::Option<#field_type> }
    // }
    // });

    // `GenerateTreeNodeHashmapKey` trait related
    let hashkey_format_value_part = struct_fields_ref.iter().map(|f: &Field| {
        let field_ident = &f.ident;

        // Should expand to `self.xxx, self.yyy, ...`
        quote! { self.#field_ident, }
    });

    let mut hashkey_format_string_part_vec = Vec::with_capacity(hashkey_format_value_part.len());
    for _ in 0..hashkey_format_value_part.len() {
        hashkey_format_string_part_vec.push("{}");
    }
    let hashkey_format_string_part = hashkey_format_string_part_vec.join("|");

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {

        // The inner struct we will generated as a `proxy` for the original struct
        #[derive(Debug)]
        pub struct #proxy_struct_ident {
            // We can use this syntax to apply all the fields into the proxy struct
            #struct_fields

            // Also, we can use this syntax to apply all original struct fields as well:
            // #(#struct_fields,)*
            //
            // It's talking about iterate all in `#struct_fields`.
            // #(#wrapped_in_option_type_fields,)*
        }

        // We implement the important traits privately
        impl #struct_name {
            pub fn new() -> #struct_name {
                #struct_name {
                    name: "Google .Inc".to_owned(),
                    address: "US Address".to_owned(),
                    ceo: "Wison Ye".to_owned(),
                    departments: None
                }
            }
        }

        impl #proxy_struct_ident {
        }

        // impl #tree_node_trait_name for #struct_name {
            // fn generate_tree_node_hashmap_key(&self) -> String {
                // format!(#hashkey_format_string_part, #(#hashkey_format_value_part)*)
            // }
        // }
    };

    // Hand the output tokens back to the compiler
    //
    TokenStream::from(expanded)
}
