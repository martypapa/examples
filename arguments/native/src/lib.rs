#[macro_use]
extern crate neon;

use neon::prelude::*;

// Creating a function that takes a function and prints it
fn print_function(mut cx: FunctionContext) -> JsResult<JsFunction> {
    let arg0 = cx.argument::<JsFunction>(0)?;
    Ok(arg0)
}

// Creating a function that checks if arguments of the expected types
// are passed to it. Will throw exception if arguemts are not passed
// or if they are of wrong type.
fn checking_arguments(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cx.check_argument::<JsString>(0)?;
    cx.check_argument::<JsNumber>(1)?;
    Ok(cx.undefined())
}

// Create a function that takes an argument that must be a number,
// add 1 to that number and then return it
fn add1(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // Attempt to cast the first argument to a JsNumber. Then
    // get the value if cast is successul
    let x = cx.argument::<JsNumber>(0)?.value();
    Ok(cx.number(x + 1.0))
}

// Create a function that gets the number of arguments passed to it
fn get_args_len(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let args_length = cx.len();
    println!("{}", args_length);
    Ok(cx.number(args_length))
}

// Creating a function that has optional arguments
fn args_opt(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match cx.argument_opt(0) {
        Some(arg) => {
            // Throw if the argument exist and it cannot be downcasted
            // to a number
            let num = arg.downcast::<JsNumber>().or_throw(&mut cx)?.value();
            println!("The 0th argument is {}", num);
        },
        None => panic!("0th argument does not exist, out of bounds!")
    }
    Ok(cx.undefined())
}

// Create functions that have default arguments:
fn default_args(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let age = match cx.argument_opt(0) {
        Some(arg) => arg.downcast::<JsNumber>().or_throw(&mut cx)?.value(),
        // Default to 12 if no value is given
        None => 12 as f64
    };

    let name = match cx.argument_opt(1) {
        Some(arg) => arg.downcast::<JsString>().or_throw(&mut cx)?.value(),
        // Default to 12 if no value is given
        None => "John Doe".to_string()
    };

    println!("i am {} years old and my name is {}", age, name);

    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("printFunction", print_function)?;
    cx.export_function("checkingArguments", checking_arguments)?;
    cx.export_function("add1", add1)?;
    cx.export_function("getArgsLen", get_args_len)?;
    cx.export_function("argsOpt", args_opt)?;
    cx.export_function("defaultArgs", default_args)?;
    Ok(())
});
