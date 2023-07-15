use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_event_listener, use_event_listener_with_options};
use web_sys::AddEventListenerOptions;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/personal_site.css"/>

        <Title text="Ethan Corgatelli"/>

        // Google fonts
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        <Link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,600;0,700;1,700&display=swap" rel="stylesheet"/>

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
    let about = create_rw_signal(cx, true);
    let projects = create_rw_signal(cx, true);
    let education = create_rw_signal(cx, true);
    let sigs = vec![about, projects, education];

    view! { cx,
        <AboutWindow hidden=about/>
        <EducationWindow hidden=education/>
        <ProjectsWindow hidden=projects/>
        <Footer sigs=sigs/>
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
fn Footer(cx: Scope, sigs: Vec<RwSignal<bool>>) -> impl IntoView {
    let about = sigs[0];
    let projects = sigs[1];
    let education = sigs[2];

    let min_about = move |_| about.update(|h| *h = !*h);
    let min_projects = move |_| projects.update(|h| *h = !*h);
    let min_education = move |_| education.update(|h| *h = !*h);

    view! {cx,
        <footer>
            <div class="win-minimized" on:mousedown=min_about class:hidden={move || about.get()}>"About Me"</div>
            <div class="win-minimized" on:mousedown=min_projects class:hidden={move || projects.get()}>"Projects"</div>
            <div class="win-minimized" on:mousedown=min_education class:hidden={move || education.get()}>"Education"</div>
        </footer>
    }
}

#[component]
fn Window(
    cx: Scope,
    window_id: &'static str,
    window_title: &'static str,
    window_content: HtmlElement<html::Div>,
    window_width: i32,
    start_pos: (i32, i32),
    hidden: RwSignal<bool>,
) -> impl IntoView {
    let (x, set_x) = create_signal(cx, start_pos.0);
    let (y, set_y) = create_signal(cx, start_pos.1);
    let (dx, set_dx) = create_signal(cx, 0);
    let (dy, set_dy) = create_signal(cx, 0);

    // let close_window = move |_| hidden.update(|hidden| *hidden = !*hidden);
    let close_window = move |_| hidden.set(false);

    let drag = move |e: MouseEvent| {
        set_dx.set(x.get_untracked() - e.client_x());
        set_dy.set(y.get_untracked() - e.client_y());
        let drag_cleanup = use_event_listener(cx, document(), ev::mousemove, move |e| {
            set_x.set(e.client_x() + dx.get_untracked());
            set_y.set(e.client_y() + dy.get_untracked());
        });

        let mut once_opt = AddEventListenerOptions::new();
        once_opt.once(true);
        let _ = use_event_listener_with_options(
            cx,
            document(),
            ev::mouseup,
            move |_| {
                drag_cleanup();
            },
            once_opt,
        );
    };

    let get_pos = move || format!("top: {}px; left: {}px", y.get(), x.get());
    let get_dim = move || format!("width: {}px;", window_width);

    view! { cx,
        <div
            id=window_id
            class="win-outer"
            class:hidden={move || !hidden.get()}
            style=get_pos>
            <div
                class="win-titlebar"
                on:mousedown=drag>
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
fn AboutWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! {cx,
        <div>
            <p>
                "Hi! My name is Ethan Corgatelli. You can contact me on discord: "
                <i><a target="_blank" href="http://www.discordapp.com/users/207897365141520384">"etbcor"</a></i>
                ", or via email: "
                <i><a target="_blank" href="mailto:etbcor@gmail.com">"etbcor@gmail.com"</a></i>
                ". My GitHub profile is here: "
                <i><a target="_blank" href="https://www.github.com/ETBCOR">"ETBCOR"</a></i>
                ". (I'm "<b><i>"etbcor"</i></b>" on most platforms!) Thanks for checking out my site!"
            </p>

        </div>
    };

    view! {cx,
        <Window
            window_id="about-win"
            window_title="About Me"
            window_content=content
            window_width=460
            start_pos=(60, 25)
            hidden=hidden
        />
    }
}

#[component]
fn EducationWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! {cx,
        <div>
            <h3>"K thru 12"</h3>
            "I was homeschooled from preschool through higschool, with two exceptions:"
            <ol>
                <li>"I did a year of Montessori in like 5th grade"</li>
                <li>"in high school, I was half-time homeschooled and half-time public school (at Idaho Falls High School)"</li>
            </ol>
            <p>"I gained an interest for coding around the age of 10. "
            "A friend of mine showed me "<a target="_blank" href="https://www.codecademy.com/">"codecademy.com"</a>
            " (back when it was still free!), which was very influential for me starting out."</p>
            <h3>"Bachelor's Degree in Computer Science"</h3>
            <p>
                "I spent 2019-2023 at the "<a target="_blank" href="https://www.uidaho.edu/">"University of Idaho"</a>
                ", getting my "<a target="_blank" href="https://catalog.uidaho.edu/courses/cs/">"B.S.C.S."</a>", as well as my "
                <a target="_blank" href="https://catalog.uidaho.edu/courses/span/">"Spanish minor"</a>"."
                <details style="max-height: 200px; overflow-y: auto;">
                    <summary>"Click here for a list of the CS classes I took at UI."</summary>
                    <ul style="font-family: consolas;">
                        <li>"CS120 | Computer Science I"</li>
                        <li>"CS121 | Computer Science II"</li>
                        <li>"CS150 | Computer Organization and Architecture"</li>
                        <li>"CS210 | Programming Languages"</li>
                        <li>"CS240 | Computer Operating Systems"</li>
                        <li>"CS270 | System Software"</li>
                        <li>"CS360 | Database Systems"</li>
                        <li>"CS383 | Software Engineering"</li>
                        <li>"CS385 | Theory of Computation"</li>
                        <li>"CS395 | Analysis of Algorithms"</li>
                        <li>"CS400 | Contemporary Issues in CS"</li>
                        <li>"CS415 | Computational Biology: Sequence Analysis"</li>
                        <li>"CS445 | Compiler Design"</li>
                        <li>"CS452 | Real-Time Operating Systems"</li>
                        <li>"CS470 | Artificial Intelligence"</li>
                        <li>"CS472 | Evolutionary Computation"</li>
                        <li>"CS475 | Machine Learning"</li>
                        <li>"CS480 | CS Senior Capstone Design I"</li>
                        <li>"CS481 | CS Senior Capstone Design II"</li>
                    </ul>
                </details>
            </p>
        </div>
    };

    view! {cx,
        <Window
            window_id="education-win"
            window_title="Education"
            window_content=content
            window_width=550
            start_pos=(25, 215)
            hidden=hidden
        />
    }
}

#[component]
fn ProjectsWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! {cx, <div>
    <ul>
        <li>"Compiler Design"<ul>
            <li>"I implemented a compiler. "
            <a target="_blank" href="https://github.com/ETBCOR/cs445">"Here"</a>"'s the repository."</li>
        </ul></li>
        <li>"Computational Biology: Sequence Analysis"<ul>
            <li><a target="_blank" href="https://drive.google.com/file/d/17M8KI3B6rCj2_WLL-YlbxBoK0WzTyexO/view?usp=sharing">"GA Simulation Runner"</a></li>
            <li><a target="_blank" href="https://drive.google.com/file/d/1v9XjTqRlf4iGjHskT7yp_KUyVBUU7WgE/view?usp=sharing">"Parameter Set Estimation"</a></li>
            <li><a target="_blank" href="https://drive.google.com/file/d/1n-nyTQzjcGy9lpTvs-WYdBcTaDUbZfap/view?usp=sharing">"Pairwise Alignment Matrix Calculation"</a></li>
        </ul></li>
    </ul>
    </div> };

    view! { cx,
        <Window
            window_id="projects-win"
            window_title="Projects"
            window_content=content
            window_width=500
            start_pos=(640, 69)
            hidden=hidden
        />
    }
}
