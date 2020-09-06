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
    Ident, Meta, MetaList,
};

#[derive(Debug)]
enum TreeNodeAttrbiuteMacroParsingError {
    TitleAttributeMacroIsMissing,
    TitleAttributeMacroMustHaveFieldNameSettings,
    SearchableAttributeMacroMustHaveFieldNamesSettings,
}

impl std::fmt::Display for TreeNodeAttrbiuteMacroParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_message = match &self {
           TreeNodeAttrbiuteMacroParsingError::TitleAttributeMacroIsMissing => "#[title(field_name = \"xxxx\"] required" ,
           TreeNodeAttrbiuteMacroParsingError::TitleAttributeMacroMustHaveFieldNameSettings => "#[title(field_name = \"xxxx\"] required" ,
           TreeNodeAttrbiuteMacroParsingError::SearchableAttributeMacroMustHaveFieldNamesSettings => "Make sure to use correct setting for 'searchable' attribute like this: #[searchable(field_names = \"xxx[,yyy])\"" ,
        };
        f.write_str(error_message)
    }
}

impl std::error::Error for TreeNodeAttrbiuteMacroParsingError {}

#[derive(Debug)]
struct TreeNodeAttrbiute {
    title: String,
    searchable: Vec<String>,
}

type TreeNodeAttrbiuteResult = std::result::Result<TreeNodeAttrbiute, Box<dyn std::error::Error>>;

// /// Example to define the outer attribute macro
// #[proc_macro_attribute]
// pub fn title(attr: TokenStream, item: TokenStream) -> TokenStream {
// println!("attr: \"{}\"", attr.to_string());
// println!("item: \"{}\"", item.to_string());
//
// item
// }
//

///
fn get_title_and_searchable_from_attrs(attrs: &Vec<syn::Attribute>) -> TreeNodeAttrbiuteResult {
    if attrs.len() <= 0 {
        return Err(TreeNodeAttrbiuteMacroParsingError::TitleAttributeMacroIsMissing.into());
    }

    // Check whether `title` and `field_name` exists or not and check
    let title_attr_option = attrs
        .iter()
        .find(|&attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "title");

    if title_attr_option.is_none() {
        return Err(TreeNodeAttrbiuteMacroParsingError::TitleAttributeMacroIsMissing.into());
    }

    let mut title_filed_name: Option<String> = None;

    if let Ok(Meta::List(meta_list)) = &title_attr_option.as_ref().unwrap().parse_meta() {
        // println!("title attr meta_list: {:#?}", meta_list);

        if meta_list.nested.len() > 0 {
            if let Some(syn::NestedMeta::Meta(Meta::NameValue(meta_name_value))) =
                meta_list.nested.iter().next()
            {
                if meta_name_value.path.segments.len() > 0
                    && meta_name_value.path.segments[0].ident == "field_name"
                {
                    if let syn::Lit::Str(str_value) = &meta_name_value.lit {
                        if str_value.value().len() > 0 {
                            title_filed_name = Some(str_value.value());
                        }
                    }
                }
            }
        }
    }

    if title_filed_name.is_none() {
        return Err(
            TreeNodeAttrbiuteMacroParsingError::TitleAttributeMacroMustHaveFieldNameSettings.into(),
        );
    }

    // Check whether `searchable` and `field_names` exists or not and check
    let searchable_attr_option = attrs
        .iter()
        .find(|&attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "searchable");

    let mut searchable_field_names_vec: Vec<String> = Vec::new();

    if searchable_attr_option.is_some() {
        if let Ok(Meta::List(meta_list)) = &searchable_attr_option.as_ref().unwrap().parse_meta() {
            println!("searchable attr meta_list: {:#?}", meta_list);

            if meta_list.nested.len() > 0 {
                if let Some(syn::NestedMeta::Meta(Meta::NameValue(meta_name_value))) =
                    meta_list.nested.iter().next()
                {
                    if meta_name_value.path.segments.len() > 0
                        && meta_name_value.path.segments[0].ident == "field_names"
                    {
                        if let syn::Lit::Str(str_value) = &meta_name_value.lit {
                            if str_value.value().len() > 0 {
                                let searchable_field_names = str_value.value();

                                for temp_str in searchable_field_names.split(",").into_iter() { 
                                    searchable_field_names_vec.push(temp_str.trim().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        if searchable_field_names_vec.len() <= 0 {
            return Err(
                TreeNodeAttrbiuteMacroParsingError::SearchableAttributeMacroMustHaveFieldNamesSettings .into(),
            );
        }
    }

    let result = TreeNodeAttrbiute {
        title: title_filed_name.unwrap(),
        searchable: searchable_field_names_vec,
    };

    Ok(result)
}

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

    // Generate the trait `Ident`
    let trait_name = Ident::new("GenerateTreeNodeHashmapKey", struct_name.span());
    let tree_node_trait_name = Ident::new("SimpleFastTreeNode", struct_name.span());
    // println!("ast {:#?}", ast);

    // Attributes result
    let title_and_searchable_result = match get_title_and_searchable_from_attrs(&ast.attrs) {
        Ok(result) => result,
        Err(error) => panic!(error.to_string()),
    };
    println!(
        "title_and_searchable_result: {:#?}",
        &title_and_searchable_result
    );

    // Get back all the `fields` in the original struct, ignore the `Enum` and `Union` cases.
    // As the `Punctuated<Field, Comma>` has the `iter()` which allows us to walk through each
    // field manually.
    let struct_fields: Option<Punctuated<Field, Comma>> = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => Some(named_fields.named),
            _ => panic!("'FlattenTreeNode' struct got empty fields."),
        },
        Data::Enum(_) => panic!("'FlattenTreeNode' currently NOT support `enum` yet."),
        Data::Union(_) => panic!("'FlattenTreeNode' currently NOT support `Union` yet."),
    };

    let struct_fields_ref = struct_fields.as_ref().unwrap();
    if struct_fields_ref.len() <= 0 {
        panic!("'FlattenTreeNode' struct got empty fields.")
    }

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

        // We implement the important traits privately
        impl #struct_name {
            // pub fn new() -> #struct_name {
                // #struct_name {
                    // name: "Google .Inc".to_owned(),
                    // address: "US Address".to_owned(),
                    // ceo: "Wison Ye".to_owned(),
                    // departments: None
                // }
            // }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
