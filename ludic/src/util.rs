use neon::prelude::{Context, JsArray, JsResult, Object};

pub fn convert_array<'a, C: Context<'a>>(ctx: &mut C, vec: Vec<String>) -> JsResult<'a, JsArray> {
    let arr = JsArray::new(ctx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let val = ctx.string(s);
        arr.set(ctx, i as u32, val)?;
    }

    return Ok(arr);
}
