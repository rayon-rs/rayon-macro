use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;
use quote::quote;
use syn::fold::{self, Fold};
use syn::parse_macro_input;

/// Transform a traditional `for` loop into a parallel `for_each`
///
/// ```rust,ignore
/// parallel!(for item in expr { /* body */ })
/// ```
///
/// becomes roughly:
///
/// ```rust,ignore
/// expr.into_par_iter().for_each(|item| { /* body */ });
/// ```
#[proc_macro]
pub fn parallel(input: TokenStream) -> TokenStream {
    // for #pat in #expr { #body }
    let syn::ExprForLoop {
        attrs,
        label,
        pat,
        expr,
        body,
        ..
    } = parse_macro_input!(input);

    assert!(attrs.is_empty(), "loop attributes are not supported");

    let mut transform = TransformBody::new(label.as_ref());
    let body = transform.fold_block(body);

    let rayon_crate = crate_name("rayon").expect("rayon is present in `Cargo.toml`");
    let rayon = Ident::new(&rayon_crate, Span::call_site());

    TokenStream::from(quote! {
        ::#rayon::iter::ParallelIterator::for_each(
            ::#rayon::iter::IntoParallelIterator::into_par_iter(#expr),
            |#pat| #label while { #body; false } {},
        )
    })
}

struct TransformBody {
    for_ident: Option<syn::Ident>,
    loop_level: usize,
    try_level: usize,
}

impl TransformBody {
    fn new(for_label: Option<&syn::Label>) -> Self {
        let for_ident = for_label.map(|l| l.name.ident.clone());
        TransformBody {
            for_ident,
            loop_level: 0,
            try_level: 0,
        }
    }

    fn is_controlled(&self, label: &Option<syn::Lifetime>) -> bool {
        if let Some(label) = label {
            self.for_ident.as_ref() == Some(&label.ident)
        } else {
            self.loop_level == 0
        }
    }

    fn take_shadowed(&mut self, label: &Option<syn::Label>) -> Option<syn::Ident> {
        if let Some(label) = label {
            if self.for_ident.as_ref() == Some(&label.name.ident) {
                return self.for_ident.take();
            }
        }
        None
    }
}

impl Fold for TransformBody {
    // We don't care about other items defined in our block
    fn fold_item(&mut self, i: syn::Item) -> syn::Item {
        i
    }

    fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
        use syn::Expr::*;
        match i {
            // Control flow doesn't escape closures, so skip them.
            Async(_) | Closure(_) => return i,

            // Inner loops might intercept `break` and `continue`
            ForLoop(syn::ExprForLoop { ref label, .. })
            | Loop(syn::ExprLoop { ref label, .. })
            | While(syn::ExprWhile { ref label, .. }) => {
                let shadowed = self.take_shadowed(label);
                self.loop_level += 1;
                let ret = fold::fold_expr(self, i);
                self.loop_level -= 1;
                if shadowed.is_some() {
                    self.for_ident = shadowed;
                }
                ret
            }

            Continue(c) => {
                if self.is_controlled(&c.label) {
                    Return(syn::ExprReturn {
                        attrs: c.attrs,
                        return_token: syn::token::Return {
                            span: c.continue_token.span,
                        },
                        expr: None,
                    })
                } else {
                    Continue(fold::fold_expr_continue(self, c))
                }
            }

            Break(b) => {
                if self.is_controlled(&b.label) {
                    panic!("`break` is not supported yet")
                } else {
                    Break(fold::fold_expr_break(self, b))
                }
            }

            Return(_) => panic!("`return` is not supported yet"),

            Try(_) if self.try_level == 0 => panic!("`?` is not supported yet"),
            TryBlock(_) => {
                self.try_level += 1;
                let ret = fold::fold_expr(self, i);
                self.try_level -= 1;
                ret
            }

            Await(_) => panic!("`.await` is not supported"),
            Yield(_) => panic!("`yield` is not supported"),

            _ => fold::fold_expr(self, i),
        }
    }
}
