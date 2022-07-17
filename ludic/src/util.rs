use std::collections::HashMap;

use neon::prelude::{Context, JsArray, JsResult, Object, JsObject};

pub fn convert_array<'a, C: Context<'a>>(ctx: &mut C, vec: Vec<String>) -> JsResult<'a, JsArray> {
    let arr = JsArray::new(ctx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let val = ctx.string(s);
        arr.set(ctx, i as u32, val)?;
    }

    return Ok(arr);
}

pub fn convert_map<'a, C: Context<'a>>(ctx: &mut C, map: HashMap<String, Vec<String>>) -> JsResult<'a, JsObject> {
    let obj = JsObject::new(ctx);

    for (_i, s) in map.iter().enumerate() {
        let key = ctx.string(s.0);
        let val = convert_array(ctx, s.1.clone())?;
        obj.set(ctx, key, val)?;
    }

    return Ok(obj);
}
