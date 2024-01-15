use std::{collections::btree_map::Keys, ffi::CString, ptr};

use napi::{bindgen_prelude::*, sys::napi_value, Env, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use rspack_error::{
  miette::{self, Diagnostic},
  thiserror::{self, Error},
};

static JS_STACK_RE: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\s+at.*[(\s].*\)?").expect("should init regex"));

#[derive(Debug, Error, Diagnostic)]
#[diagnostic()]
#[error("{0}")]
pub struct NodeError(String);

pub trait NapiErrorExt {
  fn into_rspack_error_with_detail(self, env: &Env) -> NodeError;
}

pub trait NapiResultExt<T> {
  fn into_rspack_result(self, env: &Env) -> rspack_error::Result<T>;
  fn into_rspack_result_with_detail(self, env: &Env) -> rspack_error::Result<T, NodeError>;
}

impl NapiErrorExt for Error {
  fn into_rspack_result(self, env: &Env) -> rspack_error::Result<T> {}
  fn into_rspack_error_with_detail(self, env: &Env) -> NodeError {
    let reason = extract_stack_or_message_from_napi_error(env, self);
    NodeError(reason).into()
  }
}

impl<T: 'static> NapiResultExt<T> for Result<T> {
  fn into_rspack_result_with_detail(self, env: &Env) -> rspack_error::Result<T, NodeError> {
    self.map_err(|e| e.into_rspack_error_with_detail(env))
  }
}

/// Extract stack or message from a native Node error object,
/// otherwise we try to format the error from the given `Error` object that indicates which was created on the Rust side.
#[inline(always)]
fn extract_stack_or_message_from_napi_error(env: &Env, err: Error) -> String {
  if !err.reason.is_empty() {
    return err.reason;
  }

  match unsafe { ToNapiValue::to_napi_value(env.raw(), err) } {
    Ok(napi_error) => match try_extract_string_value_from_property(env, napi_error, "stack") {
      Err(_) => match try_extract_string_value_from_property(env, napi_error, "message") {
        Err(e) => format!("Unknown NAPI error {e}"),
        Ok(message) => message,
      },
      Ok(message) => message,
    },
    Err(e) => format!("Failed to extract NAPI error stack or message: {e}"),
  }
}

fn try_extract_string_value_from_property<S: AsRef<str>>(
  env: &Env,
  napi_object: napi_value,
  property: S,
) -> napi::Result<String> {
  let property = CString::new(property.as_ref())?;

  let mut value_ptr = ptr::null_mut();

  check_status!(
    unsafe {
      sys::napi_get_named_property(env.raw(), napi_object, property.as_ptr(), &mut value_ptr)
    },
    "Failed to get the named property from object (property: {property:?})"
  )?;

  let mut str_len = 0;
  check_status!(
    unsafe {
      sys::napi_get_value_string_utf8(env.raw(), value_ptr, ptr::null_mut(), 0, &mut str_len)
    },
    "Failed to get the length of the underlying property (property: {property:?})"
  )?;

  str_len += 1;
  let mut buf = Vec::with_capacity(str_len);
  let mut copied_len = 0;

  check_status!(
    unsafe {
      sys::napi_get_value_string_utf8(
        env.raw(),
        value_ptr,
        buf.as_mut_ptr(),
        str_len,
        &mut copied_len,
      )
    },
    "Failed to get value of the property (property: {property:?})"
  )?;

  let mut buf = std::mem::ManuallyDrop::new(buf);

  let buf = unsafe { Vec::from_raw_parts(buf.as_mut_ptr() as *mut u8, copied_len, copied_len) };

  Ok(String::from_utf8_lossy(&buf).into_owned())
}
