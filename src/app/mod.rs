use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_event_listener, use_event_listener_with_options};
use rand::seq::SliceRandom;
use web_sys::AddEventListenerOptions;

pub mod home;
pub mod insa;
pub mod kalama_sin;
pub mod music;
pub mod portfolio;
pub mod tp;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Title text="etbcor's website"/>
        <Stylesheet id="leptos" href="/pkg/portfolio_site.css"/>

        // google fonts
        <Link href="https://fonts.googleapis.com" rel="preconnect"/>
        <Link href="https://fonts.gstatic.com" rel="preconnect" crossorigin=""/>
        <Link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,600;0,700;1,600;1,700&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Caveat:wght@700&display=swap" rel="stylesheet"/>


        // main router
        <Router>
            <main>
                <Routes>
                    <Route path="/"              view=home::HomePageEntry/>
                    <Route path="/portfolio"     view=portfolio::PortfolioPage/>
                    <Route path="/music"         view=music::MusicPage/>
                    <Route path="/tp"            view=tp::TokiPonaPage/>
                    <Route path="/tp/kalama_sin" view=kalama_sin::KalamaSinPage/>
                    <Route path="/insa"          view=insa::InsaPage/>
                    <Route path="/*any"          view=NotFoundPage/>
                </Routes>
                <Cyberpunk/>
            </main>
        </Router>
    }
}

#[component]
fn GoatCounter(cx: Scope, path: &'static str) -> impl IntoView {
    let settings = format!("{{\"path\": \"{}\"}}", path);
    view! { cx,
        <script
            data-goatcounter="https://etbcor.goatcounter.com/count"
            data-goatcounter-settings=settings
            async src="//gc.zgo.at/count.js">
        </script>
    }
}

