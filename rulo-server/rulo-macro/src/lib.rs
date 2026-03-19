use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn perm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let perm_code = parse_macro_input!(attr as LitStr);
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let perm_value = perm_code.value();

    // 插入一个提取器参数: Extension(__rulo_perms): Extension<PermCodes>
    // axum 要求 Json(body) 必须是最后一个参数, 所以不能 push 到末尾
    let extra_param: FnArg = syn::parse_quote! {
        axum::Extension(__rulo_perms): axum::Extension<rulo_common::model::PermCodes>
    };
    let mut inputs: Vec<FnArg> = input_fn.sig.inputs.into_iter().collect();
    inputs.insert(0, extra_param);
    input_fn.sig.inputs = inputs.into_iter().collect();

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
