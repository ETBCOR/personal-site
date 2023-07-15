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
    let about_hidden = create_rw_signal(cx, false);
    let projects_hidden = create_rw_signal(cx, false);
    let education_hidden = create_rw_signal(cx, false);
    let file_hidden = create_rw_signal(cx, true);
    let loading_hidden = create_rw_signal(cx, false);

    let hidden_sigs = vec![about_hidden, projects_hidden, education_hidden];
    let (file_src, set_file_src) = create_signal(cx, None);

    view! { cx,
        <LoadingWindow hidden=loading_hidden/>
        <EducationWindow hidden=education_hidden/>
        <ProjectsWindow hidden=projects_hidden file_win_src=set_file_src/>
        <AboutWindow hidden=about_hidden/>
        <FileWindow hidden=file_hidden src=file_src/>
        <Footer hidden_sigs=hidden_sigs/>
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
fn Footer(cx: Scope, hidden_sigs: Vec<RwSignal<bool>>) -> impl IntoView {
    let about = hidden_sigs[0];
    let projects = hidden_sigs[1];
    let education = hidden_sigs[2];

    let min_about = move |_| about.update(|h| *h = !*h);
    let min_projects = move |_| projects.update(|h| *h = !*h);
    let min_education = move |_| education.update(|h| *h = !*h);

    view! { cx,
        <footer>
            <div class="win-minimized" on:mousedown=min_about class:hidden={move || !about()}>"About Me"</div>
            <div class="win-minimized" on:mousedown=min_projects class:hidden={move || !projects()}>"Projects"</div>
            <div class="win-minimized" on:mousedown=min_education class:hidden={move || !education()}>"Education"</div>
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

    view! { cx,
        <div
            id=window_id
            class="win-outer"
            class:hidden={move || hidden()}
            style:left=move || format!("{}px", x())
            style:top=move || format!("{}px", y())>
            <div
                class="win-titlebar"
                on:mousedown=drag>
                <p>{window_title}</p>
                <a
                    class="win-close"
                    on:mousedown=move |_| hidden.set(true)></a>
            </div>
            <div
                class="win-content"
                style:width=move || format!("{}px", window_width)>
                {window_content}
            </div>
        </div>
    }
}

#[component]
fn AboutWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx,
        <div>
            <p>
                "Hi! My name is Ethan Corgatelli. I make programs and music and stuff. You can contact me on discord: "
                <i><ExternalLink href="http://www.discordapp.com/users/207897365141520384" display="etbcor"/></i>
                ", or via email: "<i><ExternalLink href="mailto:etbcor@gmail.com" display="etbcor@gmail.com"/></i>
                ". My GitHub profile is here: "<i><ExternalLink href="https://www.github.com/ETBCOR" display="ETBCOR"/></i>". "
                <i>"I'm "<u>"etbcor"</u>" on most platforms!"</i><br/>"Thanks for checking out my site!"
            </p>

        </div>
    };

    view! { cx,
        <Window
            window_id="about-win"
            window_title="About Me"
            window_content=content
            window_width=630
            start_pos=(105, 20)
            hidden=hidden
        />
    }
}

