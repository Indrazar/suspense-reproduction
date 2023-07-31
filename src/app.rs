use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/suspense-reproduction.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> } /> //ssr=SsrMode::Async />
                    <Route path="/page2" view=|| view! { <Page2/> } ssr=SsrMode::Async />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let send_strings = create_server_action::<SendStrings>();
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <A href="/page2">"Go to Page 2"</A>
        <ActionForm action=send_strings>
            <p><InnerComponent/></p>
            <p>"Outer Component"</p>
            <p><label for="input2">"Input2"</label><input type="text" name="input2" required/></p>
            <p><label for="input3">"Input3"</label><input type="text" name="input3" required/></p>
            <p><label for="input4">"Input4"</label><input type="text" name="input4" required/></p>
            <input type="submit" value="Send Strings"/>
        </ActionForm>
    }
}

#[component]
fn InnerComponent() -> impl IntoView {
    let suspense_string_resource = create_resource(
        move || (),
        move |_| {
            log::trace!("Suspense String retriever running fetcher");
            issue_suspense_string()
        },
    );
    let transition_string_resource = create_resource(
        move || (),
        move |_| {
            log::trace!("Transition String retriever running fetcher");
            issue_transition_string()
        },
    );
    view! {
        <p>"Suspense Test"</p>
        <Suspense fallback=move || view!{<>"Loading"</>}>
            {move || {
                suspense_string_resource.read().map(|n| match n {
                    Err(e) => view! {<>{format!("Load Failed {e}")}</>},
                    Ok(value) => view! {<><p><DeepInner /></p><p><input type="text" readonly name="input" value=value/></p></>},
                })
            }}
        </Suspense>
        <p>"Transition Test"</p>
        <Transition fallback=move || view!{<>"Loading"</>}>
            {move || {
                transition_string_resource.read().map(|n| match n {
                    Err(e) => view! {<>{format!("Load Failed {e}")}</>},
                    Ok(value) => view! {<><p><DeepInner /></p><p><input type="text" readonly name="input" value=value/></p></>},
                })
            }}
        </Transition>
    }
}

#[component]
fn Page2() -> impl IntoView {
    view! {
        <A href="/">"Go back to Home"</A>
    }
}

#[component]
fn DeepInner() -> impl IntoView {
    let string_ref: StoredValue<String> = store_value(String::default());
    on_cleanup(move || log!("string_ref = {}", string_ref.get_value()));
    view! {}
}

#[server(SendStrings, "/api")]
pub async fn send_strings(
    input: String,
    input2: String,
    input3: String,
    input4: String,
) -> Result<(), ServerFnError> {
    log::trace!("Strings: {input}, {input2}, {input3}, {input4}");
    Ok(())
}

#[server(IssueSuspenseString, "/api")]
async fn issue_suspense_string() -> Result<String, ServerFnError> {
    std::thread::sleep(std::time::Duration::from_millis(450));
    Ok(String::from("This is a Suspense String"))
}

#[server(IssueTransitionString, "/api")]
async fn issue_transition_string() -> Result<String, ServerFnError> {
    std::thread::sleep(std::time::Duration::from_millis(450));
    Ok(String::from("This is a Transition String"))
}
