use neon::{prelude::{FunctionContext, JsResult, Context, JsPromise}, result::NeonResult};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
use crate::util::{convert_map};

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    return RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())));
}

pub fn get_forge_versions(mut ctx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut ctx)?;
    let channel = ctx.channel();
    let (def, pro) = ctx.promise();

    rt.spawn(async move {
        let raw = forgic::get_forge_versions().await;

        def.settle_with(&channel, move |mut ctx| {
            let conv = convert_map(&mut ctx, raw);
            let res = conv.or_else(|err| ctx.throw_error(err.to_string()))?;

            match Option::from(res) {
                Some(res) => Ok(res),
                None => ctx.throw_error("Could not get forge releases!"),
            }
        });
    });

    return Ok(pro);
}