#[component]
fn ProjectsWindow(
    cx: Scope,
    hidden: RwSignal<bool>,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let fws = file_win_src;
    let content = view! { cx, <div><ul>
        <li><b>"CS415 | Computational Biology: Sequence Analysis"</b><ul class="spaced">
            <li><FileLink src="https://drive.google.com/file/d/17M8KI3B6rCj2_WLL-YlbxBoK0WzTyexO/preview" display="GA Simulation Runner" file_win_src=fws/>
            " | "<ExternalLink href="https://github.com/ETBCOR/cs415/tree/main/project01" display="Github Repository"/></li>

            <li><FileLink src="https://drive.google.com/file/d/1v9XjTqRlf4iGjHskT7yp_KUyVBUU7WgE/preview" display="Parameter Set Estimation" file_win_src=fws/>
            " | "<ExternalLink href="https://colab.research.google.com/drive/1zQtt-kDBhycueP_qyhzc9VnFeZe0wPmu?usp=sharing" display="Colab Notebook"/></li>

            <li><FileLink src="https://drive.google.com/file/d/1n-nyTQzjcGy9lpTvs-WYdBcTaDUbZfap/preview" display="Pairwise Alignment Matrix Calculation" file_win_src=fws/>
            " | "<ExternalLink href="https://colab.research.google.com/drive/1mMGnMO63KR-wHriGNYxBxF5YNwk_r7AP?usp=sharing" display="Colab Notebook"/></li>
        </ul></li>

        <li><b>"CS445 | Compiler Design"</b><ul class="spaced"><li>
            "I implemented a compiler for the \"C minus\" langauge"<br/>"(spec "
            <FileLink src="https://drive.google.com/file/d/12o5aSATedS28eJwsHIOHR7uf3DdZY20V/preview" display="here" file_win_src=fws/>
            ") in this class. Repository "<ExternalLink href="https://github.com/ETBCOR/cs445" display="here"/>"."
        </li></ul></li>

        <li><b>"CS452 | Real-Time Operating Systems"</b><ul class="spaced"><li>
            "I created multiple programs run on embedded systems (Feather RP2040 & ESP32) in this class. Repository "
            <ExternalLink href="https://github.com/ETBCOR/cs452/" display="here"/>"."
        </li></ul></li>

        <li><b>"CS470 | Artificial Intelligence"</b><ul class="spaced">
            <li>
                <FileLink src="https://drive.google.com/file/d/1ICaQOsGKwJ7RfE21xBHvozQkfQGkw43G/preview" display="Pathfinding Algorithms" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj1" display="Github Repository"/>
            </li>
            <li>
                <FileLink src="https://drive.google.com/file/d/1fK-F2X7uwnOk8CrDosopO1pRl6xlBc1u/preview" display="Connect-4 Bot Using Minmax" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj2" display="Github Repository"/>
            </li>
            <li>
                <FileLink src="https://drive.google.com/file/d/1Qr5B0yZ8s3aY3Ywdd4KCYq_7y5bXfCTg/preview" display="Map Coloring Algorithms" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj3" display="Github Repository"/>
            </li>
        </ul></li>

        <li><b>"CS472 | Evolutionary Computation"</b><ul class="spaced">
        </ul></li>

        <li><b>"CS475 | Machine Learning"</b><ul class="spaced">
        </ul></li>

        <li><b>"CS480 & CS481 | Senior Capstone Design"</b><ul class="spaced">
        </ul></li>

        <li><b>"Other"</b><ul class="spaced">
            <li>"I have worked on many other projects (academic and personal), not listed here."</li>
        </ul></li>
    </ul></div> };

    view! { cx,
        <Window
            window_id="projects-win"
            window_title="Projects"
            window_content=content
            window_width=550
            start_pos=(875, 69)
            hidden=hidden
        />
    }
}

#[component]
fn EducationWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx, <div>
        <h3>"Bachelor's Degree in Computer Science"</h3>
        <p>
            "I spent 2019-2023 at the "<ExternalLink href="https://www.uidaho.edu/" display="University of Idaho"/>
            ", getting my "<ExternalLink href="https://catalog.uidaho.edu/courses/cs/" display="B.S.C.S."/>
            ", as well as my "<ExternalLink href="https://catalog.uidaho.edu/courses/span/" display="Spanish minor"/>"."
        </p>

        <p><details style="max-height: 125px; overflow-y: auto">
            <summary><u>"CS classes I took at UI"</u></summary>
            <ul style="font-family: consolas; font-size: 10pt; line-height: 95%">
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
        </details></p>

        <h3>"K thru 12"</h3>
        "I was homeschooled from kindergarten through higschool, with two exceptions:"
        <ol>
            <li>"I did a year of Montessori in like 5th grade"</li>
            <li>"in high school, I was half-time homeschooled and half-time public school (at Idaho Falls High School)"</li>
        </ol>

        <p>"I gained an interest for coding around the age of 10. A friend of mine showed me "
        <ExternalLink href="https://www.codecademy.com/" display="codecademy.com"/>
        " (back when it was still completely free!), which was very influential for me starting out."</p>
    </div> };

    view! { cx,
        <Window
            window_id="education-win"
            window_title="Education"
            window_content=content
            window_width=400
            start_pos=(70, 240)
            hidden=hidden
        />
    }
}

#[component]
fn FileWindow(
    cx: Scope,
    hidden: RwSignal<bool>,
    src: ReadSignal<Option<&'static str>>,
) -> impl IntoView {
    let content = view! { cx, <div>
        <iframe
            src=move || { if src().is_some() { hidden.set(false); } src().unwrap_or("") }
            allow="autoplay"
            style="width: 100%; height: 800px"
        ></iframe>
    </div> };

    view! { cx,
        <Window
            window_id="file-win"
            window_title="File Viewer"
            window_content=content
            window_width=800
            start_pos=(30, 30)
            hidden=hidden
        />
    }
}

#[component]
fn LoadingWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx, <div style="background-image: radial-gradient(#93E1D8, #DDFFF7)">
        <img src="/assets/Infinity-10s-200px.svg" style="width: 100%; height: 100px" draggable="false"/>
    </div> };

    view! { cx,
        <Window
            window_id="loading-win"
            window_title="Loading Meaning"
            window_content=content
            window_width=225
            start_pos=(575, 275)
            hidden=hidden
        />
    }
}

#[component]
fn FileLink(
    cx: Scope,
    src: &'static str,
    display: &'static str,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    view! { cx, <a href="" on:mousedown=move |_| file_win_src.set(Some(src))>{display}</a> }
}

#[component]
fn ExternalLink(cx: Scope, href: &'static str, display: &'static str) -> impl IntoView {
    view! { cx,
        <a target="_blank" href=href>{display}</a>
        <a class="external-link"></a>
    }
}
