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
    token, Ident, Result, Token, Type,
};

#[allow(clippy::large_enum_variant)]
enum Definition {
    Actor(Ident),
    ResponseType(Type),
    Name(Ident),
    Tag(Ident),
}
impl Parse for Definition {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = &name.to_string();
        let _: Token![:] = input.parse()?;

        if name_str == "actor" {
            Ok(Self::Actor(input.parse()?))
        } else if name_str == "response_type" {
            Ok(Self::ResponseType(input.parse()?))
        } else if name_str == "name" {
            Ok(Self::Name(input.parse()?))
        } else if name_str == "tag" {
            Ok(Self::Tag(input.parse()?))
        } else {
            panic!("{} is not a known configuration name", name_str)
        }
    }
}

struct Configuration {
    #[allow(dead_code)]
    conf_marker: Ident,
    #[allow(dead_code)]
    brace: token::Brace,
    definitions: Punctuated<Definition, Token![,]>,
}

impl Parse for Configuration {
    fn parse(input: ParseStream) -> Result<Self> {
        let definitions;

        Ok(Self {
            conf_marker: input.parse()?,
            brace: braced!(definitions in input),
            definitions: definitions.parse_terminated(Definition::parse)?,
        })
    }
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let Configuration { definitions, .. } = parse_macro_input!(input as Configuration);

    let mut actor: Option<&Ident> = None;
    let mut name: Option<&Ident> = None;
    let mut response_type: Option<&Type> = None;
    let mut tag: Option<&Ident> = None;
    for d in definitions.iter() {
        match d {
            Definition::Actor(ident) => {
                actor = Some(ident);
            }
            Definition::Name(ident) => {
                name = Some(ident);
            }
            Definition::ResponseType(ty) => {
                response_type = Some(ty);
            }
            Definition::Tag(ident) => {
                tag = Some(ident);
            }
        }
    }

    let actor = actor.unwrap();
    let actor_str = &actor.to_string();
    let actor_msg = Ident::new(format!("{}Msg", actor_str).as_str(), Span::call_site());
    let actor_lock = Ident::new(&actor_str.to_ascii_uppercase(), Span::call_site());

    let name = name.unwrap();
    let mut parser_str = String::from("extract_");
    parser_str.push_str(&name.to_string());
    let parser = Ident::new(&parser_str, Span::call_site());

    let tag = tag.unwrap();
    let tag_str = tag.to_string();
    let tag_msg = Ident::new(
        format!("{}{}Msg", actor_str, tag_str).as_str(),
        Span::call_site(),
    );

    let response_type = response_type.unwrap();

    (quote! {
    pub async fn #name(mut req: tide::Request<crate::state::ServerState>) -> tide::Result<tide::Response> {
        #[allow(clippy::unnecessary_mut_passed)]
        let payload = #parser(&mut req).await?;
        let (reply, r) = crossbeam_channel::bounded::<Option<crate::resources::ResponseData<#response_type>>>(1);
        let state = req.state();

        crate::messages::#actor_lock
            .get()
            .unwrap()
            .read()
            .as_ref()
            .unwrap()
            .send(crate::messages::#actor_msg::#tag(crate::messages::#tag_msg { reply, db_pool: state.global.db_pool, payload }))
            .unwrap();

        crossbeam_channel::select! {
          recv(r) -> msg => {
            match msg {
              Ok(Some(crate::resources::ResponseData(code, content))) => crate::resources::respond_with::<#response_type>(code, content),
              _ => crate::resources::respond_with::<#response_type>(tide::StatusCode::InternalServerError, None)
            }
          },
          default(std::time::Duration::from_secs(3)) => crate::resources::respond_with::<#response_type>(tide::StatusCode::RequestTimeout, None),
        }
    }
    })
    .into()
}
