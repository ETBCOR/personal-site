use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_event_listener, use_event_listener_with_options};
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

        <Title text="etbcor's website"/>

        // Google fonts
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        <Link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,600;0,700;1,600;1,700&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Caveat:wght@700&display=swap" rel="stylesheet"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/portfolio" view=PortfolioPage/>
                    <Route path="/tp" view=TokiPonaPage/>
                    <Route path="/music" view=MusicPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let portfolio_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("\"Inspiration\"", loading_hidden),
        ("Portfolio", portfolio_hidden),
    ];

    view! { cx,
        <LoadingWindow pos=(20, 20) hidden=loading_hidden variant=LoadingWindowVariant::Default/>
        <LinkWindow pos=(285, 20) hidden=portfolio_hidden id="link-win" title="Portfolio".to_string() bg_img="/assets/file-icon.svg" src="/portfolio"/>
        <div style="height: 65px"></div> // spacer in narrow view
        <Footer items=footer_items/>
    }
}

#[component]
fn PortfolioPage(cx: Scope) -> impl IntoView {
    let about_hidden = create_rw_signal(cx, false);
    let education_hidden = create_rw_signal(cx, false);
    let skills_hidden = create_rw_signal(cx, false);
    let projects_hidden = create_rw_signal(cx, false);
    let file_hidden = create_rw_signal(cx, true);
    let loading_hidden = create_rw_signal(cx, false);
    let ad_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("About Me", about_hidden),
        ("Education", education_hidden),
        ("Projects", projects_hidden),
        ("Skills", skills_hidden),
        ("\"Inspiration\"", loading_hidden),
    ];
    let (file_src, set_file_src) = create_signal(cx, None);
    let z_idx = create_rw_signal(cx, 1);

    view! { cx,
        <LoadingWindow   pos=(465, 325) hidden=loading_hidden   z_idx=Some(z_idx) variant=LoadingWindowVariant::Default/>
        <AboutWindow     pos=(25, 20)   hidden=about_hidden     z_idx=Some(z_idx)/>
        <EducationWindow pos=(25, 210)  hidden=education_hidden z_idx=Some(z_idx)/>
        <SkillsWindow    pos=(735, 20)  hidden=skills_hidden    z_idx=Some(z_idx)/>
        <ProjectsWindow  pos=(735, 425) hidden=projects_hidden  z_idx=Some(z_idx) file_win_src=set_file_src/>
        <FileWindow      pos=(60, 90)   hidden=file_hidden      z_idx=Some(z_idx) src=file_src/>
        <AdWindow        pos=(255, 22)  hidden=ad_hidden        z_idx=Some(z_idx)/>
        <div style="height: 65px"></div> // spacer in narrow view
        <Footer items=footer_items/>
    }
}

#[component]
fn TokiPonaPage(cx: Scope) -> impl IntoView {}

#[component]
fn MusicPage(cx: Scope) -> impl IntoView {}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    let loading = create_rw_signal(cx, false);

    view! { cx,
        <LoadingWindow pos=(100, 100) hidden=loading variant=LoadingWindowVariant::NotFound/>
    }
}

