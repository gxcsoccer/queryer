// use anyhow::Result;
use neon::prelude::*;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

fn query(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let text = cx.argument::<JsString>(0)?.value(&mut cx);

    // deferred.resolve(&mut cx, cx.undefined());
    // Ok(promise)

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    // Create a JavaScript promise and a `deferred` handle for resolving it.
    // It is important to be careful not to perform failable actions after
    // creating the promise to avoid an unhandled rejection.
    let (deferred, promise) = cx.promise();

    // Spawn an `async` task on the tokio runtime. Only Rust types that are
    // `Send` may be moved into this block. `Context` may not be passed and all
    // JavaScript values must first be converted to Rust types.
    //
    // This task will _not_ block the JavaScript main thread.
    rt.spawn(async move {
        // Inside this block, it is possible to `await` Rust `Future`
        let mut df = queryer::query(text).await.unwrap();

        // Settle the promise from the result of a closure. JavaScript exceptions
        // will be converted to a Promise rejection.
        //
        // This closure will execute on the JavaScript main thread. It should be
        // limited to converting Rust types to JavaScript values. Expensive operations
        // should be performed outside of it.
        deferred.settle_with(&channel, move |mut cx| {
            // Convert a `reqwest::Error` to a JavaScript exception
            Ok(cx.string(df.to_csv().unwrap()))
        });
    });
    // Return the promise back to JavaScript
    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("query", query)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
        let sql = format!(
            "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
          FROM {} where new_deaths >= 100 ORDER BY new_cases DESC, new_deaths DESC",
            url
        );
        // let df = query(sql).await;
        // println!("{:?}", df);

        // assert_eq!(result, 4);
    }
}
