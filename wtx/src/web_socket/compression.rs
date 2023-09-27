//! https://datatracker.ietf.org/doc/html/rfc7692

mod compression_level;
mod deflate_config;
#[cfg(feature = "flate2")]
mod flate2;
mod window_bits;

pub use compression_level::CompressionLevel;
pub use deflate_config::DeflateConfig;
#[cfg(feature = "flate2")]
pub use flate2::{Flate2, NegotiatedFlate2};
pub use window_bits::WindowBits;

/// Initial compression parameters defined before a handshake.
pub trait Compression<const IS_CLIENT: bool> {
  /// See [NegotiatedCompression].
  type Negotiated: NegotiatedCompression;

  /// Manages the defined parameters with the received parameters to decide which
  /// parameters will be settled.
  #[cfg(feature = "web-socket-handshake")]
  fn negotiate(
    self,
    headers: &[crate::http_structs::Header<'_>],
  ) -> crate::Result<Self::Negotiated>;

  /// Writes headers bytes that will be sent to the server.
  fn write_req_headers<B>(&self, buffer: &mut B)
  where
    B: Extend<u8>;
}

impl<const IS_CLIENT: bool> Compression<IS_CLIENT> for () {
  type Negotiated = ();

  #[cfg(feature = "web-socket-handshake")]
  #[inline]
  fn negotiate(self, _: &[crate::http_structs::Header<'_>]) -> crate::Result<Self::Negotiated> {
    Ok(())
  }

  #[inline]
  fn write_req_headers<B>(&self, _: &mut B)
  where
    B: Extend<u8>,
  {
  }
}

/// Final compression parameters defined after a handshake.
pub trait NegotiatedCompression {
  fn compress<O>(
    &mut self,
    input: &[u8],
    output: &mut O,
    begin_cb: impl FnMut(&mut O) -> &mut [u8],
    rem_cb: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize>;

  fn decompress<O>(
    &mut self,
    input: &[u8],
    output: &mut O,
    begin_cb: impl FnMut(&mut O) -> &mut [u8],
    rem_cb: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize>;

  fn rsv1(&self) -> u8;

  fn write_res_headers<B>(&self, buffer: &mut B)
  where
    B: Extend<u8>;
}

impl<T> NegotiatedCompression for &mut T
where
  T: NegotiatedCompression,
{
  #[inline]
  fn compress<O>(
    &mut self,
    input: &[u8],
    output: &mut O,
    begin_cb: impl FnMut(&mut O) -> &mut [u8],
    rem_cb: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize> {
    (**self).compress(input, output, begin_cb, rem_cb)
  }

  #[inline]
  fn decompress<O>(
    &mut self,
    input: &[u8],
    output: &mut O,
    begin_cb: impl FnMut(&mut O) -> &mut [u8],
    rem_cb: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize> {
    (**self).decompress(input, output, begin_cb, rem_cb)
  }

  #[inline]
  fn rsv1(&self) -> u8 {
    (**self).rsv1()
  }

  #[inline]
  fn write_res_headers<B>(&self, buffer: &mut B)
  where
    B: Extend<u8>,
  {
    (**self).write_res_headers(buffer);
  }
}

impl NegotiatedCompression for () {
  #[inline]
  fn compress<O>(
    &mut self,
    _: &[u8],
    _: &mut O,
    _: impl FnMut(&mut O) -> &mut [u8],
    _: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize> {
    Ok(0)
  }

  #[inline]
  fn decompress<O>(
    &mut self,
    _: &[u8],
    _: &mut O,
    _: impl FnMut(&mut O) -> &mut [u8],
    _: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize> {
    Ok(0)
  }

  #[inline]
  fn rsv1(&self) -> u8 {
    0
  }

  #[inline]
  fn write_res_headers<B>(&self, _: &mut B)
  where
    B: Extend<u8>,
  {
  }
}

impl<T> NegotiatedCompression for Option<T>
where
  T: NegotiatedCompression,
{
  #[inline]
  fn compress<O>(
    &mut self,
    input: &[u8],
    output: &mut O,
    begin_cb: impl FnMut(&mut O) -> &mut [u8],
    rem_cb: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize> {
    match self {
      Some(el) => el.compress(input, output, begin_cb, rem_cb),
      None => ().compress(input, output, begin_cb, rem_cb),
    }
  }

  #[inline]
  fn decompress<O>(
    &mut self,
    input: &[u8],
    output: &mut O,
    begin_cb: impl FnMut(&mut O) -> &mut [u8],
    rem_cb: impl FnMut(&mut O, usize) -> &mut [u8],
  ) -> crate::Result<usize> {
    match self {
      Some(el) => el.decompress(input, output, begin_cb, rem_cb),
      None => ().decompress(input, output, begin_cb, rem_cb),
    }
  }

  #[inline]
  fn rsv1(&self) -> u8 {
    match self {
      Some(el) => el.rsv1(),
      None => ().rsv1(),
    }
  }

  #[inline]
  fn write_res_headers<B>(&self, buffer: &mut B)
  where
    B: Extend<u8>,
  {
    match self {
      Some(el) => el.write_res_headers(buffer),
      None => ().write_res_headers(buffer),
    }
  }
}