pub enum WindowContent {
    Page(HtmlElement<html::Div>),
    Tabs(
        (
            RwSignal<&'static str>,
            Vec<(&'static str, HtmlElement<html::Div>)>,
        ),
    ),
}

pub enum WindowPos {
    Val((i32, i32)),
    Sig(RwSignal<(i32, i32)>),
    SigOffset(RwSignal<(i32, i32)>),
}

#[component]
fn Window(
    cx: Scope,
    id: &'static str,
    title: String,
    content: WindowContent,
    pos: WindowPos,
    size: RwSignal<(u32, u32)>,
    hidden: RwSignal<bool>,
    #[prop(default = true)] expandable: bool,
    #[prop(default = false)] expanded: bool,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    #[prop(default = false)] diag: bool,
    #[prop(default = false)] scroll: bool,
    #[prop(default = false)] rainbow: bool,
    #[prop(default = false)] diag_tp: bool,
) -> impl IntoView {
    let mut offset = false;
    let pos = match pos {
        WindowPos::Val(v) => create_rw_signal(cx, v),
        WindowPos::Sig(s) => s,
        WindowPos::SigOffset(s) => {
            offset = true;
            s
        }
    };
    let dpos = create_rw_signal(cx, (0, 0));

    let expanded = create_rw_signal(cx, expanded);
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

        let (x, y) = pos.get_untracked();
        dpos.set((x - e.client_x(), y - e.client_y()));
        let drag_cleanup = use_event_listener(cx, document(), ev::mousemove, move |e| {
            if !expanded.get_untracked() {
                let (dx, dy) = dpos.get_untracked();
                pos.set((e.client_x() + dx, e.client_y() + dy))
            }
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
        } else if title.starts_with("Obtain") {
            let split: Vec<_> = title.split_whitespace().collect();
            view! { cx, <p class="title">
                "Obtain "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else if title.starts_with("o pona") {
            let split: Vec<_> = title.split_whitespace().collect();
            view! { cx, <p class="title">
                "o "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else {
            view! { cx, <p class="title">{&title}</p> }
        }
    };

    let get_pos_size = move || {
        if !expanded() {
            format!(
                "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {}",
                pos().0,
                pos().1 + if offset { 35 } else { 0 },
                size().0,
                size().1 + 34, // add space for title
                this_z_idx()
            )
        } else {
            "".to_string()
        }
    };
    let get_content_size = move || {
        if !expanded() {
            format!("height: {}px", size().1)
        } else {
            "".to_string()
        }
    };
    let get_tab_size = move || {
        if !expanded() {
            format!("height: {}px", size().1 - 34)
        } else {
            "".to_string()
        }
    };

    let get_content = match content {
        WindowContent::Page(content) => view! { cx,
            <div class="win-content" style=get_content_size class:diag={diag} class:diag-tp={diag_tp} class:scroll={scroll} class:rainbow={rainbow}>
                { content }
            </div>
        },
        WindowContent::Tabs((active_tab, combined_vec)) => {
            let (titles, tabs): (Vec<_>, Vec<_>) = combined_vec
                .into_iter()
                .map(|(title, content)| {
                    (
                        view! { cx,
                            <div
                                class="title"
                                class:active=move || active_tab().eq(title)
                                on:mousedown=move |_| active_tab.set(title)
                                tabindex=0
                                on:keydown=move |k| if k.key() == "Enter" { active_tab.set(title) }>
                                { title }
                            </div>
                        },
                        view! { cx,
                            <div
                                class="tab-content"
                                tabindex=0
                                class:hidden=move || !active_tab().eq(title)>
                                { content }
                            </div>
                        },
                    )
                })
                .unzip();

            view! { cx,
                <div class="win-content" style=get_content_size>
                    <div class="tab-titlebar">{titles}</div>
                    <div class="tab-outer" style=get_tab_size class:scroll={scroll} class:diag={diag} class:diag-tp={diag_tp} class:rainbow={rainbow}>{tabs}</div>
                </div>
            }
        }
    };

    view! { cx,
        <div
            id=id
            class="win-outer"
            style=get_pos_size
            class:hidden=move || hidden()
            class:win-expanded=move || expanded()
        >
            <div
                class="win-titlebar"
                on:mousedown=drag
                tabindex=0
                on:keydown=move |k| {
                    if let Some(z_idx) = z_idx {
                        z_idx.update(|z| *z = *z + 1);
                        this_z_idx.set(z_idx());
                    }
                    if !expanded() {
                        if match k.key().as_str() {
                            "ArrowUp" => { pos.update(|(_, a)| *a = *a - 10); true }
                            "ArrowDown" => { pos.update(|(_, a)| *a = *a + 10); true }
                            "ArrowLeft" => { pos.update(|(a, _)| *a = *a - 10); true }
                            "ArrowRight" => { pos.update(|(a, _)| *a = *a + 10); true }
                            _ => false,
                        } { k.prevent_default() }
                    }
                }
            >
                { get_title }
                <div class="win-buttons">
                    { if expandable { Some(view! { cx, <a
                        class="win-expand"
                        title="expand window"
                        on:mousedown=move |_| expanded.update(|e| *e = !*e)
                        on:keydown=move |k| if k.key() == "Enter" { expanded.update(|e| *e = !*e) }
                        tabindex=0
                    ></a> }) } else { None } }
                    <a
                        class="win-close"
                        title="close window"
                        on:mousedown=move |_| hidden.set(true)
                        on:keydown=move |k| if k.key() == "Enter" { hidden.set(true) }
                        tabindex=0
                    ></a>
                </div>
            </div>
            { get_content }
        </div>
    }
}

#[component]
fn NotFoundPage(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    let loading = create_rw_signal(cx, false);

    view! { cx,
        <LoadingWindow pos=WindowPos::Val((20, 20)) size=(500, 500) hidden=loading variant=LoadingWindowVariant::PageNotFound/>
    }
}

#[component]
fn Footer(cx: Scope, items: Vec<(&'static str, RwSignal<bool>)>) -> impl IntoView {
    view! { cx, <footer>
        {
            items
                .into_iter()
                .map(|(title, hidden)| view! { cx,
                    <div
                        class="title win-minimized"
                        on:mousedown=move |_| hidden.set(false)
                        class:hidden=move || !hidden()
                        tabindex=0
                        on:keydown=move |k| if k.key() == "Enter" { hidden.set(false) }
                    >{title}</div>
                })
                .collect::<Vec<_>>()
        }
        <a class="title win-minimized favicon" href="/"></a>
    </footer> }
}

#[component]
fn Cyberpunk(cx: Scope) -> impl IntoView {
    view! { cx, <div id="cyberpunk">
        <video
            muted
            autoplay
            loop="true"
            poster="/assets/cyberpunk.png"
            on:contextmenu=move |e| e.prevent_default()>
            <source src="/assets/cyberpunk.webm" type="video/webm"/>
        </video>
    </div> }
}

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

#[derive(PartialEq, Copy, Clone)]
enum LoadingWindowVariant {
    Default,
    HomePageLink,
    #[allow(dead_code)]
    PageComingSoon,
    PageNotFound,
    StackOverflow,
    TP,
}

#[component]
fn LoadingWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    variant: LoadingWindowVariant,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);

    let mut rng = rand::thread_rng();
    let noun: &'static str = ABSTRACT_NOUNS.choose(&mut rng).unwrap();
    let title = {
        use LoadingWindowVariant::*;
        match variant {
            Default => format!("Loading {}", noun),
            HomePageLink => format!("Obtain {}", noun),
            PageComingSoon => "Page Coming Soon".to_string(),
            PageNotFound => "Page Not Found".to_string(),
            StackOverflow => "Uh-oh! The stack overflowed".to_string(),
            TP => "o pona".to_string(),
        }
    };

    let content = WindowContent::Page(view! { cx,
        <div
            class="loading-img"
            class:wait={variant == LoadingWindowVariant::Default}
            on:mousedown=move |_| leptos_router::use_navigate(cx)(if variant == LoadingWindowVariant::StackOverflow { "/insa" } else { "/" }, Default::default(),).unwrap()
            on:keydown=move |k| if k.key() == "Enter" { leptos_router::use_navigate(cx)(if variant == LoadingWindowVariant::StackOverflow { "/insa" } else { "/" }, Default::default(),).unwrap() }
            tabindex=0
            title="ale li pona"
        ></div>
    });

    view! { cx,
        <Window id="loading-win" title=title content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx rainbow=true/>
    }
}

#[component]
fn AdWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = WindowContent::Page(view! { cx, <div>
        <img src="/assets/ur-ad-here.png" draggable="false"/>
    </div> });

    view! { cx,
        <Window id="ad-win" title="Advertisement".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

enum Webring {
    Bucket,
    Kulupu,
}

#[component]
fn WebringWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    webring: Webring,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let id = match webring {
        Webring::Bucket => "bucket-webring-win",
        Webring::Kulupu => "kulupu-webring-win",
    };
    let content = WindowContent::Page(match webring {
        Webring::Bucket => view! { cx, <div style="margin-left: 16px; margin-right: 16px">
            <iframe
                src="https://webring.bucketfish.me/embed.html?name=etbcor"
                id="bucket-webring"
                style="width: 100%; height: 63px; border: none"
            ></iframe>
        </div> },
        Webring::Kulupu => {
            view! { cx, <div id="sike-pona" style="margin-left: 16px; margin-right: 16px; height: 90%">
                <link rel="stylesheet" href="https://sike.pona.la/embed.css"/>
                <span id="left">
                    <a href="https://sike.pona.la/jan/jan%20Itan/prev.html" id="prev">"← prev"</a>
                    </span>
                <span id="mid"><a href="https://sike.pona.la">
                    <img class="tokipona" src="https://sike.pona.la/assets/tokipona.svg"></img>
                    "sike pona"
                    <img class="tokipona" src="https://sike.pona.la/assets/tokipona.svg"></img>
                </a></span>
                <span id="right">
                <a href="https://sike.pona.la/jan/jan%20Itan/next.html" id="next">"next →"</a>
                </span>
            </div> }
        }
    });

    view! { cx,
        <Window id=id title="Webring".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

#[component]
fn JohnWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = WindowContent::Page(view! { cx, <div class="rainbow">
       <iframe
            src="https://john.citrons.xyz/embed?ref=etbcor.com"
            style="max-height: 94px; width: 100%; aspect-ratio: 732 / 94; border:none"
        ></iframe>
    </div> });

    view! { cx,
        <Window id="john-win" title="Johnvertisement".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

#[component]
fn LonelyWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = WindowContent::Page(view! { cx, <div tabindex=0>
    </div> });
    view! { cx,
        <Window id="lonely-win" title="A bit lonely...".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

#[component]
fn LinkWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    id: &'static str,
    title: String,
    bg_img: &'static str,
    src: &'static str,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    #[prop(default = false)] diag: bool,
    #[prop(default = false)] diag_tp: bool,
    #[prop(default = false)] external: bool,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = WindowContent::Page(if external {
        view! { cx, <div style="cursor: alias; text-align: center">
            <a href=src target="_blank" style="max-height: 100%">
                <img
                    src=bg_img
                    style="padding: 0px; max-height: 100%; max-width: 100%"
                    draggable=false
                />
            </a>
        </div> }
    } else {
        view! { cx, <div style="cursor: pointer; text-align: center">
            <img
                src=bg_img
                style="padding: 0px; max-height: 100%; max-width: 100%"
                draggable=false
                on:mousedown=move |_| leptos_router::use_navigate(cx)(src, Default::default()).unwrap()
                on:keydown=move |k| if k.key() == "Enter" { leptos_router::use_navigate(cx)(src, Default::default()).unwrap() }
                tabindex=0
            />
        </div> }
    });

    view! { cx,
        <Window id=id title=title content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx rainbow={!diag && !diag_tp} diag={diag} diag_tp={diag_tp}/>
    }
}

#[component]
fn ExternalLink(
    cx: Scope,
    href: &'static str,
    display: &'static str,
    #[prop(default = false)] title_style: bool,
    #[prop(default = false)] bold: bool,
) -> impl IntoView {
    if bold {
        view! { cx,
            <a class="external-link" target="_blank" href=href class:title=title_style>
                <b style="color: black">{display}</b>
                <span></span>
            </a>
        }
    } else {
        view! { cx,
            <a class="external-link" target="_blank" href=href class:title=title_style>
                {display}
                <span></span>
            </a>
        }
    }
}

#[component]
fn FileWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    src: ReadSignal<Option<&'static str>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = WindowContent::Page(view! { cx, <div style="width: 100%; height: 100%">
        <iframe
            src=move || { if src().is_some() { hidden.set(false); } src().unwrap_or("") }
            allow="autoplay"
            style="width: 100%; height: 100%"></iframe>
    </div> });

    view! { cx,
        <Window id="file-win" title="File Viewer".to_string() content=content pos=pos size=size hidden=hidden expanded=true z_idx=z_idx/>
    }
}

#[component]
fn FileLink(
    cx: Scope,
    src: &'static str,
    display: &'static str,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    view! { cx, <a href="" on:mousedown=move |_| file_win_src.set(Some(src)) on:keydown=move |k| if k.key() == "Enter" { file_win_src.set(Some(src)) }>{display}</a> }
}
