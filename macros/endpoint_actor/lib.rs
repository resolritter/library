#![allow(clippy::eval_order_dependence)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Ident, Result, Token,
};

struct Delegation {
    variant: Ident,
    #[allow(dead_code)]
    colon: Token![:],
    function: Ident,
}
impl Parse for Delegation {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            variant: input.parse()?,
            colon: input.parse()?,
            function: input.parse()?,
        })
    }
}

enum Definition {
    Actor(Ident),
}
impl Parse for Definition {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = &name.to_string();
        let _: Token![:] = input.parse()?;

        if name_str == "actor" {
            Ok(Self::Actor(input.parse()?))
        } else {
            panic!("{} is not a known configuration name", name_str)
        }
    }
}

struct Configuration {
    #[allow(dead_code)]
    first_brace: token::Brace,
    definitions: Punctuated<Definition, Token![,]>,
    #[allow(dead_code)]
    comma: Token![,],
    #[allow(dead_code)]
    second_brace: token::Brace,
    delegations: Punctuated<Delegation, Token![,]>,
}

impl Parse for Configuration {
    fn parse(input: ParseStream) -> Result<Self> {
        let definitions;
        let delegations;

        Ok(Self {
            first_brace: braced!(definitions in input),
            definitions: definitions.parse_terminated(Definition::parse)?,
            comma: input.parse()?,
            second_brace: braced!(delegations in input),
            delegations: delegations.parse_terminated(Delegation::parse)?,
        })
    }
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let Configuration {
        definitions,
        delegations,
        ..
    } = parse_macro_input!(input as Configuration);

    let actor_name = match definitions.iter().next().unwrap() {
        Definition::Actor(ident) => ident.to_string(),
    };

    let actor_lock = Ident::new(&actor_name.to_ascii_uppercase(), Span::call_site());
    let actor = Ident::new(&actor_name, Span::call_site());
    let actor_msg = Ident::new(format!("{}Msg", &actor_name).as_str(), Span::call_site());

    let matches = delegations.iter().map(
        |Delegation {
             variant, function, ..
         }| {
            quote! {
                #variant(msg) => {
                    let _ =
                        msg.reply.send(match #function(&msg).await {
                            Ok(output) => Some(output),
                            err => {
                                log::error!("{:#?}", err);
                                None
                            }
                        });
                }
            }
        },
    );

    (quote! {
        pub fn actor(children: bastion::prelude::Children) -> bastion::prelude::Children {
            children
                .with_name(ActorGroups::#actor.as_ref())
                .with_exec(move |_| async move {
                    let (channel, r) = crossbeam_channel::unbounded::<#actor_msg>();
                    {
                        let mut lock = #actor_lock.get().unwrap().write();
                        *lock = Some(channel);
                    }

                    loop {
                        match logged(r.recv().unwrap()) {
                            #(#matches),*
                        }
                    }

                    #[allow(unreachable_code)]
                    Ok(())
                })
        }
    })
    .into()
}
