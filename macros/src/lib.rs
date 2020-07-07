use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Ident;
use syn::{Attribute, AttributeArgs, DeriveInput, ItemFn};

#[proc_macro_attribute]
pub fn generate_glsl_impl_inline(attr: TokenStream, item: TokenStream) -> TokenStream {
    let opts = syn::parse_macro_input!(attr as AttributeArgs);
    let mut func = syn::parse_macro_input!(item as ItemFn);

    //assert_eq!(opts.len(), 1, "Expected only one argument.");

    let mut struct_name = match &opts[0] {
        syn::NestedMeta::Lit(syn::Lit::Str(name)) => name.value(),
        _ => panic!("Expected first and only macro argument to be a string."),
    };
    println!("{:?}", opts);
    if struct_name.contains("{}") {
        /*let replacement = match &opts[1] {
            syn::NestedMeta::Lit(syn::Lit::Verbatim(name)) => name.to_string(),
            _ => panic!("You are beyond gods domain"),
        };*/
        let mut bs = Default::default();
        opts[1].to_tokens(&mut bs);
        let replacement = bs.to_string();
        struct_name = struct_name.replace("{}", &replacement);
    }

    let struct_name = syn::Ident::new(&struct_name, proc_macro::Span::call_site().into());
    let len = func.sig.inputs.len();

    let call_args = (0..len).map(|i| {
        quote! { &args[#i] }
    });

    func.sig.ident = syn::Ident::new("__impl", proc_macro::Span::call_site().into());

    (quote! {
        impl BuiltInCallableGLSL for #struct_name {
            fn generate(&self, generator: &mut GenerateGLSL, args: Vec<String>) -> String {
                assert_eq!(args.len(), #len);
                #func
                let rv = __impl(#(#call_args),*);
                rv
            }
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn generate_builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let opts = syn::parse_macro_input!(attr as AttributeArgs);
    let mut func = syn::parse_macro_input!(item as ItemFn);

    //assert_eq!(opts.len(), 1, "Expected only one argument.");

    let name = match &opts[0] {
        syn::NestedMeta::Lit(syn::Lit::Str(name)) => name.value(),
        _ => panic!("Expected first and only macro argument to be a string."),
    };

    let struct_name = func.sig.ident.clone();

    let struct_name = if struct_name.to_string().contains("{}") {
        let replacement = match &opts[1] {
            syn::NestedMeta::Lit(syn::Lit::Verbatim(name)) => name.to_string(),
            _ => panic!("You are beyond gods domain"),
        };
        Ident::new(
            &struct_name.to_string().replace("{}", &replacement),
            struct_name.span(),
        )
    } else {
        struct_name
    };

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
