use neon::prelude::*;

fn add(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    let a_var: Handle<JsNumber> = ctx.argument(0)?;
    let b_var: Handle<JsNumber> = ctx.argument(0)?;
    let a = a_var.value(&mut ctx);
    let b = b_var.value(&mut ctx);
    Ok(ctx.number(a + b))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("add", add)?;
    Ok(())
}
