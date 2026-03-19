use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn perm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let perm_code = parse_macro_input!(attr as LitStr);
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let perm_value = perm_code.value();

    // 追加一个提取器参数: Extension(__rulo_perms): Extension<PermCodes>
    let extra_param: FnArg = syn::parse_quote! {
        axum::Extension(__rulo_perms): axum::Extension<crate::system::auth::model::PermCodes>
    };
    input_fn.sig.inputs.push(extra_param);

    // 在原始函数体前插入权限检查
    let original_block = &input_fn.block;
    let new_block = syn::parse_quote! {
        {
            if !__rulo_perms.0.iter().any(|p| p==#perm_value) {
                return Err(rulo_common::error::AppError::Forbidden(format!("缺少权限:{}", #perm_value)));
            }
            #original_block
        }
    };
    input_fn.block = Box::new(new_block);

    let expanded = quote! {#input_fn};
    expanded.into()
}
