#![allow(clippy::eval_order_dependence)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Ident, Result, Token, Type,
};

struct MessageInner {
    name: Ident,
    #[allow(dead_code)]
    first_comma: token::Comma,
    reply: Type,
}
impl Parse for MessageInner {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MessageInner {
            name: input.parse()?,
            first_comma: input.parse()?,
            reply: input.parse()?,
        })
    }
}
struct Message {
    name: Ident,
    reply: Type,
}
impl Parse for Message {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        for parsed in content.parse_terminated::<MessageInner, token::Comma>(MessageInner::parse)? {
            return Ok(Message {
                name: parsed.name,
                reply: parsed.reply,
            });
        }
        unreachable!();
    }
}

struct Configuration {
    resource: Ident,
    #[allow(dead_code)]
    first_comma: token::Comma,
    #[allow(dead_code)]
    first_bracket: token::Bracket,
    messages: Punctuated<Message, Token![,]>,
}

impl Parse for Configuration {
    fn parse(input: ParseStream) -> Result<Self> {
        let messages;

        Ok(Self {
            resource: input.parse()?,
            first_comma: input.parse()?,
            first_bracket: bracketed!(messages in input),
            messages: messages.parse_terminated(Message::parse)?,
        })
    }
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let Configuration {
        resource, messages, ..
    } = parse_macro_input!(input as Configuration);

    let actor_name = resource.to_string();
    let actor_lock = Ident::new(&actor_name.to_ascii_uppercase(), Span::call_site());
    let actor_msg = Ident::new(format!("{}Msg", actor_name).as_str(), Span::call_site());

    let msg_structs = messages.iter().map(|m| {
        let name_str = m.name.to_string();
        let msg = Ident::new(format!("{}{}Msg", actor_name, name_str).as_str(), Span::call_site());
        let payload = Ident::new(format!("{}{}Payload", actor_name, name_str).as_str(), Span::call_site());
        let reply = &m.reply;

        quote! {
            #[derive(Debug)]
            pub struct #msg {
                pub reply: crossbeam_channel::Sender<Option<crate::resources::ResponseData<#reply>>>,
                pub payload: crate::entities::#payload,
                pub db_pool: &'static sqlx::PgPool,
            }
        }
    });

    let msg_variants = messages.iter().map(|m| {
        let name = &m.name;
        let msg = Ident::new(
            format!("{}{}Msg", actor_name, name).as_str(),
            Span::call_site(),
        );
        quote! {
            #name(#msg)
        }
    });

    (quote! {
        #(#msg_structs)*

        #[derive(Debug)]
        pub enum #actor_msg {
            #(#msg_variants),*
        }
        
        pub static #actor_lock: once_cell::sync::OnceCell<&'static parking_lot::RwLock<Option<crossbeam_channel::Sender<#actor_msg>>>> = once_cell::sync::OnceCell::new();
    })
    .into()
}
