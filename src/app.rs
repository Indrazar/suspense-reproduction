use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/suspense-reproduction.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> } ssr=SsrMode::Async/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let send_strings = create_server_action::<SendStrings>(cx);
    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <ActionForm action=send_strings>
            <p><InnerComponent/></p>
            <p><label for="input2">"Input2"</label><input type="text" name="input2" required/></p>
            <p><label for="input3">"Input3"</label><input type="text" name="input3" required/></p>
            <p><label for="input4">"Input4"</label><input type="text" name="input4" required/></p>
            <input type="submit" value="Send Strings"/>
        </ActionForm>
    }
}

#[component]
fn InnerComponent(cx: Scope) -> impl IntoView {
    let string_action = create_server_action::<IssueString>(cx);
    let string_resource = create_resource(
        cx,
        move || (string_action.version().get()),
        move |_| {
            log::trace!("String retriever running fetcher");
            issue_csrf(cx)
        },
    );
    view! { cx,
      <Suspense fallback=move || view!{cx, <>"Loading"</>}>
      {move || {
          string_resource.read(cx).map(|n| match n {
              Err(_) => view! {cx, <>"Load Failed"</>},
              Ok(value) => view! {cx, <><input type="text" readonly name="input" value=value/></>},
          })
      }}
    </Suspense>
    }
}

#[server(SendStrings, "/api")]
pub async fn send_strings(
    cx: Scope,
    input: String,
    input2: String,
    input3: String,
    input4: String,
) -> Result<(), ServerFnError> {
    Ok(())
}

#[server(IssueString, "/api")]
async fn issue_csrf(cx: Scope) -> Result<String, ServerFnError> {
    std::thread::sleep(std::time::Duration::from_millis(450));
    Ok(String::from("This is a String"))
}
