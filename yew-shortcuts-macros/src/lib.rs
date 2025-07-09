use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Meta, Path, PathArguments};

/// Clone multiple variables at once for use in closures.
/// 
/// # Example
/// ```rust
/// use yew_shortcuts::cs;
/// 
/// let a = vec![1, 2, 3];
/// let b = String::from("hello");
/// let c = 42;
/// 
/// cs!(a, b, c);
/// // Equivalent to:
/// // let a = a.clone();
/// // let b = b.clone();
/// // let c = c.clone();
/// ```
#[proc_macro]
pub fn cs(input: TokenStream) -> TokenStream {
    let vars = parse_macro_input!(input with Punctuated::<Ident, syn::Token![,]>::parse_terminated);
    
    let clones = vars.iter().map(|var| {
        quote! {
            let #var = #var.clone();
        }
    });
    
    TokenStream::from(quote! {
        #(#clones)*
    })
}

/// Combines `#[yew_autoprops::autoprops]` and `#[yew::function_component]` attributes.
/// 
/// This attribute macro automatically applies both attributes to a function component,
/// eliminating the need to manually specify both. The autoprops attribute removes the
/// need to create separate structs for component properties.
/// 
/// # Example
/// ```rust
/// use yew::prelude::*;
/// use yew_shortcuts::comp;
/// 
/// #[comp]
/// fn MyComponent(name: &str, #[prop_or_default] age: &u32) -> Html {
///     html! {
///         <div>
///             <p>{format!("Name: {}", name)}</p>
///             <p>{format!("Age: {}", age)}</p>
///         </div>
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn comp(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    let autoprops_attr = syn::Attribute {
        pound_token: syn::token::Pound::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket::default(),
        meta: Meta::Path(Path {
            leading_colon: None,
            segments: {
                let mut segments = Punctuated::new();
                segments.push(syn::PathSegment {
                    ident: Ident::new("yew_autoprops", Span::call_site()),
                    arguments: PathArguments::None,
                });
                segments.push(syn::PathSegment {
                    ident: Ident::new("autoprops", Span::call_site()),
                    arguments: PathArguments::None,
                });
                segments
            },
        }),
    };
    
    let comp_attr = syn::Attribute {
        pound_token: syn::token::Pound::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket::default(),
        meta: Meta::Path(Path {
            leading_colon: None,
            segments: {
                let mut segments = Punctuated::new();
                segments.push(syn::PathSegment {
                    ident: Ident::new("yew", Span::call_site()),
                    arguments: PathArguments::None,
                });
                segments.push(syn::PathSegment {
                    ident: Ident::new("function_component", Span::call_site()),
                    arguments: PathArguments::None,
                });
                segments
            },
        }),
    };

    input.attrs.insert(0, autoprops_attr);
    input.attrs.insert(1, comp_attr);

    TokenStream::from(quote::quote!(#input))
}
