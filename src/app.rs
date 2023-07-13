use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/personal_site.css"/>

        <Title text="Ethan Corgatelli"/>

        // Google fonts embed
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        <Link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>

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

/// Renders home page
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <AboutWindow/>
        <ProjectsWindow/>
        <EducationWindow/>
        <Footer/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"404: Page Not Found"</h1>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    let about_hidden = move || false;

    view! {cx,
        <footer>
            <div class="win-minimized" class:hidden={about_hidden}>"About Me"</div>
            <div class="win-minimized" class:hidden={false}>"Projects"</div>
            <div class="win-minimized" class:hidden={false}>"Education"</div>
        </footer>
    }
}

#[component]
fn Window(
    cx: Scope,
    window_id: &'static str,
    window_title: &'static str,
    window_content: HtmlElement<html::Div>,
    window_dimensions: (i32, i32),
    start_pos: (i32, i32),
) -> impl IntoView {
    let (left, set_left) = create_signal(cx, start_pos.0);
    let (top, set_top) = create_signal(cx, start_pos.1);
    let (dragging, set_dragging) = create_signal(cx, false);
    let (hidden, set_hidden) = create_signal(cx, false);

    let close_window = move |_| set_hidden.update(|hidden| *hidden = !*hidden);
    let start_dragging = move |_| set_dragging.set(true);
    let drag = move |event: MouseEvent| {
        if dragging.get() {
            let (window_width, window_height) = (
                leptos::window().inner_width().unwrap().as_f64().unwrap() as i32,
                leptos::window().inner_height().unwrap().as_f64().unwrap() as i32,
            );
            set_left.set(
                (left.get() + event.movement_x())
                    .max(0)
                    .min(window_width - window_dimensions.0 - 30),
            );
            set_top.set(
                (top.get() + event.movement_y())
                    .max(0)
                    .min(window_height - window_dimensions.1 - 69),
            );
        }
    };
    let stop_dragging = move |_| {
        set_dragging.set(false);
    };
    let get_pos = move || format!("top: {}px; left: {}px", top.get(), left.get());
    let get_dim = move || {
        format!(
            "width: {}px; height: {}px",
            window_dimensions.0, window_dimensions.1
        )
    };

    view! { cx,
        <div
            id=window_id
            class="win-outer"
            class:hidden={hidden}
            style=get_pos>
            <div
                class="win-titlebar"
                on:mousedown=start_dragging
                on:mousemove=drag
                on:mouseleave=stop_dragging
                on:mouseup=stop_dragging>
                <p>{window_title}</p>
                <a
                    class="win-close"
                    on:mousedown=close_window></a>
            </div>
            <div
                class="win-content"
                style=get_dim>
                {window_content}
            </div>
        </div>
    }
}

#[component]
fn ProjectsWindow(cx: Scope) -> impl IntoView {
    let content = view! {cx,
        <div>"PROJECTS:"<ul>
        <li><a href="_assets/pdfs/PDF.pdf">"Project 1"</a></li>
        <li>"Project 2"</li>
        <li>"Project 3"</li>
        <li>"Project 4"</li>
        </ul></div>
    };

    view! { cx,
        <Window
            window_id="projects-win"
            window_title="Projects"
            window_content=content
            window_dimensions=(500, 300)
            start_pos=(50, 250)
        />
    }
}

#[component]
fn AboutWindow(cx: Scope) -> impl IntoView {
    let content = view! {cx,
        <div>
            <p>
                "Hi! My name is Ethan Corgatelli. You contact me via email ("
                <a target="blank" href="mailto:etbcor@gmai.com">"etbcor@gmail.com"</a>
                ") or on discord ("
                <a target="blank" href="http://www.discordapp.com/users/207897365141520384">"etbcor"</a>
                "). My GitHub profile can be found "
                <a target="blank" href="https://www.github.com/ETBCOR">"here"</a>
                ". Thanks for checking out my personal site!"
            </p>

        </div>
    };

    view! {cx,
        <Window
            window_id="about-win"
            window_title="About Me"
            window_content=content
            window_dimensions=(460, 75)
            start_pos=(500, 50)
        />
    }
}

#[component]
fn EducationWindow(cx: Scope) -> impl IntoView {
    let content = view! {cx,
        <div>
            "MY EDUCATION"
        </div>
    };

    view! {cx,
        <Window
            window_id="education-win"
            window_title="Education"
            window_content=content
            window_dimensions=(500, 300)
            start_pos=(700, 250)
        />
    }
}
