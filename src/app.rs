use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_event_listener, use_event_listener_with_options};
// use rand::{distributions::Slice, Rng};
use rand::seq::SliceRandom;
use web_sys::AddEventListenerOptions;

#[rustfmt::skip]
const ABSTRACT_NOUNS: [&str; 95] = [
    "Joy", "Hope", "Love", "Peace", "Serenity", "Happiness", "Bliss", "Gratitude", "Contentment", "Harmony",
    "Beauty", "Abundance", "Faith", "Trust", "Wonder", "Inspiration", "Courage", "Freedom", "Unity",
    "Compassion", "Generosity", "Empathy", "Kindness", "Forgiveness", "Patience", "Respect", "Gentleness",
    "Humility", "Graciousness", "Acceptance", "Radiance", "Positivity", "Enthusiasm", "Laughter", "Elation",
    "Zeal", "Determination", "Confidence", "Belief", "Optimism", "Sincerity", "Hopefulness", "Foresight",
    "Integrity", "Authenticity", "Nobility", "Honesty", "Loyalty", "Resilience", "Appreciation", "Vitality",
    "Curiosity", "Imagination", "Wonderment", "Exploration", "Ingenuity", "Creativity", "Innovation",
    "Empowerment", "Success", "Satisfaction", "Fulfillment", "Excitement", "Thrill",
    "Delight", "Exhilaration", "Peacefulness", "Tranquility", "Stillness", "Clarity", "Serendipity",
    "Enlightenment", "Progress", "Growth", "Transformation", "Expansion", "Meaning", "Grace", "Blessing",
    "Brilliance", "Wonderfulness", "Affection", "Warmth", "Caring", "Tenderness", "Nurturing", "Support",
    "Balance", "Moderation", "Simplicity", "Adaptability", "Flexibility", "Openness", "Belonging", "Ingenuity"
];

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
        <Link href="https://fonts.googleapis.com/css2?family=Caveat:wght@700&display=swap" rel="stylesheet"/>

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
    let skills_hidden = create_rw_signal(cx, false);
    let file_hidden = create_rw_signal(cx, true);
    let loading_hidden = create_rw_signal(cx, false);
    let ad_hidden = create_rw_signal(cx, false);
    let more_about_hidden = create_rw_signal(cx, true);

    let hidden_sigs = vec![
        about_hidden,
        projects_hidden,
        education_hidden,
        skills_hidden,
    ];
    let (file_src, set_file_src) = create_signal(cx, None);

    view! { cx,
        <AdWindow hidden=ad_hidden/>
        <LoadingWindow hidden=loading_hidden/>
        <AboutWindow hidden=about_hidden more_hidden=more_about_hidden/>
        <EducationWindow hidden=education_hidden/>
        <ProjectsWindow hidden=projects_hidden file_win_src=set_file_src/>
        <SkillsWindow hidden=skills_hidden/>
        <MoreAboutWindow hidden=more_about_hidden/>
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
        <h1>"- 404 -"</h1>
        <h2>"Page Not Found"</h2>
    }
}

#[component]
fn Footer(cx: Scope, hidden_sigs: Vec<RwSignal<bool>>) -> impl IntoView {
    let about = hidden_sigs[0];
    let projects = hidden_sigs[1];
    let education = hidden_sigs[2];
    let skills = hidden_sigs[3];

    let min_about = move |_| about.update(|h| *h = !*h);
    let min_projects = move |_| projects.update(|h| *h = !*h);
    let min_education = move |_| education.update(|h| *h = !*h);
    let min_skills = move |_| skills.update(|h| *h = !*h);

    view! { cx,
        <footer>
            <div class="win-minimized" on:mousedown=min_about class:hidden={move || !about()}>"About Me"</div>
            <div class="win-minimized" on:mousedown=min_education class:hidden={move || !education()}>"Education"</div>
            <div class="win-minimized" on:mousedown=min_projects class:hidden={move || !projects()}>"Projects"</div>
            <div class="win-minimized" on:mousedown=min_skills class:hidden={move || !skills()}>"Skills"</div>
        </footer>
    }
}

