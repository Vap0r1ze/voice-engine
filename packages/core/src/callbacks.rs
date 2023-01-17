use napi::{Env, JsFunction, JsObject, JsUnknown, NapiValue, Ref};

pub struct CallbackStore {
    pub device_change: Option<CallbackRef>,
    pub volume_change: Option<CallbackRef>,
}

pub struct CallbackRef {
    reference: Ref<()>,
}

impl CallbackRef {
    pub fn new(env: &Env, callback: JsFunction) -> napi::Result<CallbackRef> {
        let reference = env.create_reference::<JsFunction>(callback)?;
        Ok(CallbackRef { reference })
    }
    pub fn call<V: NapiValue>(
        &self,
        env: &Env,
        this: Option<&JsObject>,
        args: &[V],
    ) -> napi::Result<JsUnknown> {
        let value: JsFunction = env.get_reference_value(&self.reference)?;
        value.call(this, args)
    }
}
