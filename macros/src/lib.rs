use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::*;
use syn::parse::{Parse, ParseStream, Result};
use syn::Pat::{Type};

fn replace_str_dyn(mut source: String, insert: &[String]) -> String {
    let mut i = 0;
    while let Some(offset) = source.find("{}") {
        if insert.len() < i {
            panic!();
        }
        source.replace_range(offset..(offset + 2), insert[i].as_str());
        i += 1;
    }

    if insert.len() != i {
        panic!();
    }

    source
}

#[proc_macro_attribute]
pub fn generate_glsl_impl_inline(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let opts = syn::parse_macro_input!(attr as AttributeArgs);
	let mut func = syn::parse_macro_input!(item as ItemFn);

	//assert_eq!(opts.len(), 1, "Expected only one argument.");

	let mut struct_name = match &opts[0] {
		syn::NestedMeta::Lit(syn::Lit::Str(name)) => name.value(),
		_ => panic!("Expected first and only macro argument to be a string."),
	};
	//println!("{:?}", opts);
	if struct_name.contains("{}") {
        let mut str_ops = Vec::new();
        for i in 1..opts.len() {
            let mut bs = Default::default();
            opts[i].to_tokens(&mut bs);
            str_ops.push(bs.to_string());
        }
        struct_name = replace_str_dyn(struct_name, str_ops.as_slice());
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
pub fn generate_builtin_fn(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let opts = syn::parse_macro_input!(attr as AttributeArgs);
	let mut func = syn::parse_macro_input!(item as ItemFn);

	//assert_eq!(opts.len(), 1, "Expected only one argument.");

	let mut name = match &opts[0] {
		syn::NestedMeta::Lit(syn::Lit::Str(name)) => name.value(),
		_ => panic!("Expected first and only macro argument to be a string."),
    };
    if name.contains("{}") {
        let mut str_ops = Vec::new();
        for i in 1..opts.len() {
            let mut bs = Default::default();
            opts[i].to_tokens(&mut bs);
            str_ops.push(bs.to_string());
        }
        name = replace_str_dyn(name, str_ops.as_slice());
    }
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

#[proc_macro]
pub fn generate_vector_ctor(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let n: usize = syn::parse_macro_input!(item as LitInt).to_string().parse().unwrap();
	let ident = format_ident!("Vec{}", n);

	generate_ctor(&ident.to_string(), n, quote!(f32), "", |args| {
		quote! { #ident::new(#(#args),*) }
	})
}

#[proc_macro]
pub fn generate_matrix_ctor(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let parsed = item
		.into_iter()
		.filter_map(|x| match x {
			proc_macro::TokenTree::Literal(l) => Some(l.to_string().parse::<usize>().unwrap()),
			_ => None,
		})
		.collect::<Vec<_>>();
	let (m, n) = (parsed[0], parsed[1]);

	let name = if n == m {
		format!("Mat{}", n)
	} else {
		format!("Mat{}x{}", m, n)
	};

	let mut ctors = generate_ctor(&name, m * n, quote!(f32), "", |args| {
		let chunks = args.chunks(n);
		let rows_arr = chunks.into_iter().map(|x| quote! { [ #(#x),* ] });
		quote! {
			Matrix::new([
				#(#rows_arr),*
			])
		}
	});
	ctors.extend(generate_ctor(&name, m, format_ident!("Vec{}", n), "Vector", |args| {
		quote! {
			Matrix::from_vecs([
				#(#args),*
			])
		}
	}));

	ctors
}

fn generate_ctor(
	name: &str,
	params: usize,
	param_type: impl ToTokens,
	name_suffix: &str,
	body: impl Fn(Vec<Ident>) -> TokenStream,
) -> proc_macro::TokenStream {
	let args_packed = (0..params).map(|x| {
		let ident = format_ident!("m{}", x);
		(ident.clone(), quote! {#ident: #param_type}, quote! {#ident: &str})
	});
	let args = args_packed.clone().map(|x| x.0);
	let args_typed_f32 = args_packed.clone().map(|x| x.1);
	let args_typed_str = args_packed.clone().map(|x| x.2);

	let name_ident = format_ident!("{}", &name);
	let name_lower = name.to_lowercase();

	let fmt_string = (0..params - 1).fold("{}".to_string(), |acc, _| acc + ", {}");
	let fmt_string = format!("{}({})", name_lower, fmt_string);
	let impl_string = format!("{}{}Constructor", name, name_suffix);
	let impl_ident = format_ident!("{}", &impl_string);

	let body_result = body(args.clone().collect());

	(quote! {
		#[generate_builtin_fn(#name)]
		fn #impl_ident(#(#args_typed_f32),*) -> #name_ident {
			#body_result
		}

		#[generate_glsl_impl_inline(#impl_string)]
		fn generate(#(#args_typed_str),*) -> String {
			format!(#fmt_string, #(#args),*)
		}
	})
	.into()
}

#[derive(Clone, Debug)]
struct Test {
    func_name: Ident,
    sl_func_name: Ident,
    impl_body: ExprClosure,
    sl_impl_body: LitStr
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let sl_func_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let impl_body: ExprClosure = input.parse()?;
        input.parse::<Token![,]>()?;
        let sl_impl_body: LitStr = input.parse()?;

        Ok(Self {
            func_name,
            sl_func_name,
            impl_body,
            sl_impl_body
        })
    }
}

#[proc_macro]
pub fn bingbong(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // bingbong!(Elem, elem, |a: Vec2, b: Int| -> Float { a.get_elem(b as usize) }, "{}[{}]");

    let parsed = parse_macro_input!(item as Test);
    let closure = parsed.impl_body.clone();
    let fmt_string = parsed.sl_impl_body.clone();
    let glfunc = parsed.sl_func_name.clone().to_string().to_token_stream();
    let ret_type = parsed.impl_body.output;
    let ret_type = match ret_type {
        ReturnType::Type(_, x) => x,
        _ => panic!()
    };

    let args_untyped_first = parsed.impl_body.inputs.clone().into_iter()
        .map(|x| match x { Type(pt) => *pt.pat, _ => panic!() }).collect::<Vec<_>>();

    let types = parsed.impl_body.inputs.to_token_stream();
    
    fn str_vec_to_tokenstream(a: Vec<String>) -> TokenStream {
        a.join(", ").parse::<TokenStream>().unwrap()
    }

    let num_args = parsed.impl_body.inputs.clone().into_iter().len();
    let str_args_typed = str_vec_to_tokenstream((0..num_args).map(|i| format!("a{}: &str", i)).collect());
    let str_args_untyped = str_vec_to_tokenstream((0..num_args).map(|i| format!("a{}", i)).collect());

    let args_typed_first = parsed.impl_body.inputs.clone().into_iter()
        .map(|x| match x { Type(pt) => (*pt.ty).into_token_stream().to_string(), _ => panic!() })
        .collect::<Vec<_>>().join("");
    let ident_final = format_ident!("{}{}", args_typed_first, parsed.func_name);
    let ident_final_str = ident_final.to_string().to_token_stream();

    (quote!{
        #[generate_builtin_fn(#glfunc)]
        fn #ident_final(#types) -> #ret_type {
            let _impl = #closure;
            _impl(#(#args_untyped_first),*)
        }

        #[generate_glsl_impl_inline(#ident_final_str)]
        fn generate(#str_args_typed) -> String {
            format!(#fmt_string, #str_args_untyped)
        }
    }).into()
}