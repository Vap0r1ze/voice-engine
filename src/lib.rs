use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn segfault(mut _cx: FunctionContext) -> JsResult<JsString> {
  segfault::segfault();
  unreachable!()
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("segfault", segfault)?;
    Ok(())
}