#[component]
fn Window(
    cx: Scope,
    window_id: &'static str,
    window_title: String,
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

    let get_title = move || {
        if window_title.starts_with("Loading") {
            let split: Vec<_> = window_title.split_whitespace().collect();
            view! { cx, <p>
                "Loading "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else {
            view! { cx, <p>{ &window_title }</p> }
        }
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
                { get_title }
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
#[allow(unused_variables)]
fn AboutWindow(cx: Scope, hidden: RwSignal<bool>, more_hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx,
        <div>
            <p>
                "Hi! My name is Ethan Corgatelli. I make programs and music and stuff. You can contact me on discord: "
                <ExternalLink href="http://www.discordapp.com/users/207897365141520384" display="etbcor"/>
                ", or via email: "<ExternalLink href="mailto:etbcor@gmail.com" display="etbcor@gmail.com"/>
                ". My GitHub profile is here: "<ExternalLink href="https://www.github.com/ETBCOR" display="ETBCOR"/>". "
                <i>"I'm "<u>"etbcor"</u>" on most platforms!"</i>
                <br/>
                // " Click "<a href="" on:click=move |_| more_hidden.set(false)>"here"</a>" to read more about me."
                " Thanks for checking out my site!"

            </p>

        </div>
    };

    view! { cx,
        <Window
            window_id="about-win"
            window_title="About Me".to_string()
            window_content=content
            window_width=640
            start_pos=(25, 20)
            hidden=hidden
        />
    }
}

#[component]
fn MoreAboutWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx, <div>
        <p>"Hello! ¡Hola! toki!"</p>
        <p>"I'm Friday / Ethan / jan Itan / ijo tan anpa nanpa."</p>
        <p>"I speak English (native), Spanish (advanced), and toki pona ("</p>
        <p>"I write code!"</p>
        <p>"I make music!"</p>
        <p>"I'm planning on extending this window in the future!"</p>

    </div> };

    view! { cx,
        <Window
            window_id="more-about-win"
            window_title="More About Me".to_string()
            window_content=content
            window_width=500
            start_pos=(120, 200)
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
        <li><b>"Projects From CS Classes"</b><ul>
            <li class="spaced">
                <b>"CS415 | Computational Biology: Sequence Alignment"</b>
                <br/>
                <FileLink src="https://drive.google.com/file/d/17M8KI3B6rCj2_WLL-YlbxBoK0WzTyexO/preview" display="GA Simulation Runner" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs415/tree/main/project01" display="Github Repository"/>
                <br/>
                <FileLink src="https://drive.google.com/file/d/1v9XjTqRlf4iGjHskT7yp_KUyVBUU7WgE/preview" display="Parameter Set Estimation" file_win_src=fws/>
                " | "<ExternalLink href="https://colab.research.google.com/drive/1zQtt-kDBhycueP_qyhzc9VnFeZe0wPmu?usp=sharing" display="Colab Notebook"/>
                <br/>
                <FileLink src="https://drive.google.com/file/d/1n-nyTQzjcGy9lpTvs-WYdBcTaDUbZfap/preview" display="Pairwise Alignment Matrix Calculation" file_win_src=fws/>
                " | "<ExternalLink href="https://colab.research.google.com/drive/1mMGnMO63KR-wHriGNYxBxF5YNwk_r7AP?usp=sharing" display="Colab Notebook"/>
            </li>

            <li class="spaced">
                <b>"CS445 | Compiler Design"</b>
                <br/>
                "I fully implemented a compiler for the \"C minus\" langauge (spec "
                <FileLink src="https://drive.google.com/file/d/12o5aSATedS28eJwsHIOHR7uf3DdZY20V/preview" display="here" file_win_src=fws/>
                ") in "<ExternalLink href="http://www2.cs.uidaho.edu/~mdwilder/cs445/" display="this class"/>
                ". I feel that this is the single largest project I've ever completed. Repository "
                <ExternalLink href="https://github.com/ETBCOR/cs445" display="here"/>"."
            </li>

            <li class="spaced">
                <b>"CS452 | Real-Time Operating Systems"</b>
                <br/>
                "I created multiple programs run on embedded systems (Feather RP2040 & ESP32) in this class. Repository "
                <ExternalLink href="https://github.com/ETBCOR/cs452/" display="here"/>"."
            </li>

            <li class="spaced">
                <b>"CS470 | Artificial Intelligence"</b>
                <br/>
                <FileLink src="https://drive.google.com/file/d/1ICaQOsGKwJ7RfE21xBHvozQkfQGkw43G/preview" display="Pathfinding Algorithms" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj1" display="Github Repository"/>
                <br/>
                <FileLink src="https://drive.google.com/file/d/1fK-F2X7uwnOk8CrDosopO1pRl6xlBc1u/preview" display="Connect-4 Bot Using Minmax" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj2" display="Github Repository"/>
                <br/>
                <FileLink src="https://drive.google.com/file/d/1Qr5B0yZ8s3aY3Ywdd4KCYq_7y5bXfCTg/preview" display="Map Coloring Algorithms" file_win_src=fws/>
                " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj3" display="Github Repository"/>
            </li>

            <li class="spaced">
                <b>"CS475 | Machine Learning"</b>
                <br/>
                "In "<ExternalLink href="http://marvin.cs.uidaho.edu/Teaching/CS475/index.html" display="this class"/>
                " I completed 8 assignments machine learning topics of varying difficulty. Although the repo is a bit messy, the link is "
                <ExternalLink href="https://github.com/ETBCOR/cs475" display="here"/>"."
            </li>

            <li class="spaced">
                <b>"CS480 & CS481 | Senior Capstone Design"</b>
                <br/>
                "My capstone project was to design calibration software for a laser communication device for "
                <ExternalLink href="https://www.hansenphotonics.com/" display="Hansen Photonics Inc"/>
                ". I was on a team with three other CS majors. The resulting software was simple, yet effective. "
                "And the creation process is well documented (contact me for details). Repository "
                <ExternalLink href="https://github.com/Hunter-SE/FibAir-Repository" display="here"/>"."
            </li>
        </ul></li>

        <li><b>"Other Projects"</b><ul>
            <li class="spaced">
                "This portfolio website, which I built from scratch using "
                <ExternalLink href="https://leptos.dev/" display="leptos"/>" (a full-stack web framework built with "
                <ExternalLink href="https://www.rust-lang.org/" display="Rust"/>" (my favorite programming language!))."
            </li>

            <li class="spaced">
                "I designed a font for sitelen pona (the writing system of a constructed language). Repository "
                <ExternalLink href="https://github.com/ETBCOR/nasin-nanpa" display="here"/>"."
            </li>

            <li class="spaced">
                "I have "<ExternalLink href="https://www.instagram.com/ecridisedits/" display="an Instagram page"/>" full of cool audio/visaully synced edits I made with After Effects."
            </li>

            <li>"I have worked on quite a few other projects, both personal projects and projects for school (this list is nonexhaustive)."</li>
        </ul></li>
    </ul></div> };

    view! { cx,
        <Window
            window_id="projects-win"
            window_title="Projects".to_string()
            window_content=content
            window_width=550
            start_pos=(775, 20)
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
            <ul style="font-family: consolas; font-size: 10pt; font-style: bold; line-height: 110%">
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
                <li>"CS415 | Computational Biology: Sequence Alignment"</li>
                <li>"CS445 | Compiler Design"</li>
                <li>"CS452 | Real-Time Operating Systems"</li>
                <li>"CS470 | Artificial Intelligence"</li>
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
            window_title="Education".to_string()
            window_content=content
            window_width=400
            start_pos=(25, 240)
            hidden=hidden
        />
    }
}

#[component]
fn SkillsWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx, <div><ul>
        <li class="spaced"><b>"Data structures and algorithms"</b>
        ": my B.S.C.S. has given me a strong foundation in the fundamentals of Computer Science. "
        "I am experienced in designing and analyzing various data structures and algorithms."</li>
        <li class="spaced">"I'm proficient in multiple "<b>"programming languages"</b>":"<ul>
            <li><b style="font-family: 'VT323'">"C / C++"</b>" were the primary languages taught at my univirsity, so I'm very comfortable with them."</li>
            <li><b style="font-family: 'VT323'">"Rust"</b>" is currently my favorite language. I learned about it at some point in 2022, and recently started using it for all my school projects, so I'm at an intermediate/advanced level."</li>
            <li><b style="font-family: 'VT323'">"Python"</b>" isn't usually what I reach to first for my projects, but I'm still proficient with it, and have used it for a few."</li>
            <li><b style="font-family: 'VT323'">""</b>"...and more! Including "<b style="font-family: 'VT323'">"JavaScript"</b>", "<b style="font-family: 'VT323'">"Java"</b>", and even some "<b style="font-family: 'VT323'">"Prolog"</b>"!"</li>
        </ul></li>
        <li class="spaced">"I'm also fluent in multiple "<b>"spoekn languages"</b>":"<ul>
            <li>"English (native)"</li>
            <li>"Spanish (fluent)"</li>
            <li>"toki pona (fluent) (a minimalist constructed language)"</li>
        </ul></li>
    </ul></div> };

    view! { cx,
        <Window
            window_id="skills-win"
            window_title="Skills".to_string()
            window_content=content
            window_width=550
            start_pos=(775, 425)
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
            window_title="File Viewer".to_string()
            window_content=content
            window_width=800
            start_pos=(30, 30)
            hidden=hidden
        />
    }
}

#[component]
fn LoadingWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let noun: &'static str = ABSTRACT_NOUNS.choose(&mut rng).unwrap();
    let title = format!("Loading {}", noun);
    let content = view! { cx, <div>
        <img src="/assets/infinity.svg" style="width: 100%; height: 100px" draggable="false"/>
    </div> };

    view! { cx,
        <Window
            window_id="loading-win"
            window_title=title
            window_content=content
            window_width=225
            start_pos=(495, 325)
            hidden=hidden
        />
    }
}

#[component]
fn AdWindow(cx: Scope, hidden: RwSignal<bool>) -> impl IntoView {
    let content = view! { cx, <div style="height: 100px">
        <img src="/assets/ur-ad-here.png" style="height: 100px; width: 200px; image-rendering: pixelated" draggable="false"/>
    </div> };

    view! { cx,
        <Window
            window_id="ad-win"
            window_title="Advertisement".to_string()
            window_content=content
            window_width=200
            start_pos=(255, 22)
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
