use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Error, Item, Visibility, parse};

#[proc_macro_attribute]
pub fn make_pub_for_tests(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match process_struct(attr, tokens) {
        Ok(item) => item,
        Err(err) => return err.into_compile_error().into(),
    }
}

fn process_struct(attr: TokenStream, tokens: TokenStream) -> Result<TokenStream, Error> {
    let ast_item: Item = parse::<Item>(tokens)?;
    let pub_vis: Visibility = match attr.is_empty() {
        true => parse(quote!{ pub(crate) }.into()),
        false => parse(attr),
    }.expect("this must be valid visibility tokens (`pub`, `pub(crate)`, etc)");

    let all_pub_struct = make_all_fields_public(&ast_item, pub_vis)?;

    let og_struct_tokens = ast_item.into_token_stream();
    let new_struct_tokens = all_pub_struct.into_token_stream();

    let result = quote! {
        #[cfg(not(test))]
        #og_struct_tokens

        #[cfg(test)]
        #new_struct_tokens
    };

    Ok(result.into())
}

fn make_all_fields_public(ast_item: &Item, pub_vis: Visibility) -> Result<Item, Error> {
    let mut all_pub_item: Item = ast_item.clone();
    match &mut all_pub_item {
        Item::Const(item_const) => item_const.vis = pub_vis,
        Item::Enum(item_enum) => {
            for variant in item_enum.variants.iter_mut() {
                for field in variant.fields.iter_mut() {
                    field.vis = pub_vis.clone();
                }
            }
            item_enum.vis = pub_vis;
        }
        Item::ExternCrate(item_extern_crate) => item_extern_crate.vis = pub_vis,
        Item::Fn(item_fn) => item_fn.vis = pub_vis,
        Item::Mod(item_mod) => item_mod.vis = pub_vis,
        Item::Static(item_static) => item_static.vis = pub_vis,
        Item::Struct(item_struct) => {
            for field in item_struct.fields.iter_mut() {
                field.vis = pub_vis.clone();
            }
            item_struct.vis = pub_vis;
        }
        Item::Trait(item_trait) => item_trait.vis = pub_vis,
        Item::TraitAlias(item_trait_alias) => item_trait_alias.vis = pub_vis,
        Item::Type(item_type) => item_type.vis = pub_vis,
        Item::Union(item_union) => item_union.vis = pub_vis,
        Item::Use(item_use) => item_use.vis = pub_vis,
        _ => {
            return Err(Error::new_spanned(
                ast_item,
                "unable to affect visibility, ergo this macro not compatible with this item!",
            ));
        }
    }

    Ok(all_pub_item)
}
