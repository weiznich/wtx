mod attrs;

use crate::{misc::create_ident, owned_or_ref::OwnedOrRef, transport_group::TransportGroup};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{
  parse_macro_input, punctuated::Punctuated, spanned::Spanned, AttributeArgs, Item, Path,
  PathArguments, PathSegment,
};

pub(crate) fn api_types(
  attrs: proc_macro::TokenStream,
  token_stream: proc_macro::TokenStream,
) -> crate::Result<proc_macro::TokenStream> {
  let attr_args = parse_macro_input::parse::<AttributeArgs>(attrs)?;
  let mut item: Item = syn::parse_macro_input::parse(token_stream)?;

  let attrs::Attrs { pkgs_aux, transports } = attrs::Attrs::try_from(&*attr_args)?;

  let pkgs_aux_path = pkgs_aux.map_or_else(
    || {
      let mut segments = Punctuated::new();
      segments.push(PathSegment {
        ident: Ident::new("PkgsAux", Span::mixed_site()),
        arguments: PathArguments::None,
      });
      OwnedOrRef::Owned(Path { leading_colon: None, segments })
    },
    OwnedOrRef::Ref,
  );

  let api_ident = match &mut item {
    Item::Enum(container) => &mut container.ident,
    Item::Struct(container) => &mut container.ident,
    Item::Type(container) => &mut container.ident,
    Item::Const(_)
    | Item::ExternCrate(_)
    | Item::Fn(_)
    | Item::ForeignMod(_)
    | Item::Impl(_)
    | Item::Macro(_)
    | Item::Macro2(_)
    | Item::Mod(_)
    | Item::Static(_)
    | Item::Trait(_)
    | Item::TraitAlias(_)
    | Item::Union(_)
    | Item::Use(_)
    | Item::Verbatim(_)
    | _ => return Err(crate::Error::NoEnumStructOrType(item.span())),
  };

  let mut buffer = String::new();
  buffer.push_str(&api_ident.to_string());

  let generic_pair_ident = create_ident(&mut buffer, ["Pair"]);
  let generic_pair_tt = quote::quote_spanned!(api_ident.span() =>
    #[allow(unused_qualifications)]
    #[doc = concat!("[wtx::misc::Pair] with [", stringify!(#api_ident), "] as the API.")]
    pub type #generic_pair_ident<DRSR, T> = wtx::misc::Pair<
      #pkgs_aux_path<#api_ident, DRSR, <T as wtx::network::transport::Transport<DRSR>>::Params>,
      T
    >;
  );

  let generic_pkgs_aux_ident = create_ident(&mut buffer, ["PkgsAux"]);
  let generic_pkgs_aux_tt = quote::quote_spanned!(api_ident.span() =>
    #[doc = concat!("[", stringify!(#pkgs_aux_path), "] with [", stringify!(#api_ident), "] as the API.")]
    pub type #generic_pkgs_aux_ident<DRSR, TP> = #pkgs_aux_path<#api_ident, DRSR, TP>;
  );

  let mut tys = Vec::new();
  let mut custom_placeholder;

  for transport in transports {
    let [camel_abbr, params] = match transport {
      TransportGroup::Custom(tt) => {
        custom_placeholder = tt.to_string();
        [custom_placeholder.as_str(), custom_placeholder.as_str()]
      }
      TransportGroup::Http => ["Http", "HttpParams"],
      TransportGroup::Stub => ["Stub", "()"],
      TransportGroup::Tcp => ["Tcp", "TcpParams"],
      TransportGroup::Udp => ["Udp", "UdpParams"],
      TransportGroup::WebSocket => ["Ws", "WsParams"],
    };
    let local_tp_ident = Ident::new(params, Span::mixed_site());
    let local_ty_ident = create_ident(&mut buffer, [camel_abbr, "PkgsAux"]);
    tys.push(quote::quote!(
      #[allow(unused_qualifications)]
      #[doc = concat!(
        "[", stringify!(#pkgs_aux_path), "] with [",
        stringify!(#api_ident),
        "] as the API and [wtx::network::",
        stringify!(#local_tp_ident),
        "] as the transport parameters."
      )]
      pub type #local_ty_ident<DRSR> = #pkgs_aux_path<#api_ident, DRSR, wtx::network::#local_tp_ident>;
    ));
  }

  Ok(
    quote::quote!(
      #item
      #generic_pair_tt
      #generic_pkgs_aux_tt
      #(#tys)*
    )
    .to_token_stream()
    .into(),
  )
}
