use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, AttributeArgs, DeriveInput, ItemFn};

#[proc_macro_attribute]
pub fn generate_builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let opts = syn::parse_macro_input!(attr as AttributeArgs);
    let mut func = syn::parse_macro_input!(item as ItemFn);

    assert_eq!(opts.len(), 1, "Expected only one argument.");

    let name = match &opts[0] {
        syn::NestedMeta::Lit(syn::Lit::Str(name)) => name.value(),
        _ => panic!("Expected first and only macro argument to be a string."),
    };

    let struct_name = func.sig.ident.clone();

    let ret_type = match &func.sig.output {
        syn::ReturnType::Type(_, t) => t,
        _ => panic!(),
    };

    let cursed_wrap = func.sig.inputs.clone().into_iter().map(|a| match a {
        syn::FnArg::Typed(pt) => {
            let name = match &*pt.pat {
                syn::Pat::Ident(i) => i.ident.clone(),
                _ => panic!("Please stop"),
            };

            let ty = pt.ty;

            let args = quote! {
                let #name = unsafe { vm.pop_stack::<#ty>() };
            };

            let arg_types = quote! {
                <#ty as BuiltInType>::type_kind()
            };

            (args, arg_types)
        }
        v => panic!("Unexpected argument: {:?}", v),
    });

    let args = cursed_wrap.clone().rev().map(|(a, _)| a);
    let arg_types = cursed_wrap.clone().map(|(_, a)| a);

    let call_args = func.sig.inputs.clone().into_iter().map(|a| match a {
        syn::FnArg::Typed(syn::PatType { pat: p, .. }) => match &*p {
            syn::Pat::Ident(i) => i.ident.clone(),
            _ => panic!(),
        },
        _ => panic!(),
    });

    func.sig.ident = syn::Ident::new("__impl", proc_macro::Span::call_site().into());

    (quote! {

        pub struct #struct_name;


        impl BuiltInCallable for #struct_name {
            fn ident(&self) -> &str { #name }
            fn return_type(&self) -> TypeKind { <#ret_type as BuiltInType>::type_kind() }
            fn arg_types(&self) -> Vec<TypeKind> {
                vec![
                    #(#arg_types),*
                ]
            }
            fn vm_impl(&self, vm: &mut VirtualMachine) {
                #(#args)*

                #func;

                let rv = __impl(#(#call_args),*);

                vm.push_stack(rv);
            }
        }
    })
    .into()
}
