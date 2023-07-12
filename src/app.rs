use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/personal_site.css"/>

        // sets the document title
        <Title text="Ethan Corgatelli"/>

        // Google fonts embed
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        // <button on:click=on_click>"Click Me: " {count}</button>
        <ProjectsWindow/>
        <Footer/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! {cx,
        <footer>
            <div class="win-minimized">"Window"</div>
        </footer>
    }
}

#[component]
fn Window(cx: Scope, window_title: &'static str) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(cx, false);
    let close_window = move |_| set_hidden.update(|hidden| *hidden = !*hidden);

    view! { cx,
        <div class="win-outer" class:hidden={hidden}>
            <div class="win-titlebar">
                <p>{window_title}</p>
                <a class="win-close" on:click=close_window></a>
            </div>
            <div class="win-content">
                <p>"This is a window!"</p>
            </div>
        </div>
    }
}

#[component]
fn ProjectsWindow(cx: Scope) -> impl IntoView {
    view! { cx,
        <Window window_title="Projects"/>
    }
}
