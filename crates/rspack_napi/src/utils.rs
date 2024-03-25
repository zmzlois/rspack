use std::marker::PhantomData;

use napi::{
  bindgen_prelude::{FromNapiValue, Function},
  sys::{self, napi_value},
  Env, JsObject, JsString, JsStringUtf8, JsUnknown, NapiRaw, NapiValue, Result,
};

/// `Object.prototype.toString.call`
/// Safety: [napi::JsStringUtf8]'s lifetime is bound to `&T`
unsafe fn object_prototype_to_string_call(
  raw_env: sys::napi_env,
  obj: napi_value,
) -> Result<JsStringUtf8> {
  let env = Env::from(raw_env);
  let s: JsString = env
    .get_global()?
    // `Object` is a function, but we want to use it as an JSObject.
    .get_named_property_unchecked::<JsObject>("Object")?
    .get_named_property::<JsObject>("prototype")?
    .get_named_property::<Function>("toString")?
    .call(
      // Safety: `T` is with constraint of `NapiRaw`
      unsafe { JsUnknown::from_raw_unchecked(raw_env, obj) },
    )?
    .try_into()?;
  s.into_utf8()
}

pub struct NapiTypeRef<'r>(String, PhantomData<&'r *mut ()>);

impl FromNapiValue for NapiTypeRef<'_> {
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    let s = unsafe { object_prototype_to_string_call(env, napi_val) }?
      .as_str()?
      .to_string();
    Ok(Self(s, PhantomData))
  }
}

impl<'r> NapiTypeRef<'r> {
  pub fn get_type(&self) -> &str {
    &self.0
  }

  pub fn is_regex(&self) -> Result<bool> {
    Ok(self.get_type() == "[object RegExp]")
  }
}

pub fn downcast_into<T: FromNapiValue + 'static>(o: JsUnknown) -> napi::Result<T> {
  <T as FromNapiValue>::from_unknown(o)
}
