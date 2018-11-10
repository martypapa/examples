#[macro_use]
extern crate neon;

use neon::prelude::*;

fn convert_vec_to_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let vec: Vec<String> = Vec::with_capacity(100);

    // Create the JS array
    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    // Iterate over the rust Vec and map each value in the Vec to the JS array
    for (i, obj) in vec.iter().enumerate() {
        let js_string = cx.string(obj);
        let _  = js_array.set(&mut cx, i as u32, js_string);
    }

    Ok(js_array)
}

fn return_js_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    Ok(cx.empty_array())
}

fn return_js_array_with_number(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let n = cx.number(9000.0);
    array.set(&mut cx, 0, n)?;
    Ok(array)
}

fn return_js_array_with_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let s = cx.string("hello node");
    array.set(&mut cx, 0, s)?;
    Ok(array)
}

register_module!(mut cx, {
    cx.export_function("convertVecToArray", convert_vec_to_array)?;
    cx.export_function("returnJsArray", return_js_array)?;
    cx.export_function("returnJsArrayWithNumber", return_js_array_with_number)?;
    cx.export_function("returnJsArrayWithString", return_js_array_with_string)?;
    Ok(())
});