#[component]
fn Footer(cx: Scope, items: Vec<(&'static str, RwSignal<bool>)>) -> impl IntoView {
    view! { cx, <footer>
        <a class="title win-minimized favicon" href="/"></a> {
            items
                .into_iter()
                .map(|(title, hidden)| view! { cx,
                    <div
                        class="title win-minimized"
                        on:mousedown={move |_| hidden.update(|h| *h = !*h)}
                        class:hidden={move || !hidden()}
                    >{title}</div>
                })
                .collect::<Vec<_>>()
        }
    </footer> }
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
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let x = create_rw_signal(cx, pos.0);
    let y = create_rw_signal(cx, pos.1);
    let dx = create_rw_signal(cx, 0);
    let dy = create_rw_signal(cx, 0);

    let this_z_idx = create_rw_signal(
        cx,
        if id.eq("ad-win") || !z_idx.is_some() {
            0
        } else {
            z_idx.unwrap().get_untracked()
        },
    );

    let drag = move |e: MouseEvent| {
        if let Some(z_idx) = z_idx {
            z_idx.update(|z| *z = *z + 1);
            this_z_idx.set(z_idx());
        }

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
fn LinkWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    id: &'static str,
    title: String,
    bg_img: &'static str,
    src: &'static str,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let nav = leptos_router::use_navigate(cx);
    let content = view! { cx, <div style="cursor: pointer" on:click=move |_| nav(src, Default::default()).unwrap()>
        <img src=bg_img style="padding: 10px" draggable=false/>
    </div> };

    view! { cx,
        <Window id=id title=title content=content pos=pos hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn AboutWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let content = view! { cx, <div> <p>
        "Hello! I'm Ethan, and was born in April 2001. "
        "I'm passionate about making software, writing music, and learning languages. You can contact me "
        <ExternalLink href="http://www.discordapp.com/users/207897365141520384" display="on discord"/>
        ", or "<ExternalLink href="mailto:etbcor@gmail.com" display="via email"/>
        ". Here's my "<ExternalLink href="https://www.github.com/ETBCOR" display="GitHub profile"/>". "
        <i>"I'm "<b>"etbcor"</b>" on most platforms!"</i>" Thanks for coming to my site!"
    </p> </div> };

    view! { cx,
        <Window id="about-win" title="About Me".to_string() content=content pos=pos hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn EducationWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let content = view! { cx, <div>
        <h4>"Bachelor's Degree in Computer Science"</h4>
        <div class="spaced">
            "I spent 2019-2023 at the "<ExternalLink href="https://www.uidaho.edu/" display="University of Idaho"/>
            " getting my "<ExternalLink href="https://catalog.uidaho.edu/courses/cs/" display="B.S.C.S."/>
            " as well as my "<ExternalLink href="https://catalog.uidaho.edu/courses/span/" display="Spanish minor"/>"."
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
        <Window id="education-win" title="Education".to_string() content=content pos=pos hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn SkillsWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
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
                    <li><span class="title">"Rust"</span>" is currently my favorite language. I learned about it in 2022 "
                        "and quickly started using it wherever it makes sense, so I'm at an intermediate / advanced level."</li>
                    <li><span class="title">"Python"</span>" isn't usually what I reach to first "
                        "for my projects, but I'm still proficient with it, and have used it for a few."</li>
                    <li><span class="title">""</span>"...and more, including "<span class="title">"JavaScript"</span>", "
                    <span class="title">"Java"</span>", "<span class="title">"SQL"</span>", "<span class="title">"C#"</span>
                    ", and even some "<span class="title">"ML"</span>" and "<span class="title">"Prolog"</span>"."</li>
                </ul></li>

                <li class="spaced">
                    <b>"Data structures and algorithms"</b>
                    ": my B.S.C.S. has given me a strong foundation in the fundamentals of Computer Science. "
                    "I am experienced in designing and analyzing various data structures and algorithms."
                </li>

                <li class="spaced">
                    "I'm farmiliar with "<b>"software development concepts"</b>
                    ", including code "<i>"modularity / testing / documentation / version control"</i>" techniques, "
                    <span class="title">"agile"</span>", "<span class="title">"continuous integration and delivery"
                    </span>" and "<span class="title">"the software development life cycle"</span>"."
                </li>

                <li class="spaced">
                    "I have a solid understanding of "<b>"networking"</b>" and "<b>"web development"</b>", including how to work with protocols like "
                    <span class="title">"IP"</span>", "<span class="title">"HTTP"</span>", "<span class="title">"TCP"</span>" and "<span class="title">"UDP"</span>
                    ", as well as technologies like "<span class="title">"databases"</span>", "<span class="title">"HTML"</span>", "<span class="title">"CSS"</span>" and "<span class="title">"JavaScript"</span>"."
                </li>

                <li class="spaced">
                    "I know how to write code for "<b>"embedded systems"</b>" using the principles of "
                    <span class="title">"real-time operating systems"</span>"."
                </li>

                <li>
                    "I also have a solid understanding of "<b>"computer architecture"</b>
                    " and "<b>"operating systems"</b>" concepts in general."
                </li>
            </ul></div> },
        ),
        (
            "Audio / Visual",
            view! { cx, <div><ul>
                <li><b>"Audio"</b><ul>
                    <li class="spaced">
                        "I purchased "<ExternalLink href="https://www.ableton.com/en/live/" display="Ableton Live" title_style=true/>
                        " in 2018, and I've been using it to make music in my free time ever since. I've honed my production skills "
                        "quite a bit, including a few years of experimenting with other DAWs before settling on Live. "
                        "I'm planning to release my first album soon!"
                    </li>
                    <li class="spaced">
                        "I volunteered at my church for several years in high school operating the sound booth for the live band, "
                        "so I'm comfortable running a large sound board (analog or digital) and with the basics of audio engineering."
                    </li>
                </ul></li>

                <li><b>"Visual"</b><ul>
                    <li class="spaced">
                        "I'm quite experienced with "
                        <ExternalLink href="https://www.adobe.com/products/aftereffects.html" display="After Effects" title_style=true/>
                        ". You can see some of what I've created with it on "
                        <ExternalLink href="https://www.instagram.com/ecridisedits/" display="my IG page"/>"."
                    </li>
                    <li>
                        "I've also volunteered at my church to run slides/lights for sermons, so I'm familiar with "
                        <ExternalLink href="https://renewedvision.com/propresenter/" display="ProPresenter" title_style=true/>
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
        <Window id="skills-win" title="Skills".to_string() content=content tabs=Some((active_tab, tabs)) pos=pos hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn ProjectsWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
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
                    <b>"CS415 | Computational Biology: Sequence Alignment"</b><br/>
                    "Description: \"Design and analyze algorithms that address the computational problems posed by biological sequence data, "
                    "such as DNA or protein sequences.\" Projects:"<br/>
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
                    <b>"CS445 | Compiler Design"</b><br/>
                    "In "<ExternalLink href="http://www2.cs.uidaho.edu/~mdwilder/cs445/" display="this class"/>
                    " I fully implemented a compiler for the "<span style="white-space: nowrap">"\"C minus\""</span>" langauge (grammar specification "
                    <FileLink src="https://drive.google.com/file/d/12o5aSATedS28eJwsHIOHR7uf3DdZY20V/preview" display="here" file_win_src=fws/>
                    "). This is probably the largest solo project I've completed so far. Repository "
                    <ExternalLink href="https://github.com/ETBCOR/cs445" display="here"/>"."
                </li>

                <li class="spaced">
                    <b>"CS452 | Real-Time Operating Systems"</b><br/>
                    "I created multiple programs for embedded systems (Feather RP2040 & ESP32) in this class, including a basic IOT device with its own webserver. Repository "
                    <ExternalLink href="https://github.com/ETBCOR/cs452/" display="here"/>"."
                </li>

                <li class="spaced">
                    <b>"CS470 | Artificial Intelligence"</b><br/>
                    "This class taugh common concepts and techniques involved in artificial intelligence. Projects:"<br/>
                    <FileLink src="https://drive.google.com/file/d/1ICaQOsGKwJ7RfE21xBHvozQkfQGkw43G/preview" display="Pathfinding Algorithms" file_win_src=fws/>
                    " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj1" display="Github Repository"/>
                    <br/>
                    <FileLink src="https://drive.google.com/file/d/1fK-F2X7uwnOk8CrDosopO1pRl6xlBc1u/preview" display="Connect-4 Bot Using Minmax" file_win_src=fws/>
                    " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj2" display="Github Repository"/>
                    <br/>
                    <FileLink src="https://drive.google.com/file/d/1Qr5B0yZ8s3aY3Ywdd4KCYq_7y5bXfCTg/preview" display="Map Coloring Algorithms" file_win_src=fws/>
                    " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj3" display="Github Repository"/>
                    <br/>
                    <FileLink src="https://drive.google.com/file/d/1ysXZTxxRYNOqZDYkrTWZj6VWc2TndJZR/preview" display="Modeling Genealogy in Prolog" file_win_src=fws/>
                </li>

                <li class="spaced">
                    <b>"CS475 | Machine Learning"</b><br/>
                    "In "<ExternalLink href="http://marvin.cs.uidaho.edu/Teaching/CS475/index.html" display="this class"/>
                    " I completed 8 assignments machine learning topics of varying difficulty. Although the repository is a bit messy, the link is "
                    <ExternalLink href="https://github.com/ETBCOR/cs475" display="here"/>"."
                </li>

                <li>
                    <b>"CS480 & CS481 | Senior Capstone Design"</b><br/>
                    "For my capstone project I designed calibration software for a laser communication device made by "
                    <ExternalLink href="https://www.hansenphotonics.com/" display="Hansen Photonics Inc"/>
                    " on a team with three other CS majors. The resulting software is simple yet effective. "
                    "The creation process is well documented, but the repository is private; contact me if you're interested in seeing it."
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
                    "with Ableton Live in my free time, but I haven't released anything yet."
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
        <Window id="projects-win" title="Projects".to_string() content=content tabs=Some((active_tab, tabs)) pos=pos hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn FileWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
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
        <Window id="file-win" title="File Viewer".to_string() content=content pos=pos hidden=hidden z_idx=z_idx/>
    }
}

enum LoadingWindowVariant {
    Default,
    NotFound,
}

#[component]
fn LoadingWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    variant: LoadingWindowVariant,
) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let noun: &'static str = ABSTRACT_NOUNS.choose(&mut rng).unwrap();
    let mut title = format!("Loading {}", noun);
    let nav = leptos_router::use_navigate(cx);

    let content = match variant {
        LoadingWindowVariant::Default => {
            view! { cx, <div style="cursor: wait">
                <img src="/assets/infinity.svg" style="width: 100%; height: 100px" draggable="false" title="ale li pona"/>
            </div> }
        }
        LoadingWindowVariant::NotFound => {
            title = "Page Not Found".to_string();
            view! { cx, <div style="cursor: pointer" on:click=move |_| nav("/", Default::default()).unwrap()>
                <img src="/assets/infinity.svg" style="width: 100%; height: 100px" draggable="false"/>
            </div> }
        }
    };

    view! { cx,
        <Window id="loading-win" title=title content=content pos=pos hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn AdWindow(
    cx: Scope,
    pos: (i32, i32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let content = view! { cx, <div style="height: 100px">
        <img
            src="/assets/ur-ad-here.png"
            style="height: 100px; width: 200px; image-rendering: pixelated"
            draggable="false"
        />
    </div> };

    view! { cx,
        <Window id="ad-win" title="Advertisement".to_string() content=content pos=pos hidden=hidden z_idx=z_idx/>
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
        <a class="external-link" target="_blank" href=href class:title=title_style>
            {display}
            <span></span>
        </a>
    }
}
