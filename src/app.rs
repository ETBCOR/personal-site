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
    "Empowerment", "Success", "Satisfaction", "Fulfillment", "Excitement", "Thrill", "Delight",
    "Exhilaration", "Peacefulness", "Tranquility", "Stillness", "Clarity", "Serendipity", "Enlightenment",
    "Progress", "Growth", "Change", "Expansion", "Meaning", "Grace", "Blessing", "Brilliance", "Affection",
    "Warmth", "Caring", "Tenderness", "Nurturing", "Support", "Balance", "Moderation", "Simplicity",
    "Adaptability", "Flexibility", "Openness", "Belonging", "Ingenuity", "Mediation"
];

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio_site.css"/>

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

    let hidden_sigs = vec![
        about_hidden,
        projects_hidden,
        education_hidden,
        skills_hidden,
        loading_hidden,
    ];
    let (file_src, set_file_src) = create_signal(cx, None);
    let z_idx = create_rw_signal(cx, 1);

    view! { cx,
        <AboutWindow hidden=about_hidden z_idx=z_idx/>
        <EducationWindow hidden=education_hidden z_idx=z_idx/>
        <SkillsWindow hidden=skills_hidden z_idx=z_idx/>
        <ProjectsWindow hidden=projects_hidden z_idx=z_idx file_win_src=set_file_src/>
        <LoadingWindow hidden=loading_hidden z_idx=z_idx/>
        <AdWindow hidden=ad_hidden z_idx=z_idx/>
        <FileWindow hidden=file_hidden z_idx=z_idx src=file_src/>
        <div style="height: 65px"></div> // spacer
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
    let loading = hidden_sigs[4];
    let footer = move || !(about() || education() || projects() || skills() || loading());

    let min_about = move |_| about.update(|h| *h = !*h);
    let min_projects = move |_| projects.update(|h| *h = !*h);
    let min_education = move |_| education.update(|h| *h = !*h);
    let min_skills = move |_| skills.update(|h| *h = !*h);
    let min_loading = move |_| loading.update(|h| *h = !*h);

    view! { cx,
        <footer class:hidden={footer}>
            <div class="title win-minimized" on:mousedown=min_about class:hidden={move || !about()}>"About Me"</div>
            <div class="title win-minimized" on:mousedown=min_education class:hidden={move || !education()}>"Education"</div>
            <div class="title win-minimized" on:mousedown=min_projects class:hidden={move || !projects()}>"Projects"</div>
            <div class="title win-minimized" on:mousedown=min_skills class:hidden={move || !skills()}>"Skills"</div>
            <div class="title win-minimized" on:mousedown=min_loading class:hidden={move || !loading()}>"\"Inspiration\""</div>
        </footer>
    }
}
type Tabs = Option<(
    RwSignal<&'static str>,
    Vec<(&'static str, HtmlElement<html::Div>)>,
)>;

#[component]
fn Window(
    cx: Scope,
    id: &'static str,
    title: String,
    content: HtmlElement<html::Div>,
    #[prop(default = None)] tabs: Tabs,
    start_pos: (i32, i32),
    hidden: RwSignal<bool>,
    z_idx: RwSignal<usize>,
) -> impl IntoView {
    let x = create_rw_signal(cx, start_pos.0);
    let y = create_rw_signal(cx, start_pos.1);
    let dx = create_rw_signal(cx, 0);
    let dy = create_rw_signal(cx, 0);

    let this_z_idx = create_rw_signal(cx, if id.eq("ad-win") { 0 } else { z_idx() });

    let drag = move |e: MouseEvent| {
        z_idx.update(|z| *z = *z + 1);
        this_z_idx.set(z_idx());

        dx.set(x.get_untracked() - e.client_x());
        dy.set(y.get_untracked() - e.client_y());
        let drag_cleanup = use_event_listener(cx, document(), ev::mousemove, move |e| {
            x.set(e.client_x() + dx.get_untracked());
            y.set(e.client_y() + dy.get_untracked());
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
        if title.starts_with("Loading") {
            let split: Vec<_> = title.split_whitespace().collect();
            view! { cx, <p class="title">
                "Loading "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else {
            view! { cx, <p class="title">{&title}</p> }
        }
    };

    let get_content = match tabs {
        Some((active_tab, combined_vec)) => {
            let (titles, tabs): (Vec<_>, Vec<_>) = combined_vec
                .into_iter()
                .map(|(title, content)| {
                    (
                        view! { cx,
                            <div class="title"
                                class:active=move || active_tab().eq(title)
                                on:click=move |_| active_tab.set(title)>
                                { title }
                            </div>
                        },
                        view! { cx,
                            <div class="win-content"
                                class:hidden=move || !active_tab().eq(title)>
                                { content }
                            </div>
                        },
                    )
                })
                .unzip();

            view! { cx,
                <div>
                    <div class="tab-titlebar"> { titles } </div>
                    { tabs }
                </div>
            }
        }
        None => view! { cx,
            <div class="win-content">
                { content }
            </div>
        },
    };

    view! { cx,
        <div
            id=id
            class="win-outer"
            class:hidden={move || hidden()}
            style=move || format!("left: {}px; top: {}px; z-index: {}", x(), y(), this_z_idx())>
            <div
                class="win-titlebar"
                on:mousedown=drag>
                { get_title }
                <a
                    class="win-close"
                    on:mousedown=move |_| hidden.set(true)></a>
            </div>
            { get_content }
        </div>
    }
}

#[component]
#[allow(unused_variables)]
fn AboutWindow(cx: Scope, hidden: RwSignal<bool>, z_idx: RwSignal<usize>) -> impl IntoView {
    let content = view! { cx, <div> <p>
        "Hello! I'm Ethan Corgatelli, and was born in April 2001. "
        "I'm passionate about making software, writing music, and learning languages. You can contact me "
        <ExternalLink href="http://www.discordapp.com/users/207897365141520384" display="on discord"/>
        ", or "<ExternalLink href="mailto:etbcor@gmail.com" display="via email."/>
        ". Here's my "<ExternalLink href="https://www.github.com/ETBCOR" display="GitHub profile"/>". "
        <i>"I'm "<u>"etbcor"</u>" on most platforms!"</i>" Thanks for coming to my site!"
        // " Click "<a href="" on:click=move |_| more_hidden.set(false)>"here"</a>" to read more about me."
    </p> </div> };

    view! { cx,
        <Window
            id="about-win"
            title="About Me".to_string()
            content=content
            start_pos=(25, 20)
            hidden=hidden
            z_idx=z_idx
        />
    }
}

#[component]
fn EducationWindow(cx: Scope, hidden: RwSignal<bool>, z_idx: RwSignal<usize>) -> impl IntoView {
    let content = view! { cx, <div>
        <h4>"Bachelor's Degree in Computer Science"</h4>
        <div class="spaced">
            "I spent 2019-2023 at the "<ExternalLink href="https://www.uidaho.edu/" display="University of Idaho"/>
            ", getting my "<ExternalLink href="https://catalog.uidaho.edu/courses/cs/" display="B.S.C.S."/>
            ", as well as my "<ExternalLink href="https://catalog.uidaho.edu/courses/span/" display="Spanish minor"/>"."
        </div>

        <div>"CS Classes I took at UI:"</div>
        <div style="border: 1px black solid; max-height: 110px; overflow-y: scroll">
            <ul  style="font-family: consolas; font-size: 10pt; font-style: bold; line-height: 110%">
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
        </div>
        <div class="spaced"></div>

        <h4>"K thru 12"</h4>
        "I was homeschooled from kindergarten through high school, with two exceptions:"
        <ol>
            <li>"I did a year of Montessori in 5th grade"</li>
            <li>"in high school, I was half-time homeschooled and half-time public school (at Idaho Falls High School)"</li>
        </ol>

        <p>"I gained an interest for coding around the age of 10. A friend of mine showed me "
        <ExternalLink href="https://www.codecademy.com/" display="codecademy.com"/>
        " (back when it was still completely free!), which was very influential for me starting out."</p>
    </div> };

    view! { cx,
        <Window
            id="education-win"
            title="Education".to_string()
            content=content
            start_pos=(25, 210)
            hidden=hidden
            z_idx=z_idx
        />
    }
}

#[component]
fn SkillsWindow(cx: Scope, hidden: RwSignal<bool>, z_idx: RwSignal<usize>) -> impl IntoView {
    let active_tab = create_rw_signal(cx, "Technical");

    let content = view! { cx,
        <div>"Failed to load tabs for this window"</div>
    };

    let tabs = vec![
        (
            "Technical",
            view! { cx, <div><ul>
                <li class="spaced">"I'm proficient in multiple "<b>"programming languages"</b>":"<ul>
                    <li><span class="title">"C / C++"</span>" were the primary languages taught at my univirsity, so I'm very comfortable with them."</li>
                    <li><span class="title">"Rust"</span>" is currently my favorite language. I learned about it at some point in 2022, "
                        "and recently started using it for all my school projects, so I'm at an intermediate/advanced level."</li>
                    <li><span class="title">"Python"</span>" isn't usually what I reach to first "
                        "for my projects, but I'm still proficient with it, and have used it for a few."</li>
                    <li><span class="title">""</span>"...and more, including "<span class="title">"JavaScript"</span>", "
                    <span class="title">"Java"</span>", and even some "<span class="title">"Prolog"</span>"!"</li>
                </ul></li>

                <li class="spaced"><b>"Data structures and algorithms"</b>
                ": my B.S.C.S. has given me a strong foundation in the fundamentals of Computer Science. "
                "I am experienced in designing and analyzing various data structures and algorithms."</li>

                <li class="spaced">
                    "I'm farmiliar with "<b>"software development concepts"</b>
                    ", including code modularity / testing / documentation / version control techniques, "
                    <span class="title">"agile"</span>", "<span class="title">"continuous integration and delivery"
                    </span>" and "<span class="title">"the software development life cycle"</span>"."
                </li>

                <li class="spaced">
                    "I have a solid understanding of "<b>"networking"</b>" and "<b>"web development"</b>", including how to work with protocols like "
                    <span class="title">"IP"</span>", "<span class="title">"HTTP"</span>", "<span class="title">"TCP"</span>" and "<span class="title">"UDP"</span>
                    ", as well as technologies like "<span class="title">"databases"</span>", "<span class="title">"HTML"</span>", "<span class="title">"CSS"</span>" and "<span class="title">"JavaScript"</span>"."
                </li>

                <li class="spaced">
                    "I also have a solid understanding of "<b>"computer architecture"</b>
                    " and "<b>"operating systems"</b>" concepts in general."
                </li>

                <li>
                    "I know how to write code for "<b>"embedded systems"</b>" using the principles of "
                    <span class="title">"real-time operating systems"</span>"."
                </li>
            </ul></div> },
        ),
        (
            "Audio / Visual",
            view! { cx, <div><ul>
                <li><b>"Audio"</b><ul>
                    <li class="spaced">
                        "I purchased "<ExternalLink href="https://www.ableton.com/en/live/" display="Ableton Live" title_style=true/>
                        " in 2018, and I've been using it to make music in my free time ever since. "
                        "I've honed my production skills quite a bit, but I'm still yet to start releasing music."
                    </li>
                    <li class="spaced">
                        "I volunteered at my church for several years in high school operating the sound booth for the live band, "
                        "so I'm comfortable running a large sound board (analog or digital) and with the basics of audio engineering."
                    </li>
                </ul></li>

                <li><b>"Visual"</b><ul>
                    <li class="spaced">
                        "I'm quite experienced with "<span class="title">"After Effects"</span>". You can see some of what I've created with it on "
                        <ExternalLink href="https://www.instagram.com/ecridisedits/" display="my IG page"/>"."
                    </li>
                    <li>
                        "I've also volunteered at my church to run slides/lights for sermons, so I'm familiar with "<span class="title">"ProPresenter"</span>
                        " as well as "<br/><span class="title">"DMX lighting systems"</span>"."
                    </li>
                </ul></li>
            </ul></div> },
        ),
        (
            "Other",
            view! { cx, <div><ul>
                <li class="spaced">"I speak "<b>"three languages"</b>":"<ul>
                    <li><span class="title">"English"</span>" (native)"</li>
                    <li><span class="title">"Spanish"</span>" (fluent)"</li>
                    <li><ExternalLink href="https://tokipona.org/" display="toki pona" title_style=true/>" (fluent)"</li>
                    <li><span class="title">"Japanese"</span>" (beginner)"</li>
                </ul></li>

                <li class="spaced">"I have great "<b>"interpersonal"</b>" and "<b>"conflict-resolution"</b>
                    " skills; I'm able to meaningfully communicate with people, even when we have conflicting views."</li>

                <li>"I care deeply about my "<b>"work ethic"</b>"; I enjoy locking into my work and getting in the zone."</li>
            </ul></div> },
        ),
    ];

    view! { cx,
        <Window
            id="skills-win"
            title="Skills".to_string()
            content=content
            tabs=Some((active_tab, tabs))
            start_pos=(735, 20)
            hidden=hidden
            z_idx=z_idx
        />
    }
}

#[component]
fn ProjectsWindow(
    cx: Scope,
    hidden: RwSignal<bool>,
    z_idx: RwSignal<usize>,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let fws = file_win_src;
    let active_tab = create_rw_signal(cx, "From CS Classes");

    let content = view! { cx,
        <div>"Failed to load tabs for this window"</div>
    };

    let tabs = vec![
        (
            "From CS Classes",
            view! { cx, <div><ul>
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
                    "I fully implemented a compiler for the \"C minus\" langauge (grammar specification "
                    <FileLink src="https://drive.google.com/file/d/12o5aSATedS28eJwsHIOHR7uf3DdZY20V/preview" display="here" file_win_src=fws/>
                    ") in "<ExternalLink href="http://www2.cs.uidaho.edu/~mdwilder/cs445/" display="this class"/>
                    ". This could be the single largest project I've completed so far. Repository "
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

                <li>
                    <b>"CS480 & CS481 | Senior Capstone Design"</b>
                    <br/>
                    "My capstone project was to design calibration software for a laser communication device for "
                    <ExternalLink href="https://www.hansenphotonics.com/" display="Hansen Photonics Inc"/>
                    ". I was on a team with three other CS majors. The resulting software was simple, yet effective. "
                    "And the creation process is well documented (contact me for details). Repository "
                    <ExternalLink href="https://github.com/Hunter-SE/FibAir-Repository" display="here"/>"."
                </li>
            </ul></div> },
        ),
        (
            "Other Projects",
            view! { cx, <div><ul>
                <li class="spaced">
                    "I made "<b>"this very portfolio website"</b>" with "
                    <ExternalLink href="https://leptos.dev/" display="leptos"/>" (a full-stack web framework built in "
                    <ExternalLink href="https://www.rust-lang.org/" display="Rust"/>")."
                </li>

                <li class="spaced">
                    "I designed "<b>"a font"</b>" for sitelen pona (the writing system of a constructed language). Repository "
                    <ExternalLink href="https://github.com/ETBCOR/nasin-nanpa" display="here"/>"."
                </li>

                <li class="spaced">
                    "I've made hundereds of "<b>"songs"</b>" (varying in completeness) "
                    "with Ableton in my free time, but I haven't released anything yet."
                </li>

                <li class="spaced">
                    "I have "<ExternalLink href="https://www.instagram.com/ecridisedits/" display="an Instagram page"/>
                    " full of cool audio/visaully synced "<b>"edits"</b>" I made with After Effects."
                </li>

                <li>"I have worked on quite a few other projects, both personal projects and projects for school (this list is nonexhaustive)."</li>
            </ul></div> },
        ),
    ];

    view! { cx,
        <Window
            id="projects-win"
            title="Projects".to_string()
            content=content
            tabs=Some((active_tab, tabs))
            start_pos=(735, 425)
            hidden=hidden
            z_idx=z_idx
        />
    }
}

#[component]
fn FileWindow(
    cx: Scope,
    hidden: RwSignal<bool>,
    z_idx: RwSignal<usize>,
    src: ReadSignal<Option<&'static str>>,
) -> impl IntoView {
    let content = view! { cx, <div>
        <iframe
            src=move || { if src().is_some() { hidden.set(false); } src().unwrap_or("") }
            allow="autoplay"
            style="width: 100%; height: 655px"
        ></iframe>
    </div> };

    view! { cx,
        <Window
            id="file-win"
            title="File Viewer".to_string()
            content=content
            start_pos=(60, 90)
            hidden=hidden
            z_idx=z_idx
        />
    }
}

#[component]
fn LoadingWindow(cx: Scope, hidden: RwSignal<bool>, z_idx: RwSignal<usize>) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let noun: &'static str = ABSTRACT_NOUNS.choose(&mut rng).unwrap();
    let title = format!("Loading {}", noun);
    let content = view! { cx, <div>
        <img src="/assets/infinity.svg" style="width: 100%; height: 100px" draggable="false"/>
    </div> };

    view! { cx,
        <Window
            id="loading-win"
            title=title
            content=content
            start_pos=(465, 325)
            hidden=hidden
            z_idx=z_idx
        />
    }
}

#[component]
fn AdWindow(cx: Scope, hidden: RwSignal<bool>, z_idx: RwSignal<usize>) -> impl IntoView {
    let content = view! { cx, <div style="height: 100px">
        <img
            src="/assets/ur-ad-here.png"
            style="height: 100px; width: 200px; image-rendering: pixelated"
            draggable="false"
        />
    </div> };

    view! { cx,
        <Window
            id="ad-win"
            title="Advertisement".to_string()
            content=content
            start_pos=(255, 22)
            hidden=hidden
            z_idx=z_idx
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
fn ExternalLink(
    cx: Scope,
    href: &'static str,
    display: &'static str,
    #[prop(default = false)] title_style: bool,
) -> impl IntoView {
    view! { cx,
        <a target="_blank" href=href class:title=title_style>
            {display}
            <span class="external-link"></span>
        </a>
    }
}
