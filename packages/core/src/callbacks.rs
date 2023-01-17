use napi::{Env, JsFunction, JsObject, JsUnknown, NapiValue, Ref};

pub struct CallbackStore {
    pub device_change: Option<CallbackRef>,
    pub volume_change: Option<CallbackRef>,
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
