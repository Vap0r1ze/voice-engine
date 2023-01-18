use napi::{Env, JsFunction, JsObject, JsUnknown, NapiValue, Ref};

#[derive(Default)]
pub struct CoreCallbackStore {
    pub device_change: Option<CallbackRef>,
    pub volume_change: Option<CallbackRef>,
    pub video_input_init: Option<CallbackRef>,
}

pub struct CallbackRef {
    env: Env,
    pub reference: Ref<()>,
}

impl CallbackRef {
    pub fn new(env: Env, callback: JsFunction) -> napi::Result<CallbackRef> {
        let reference = env.create_reference::<JsFunction>(callback)?;
        Ok(CallbackRef { reference, env })
    }
    pub fn call<V: NapiValue>(
        &self,
        this: Option<&JsObject>,
        args: &[V],
    ) -> napi::Result<JsUnknown> {
        let value: JsFunction = self.env.get_reference_value(&self.reference)?;
        value.call(this, args)
    }
}

impl Drop for CallbackRef {
    fn drop(&mut self) {
        if let Err(e) = self.reference.unref(self.env) {
            eprintln!("Error unrefing callback: {}", e);
        }
    }
}

/// ### Usage
/// ```rs
/// define_callback!(Struct, method_name, callback_key)
/// ```
/// ### Description
/// Implements a `#[napi]` method on a `#[napi]` struct.
/// The method takes a `JsFunction` and saves the respective `CallbackRef` to `self.callbacks.${callback_key}`
#[macro_export]
macro_rules! define_callback {
    ( $Type:ident, $func:ident, $key:ident ) => {
        #[napi]
        impl $Type {
            #[napi]
            pub fn $func(&mut self, env: Env, callback: JsFunction) -> napi::Result<()> {
                self.callbacks.$key = Some(CallbackRef::new(env, callback)?);
                Ok(())
            }
        }
    };
}
