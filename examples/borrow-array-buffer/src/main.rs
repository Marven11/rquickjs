use rquickjs::{ArrayBuffer, Context, Runtime};

fn main() {
    let vec = vec![11u8, 45, 14, 19, 19, 81, 0];
    let slice = &vec;
    {
        let runtime = Runtime::new().unwrap();
        let context = Context::full(&runtime).unwrap();
        {
            context.with(|ctx| {
                let global = ctx.globals();
                let array_buffer = ArrayBuffer::new_borrow(ctx, slice).unwrap();
                global.set("buffer", array_buffer).unwrap();
            });
        }

        context.with(|ctx| {
            let result: String = ctx
                .eval(
                    r#"
            const arr = new Uint8Array(buffer);
            "ArrayBuffer content: [" + Array.from(arr).join(", ") + "]"
        "#,
                )
                .unwrap();

            println!("{}", result);
        })
    }
}
