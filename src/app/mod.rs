use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_event_listener, use_event_listener_with_options, UseEventListenerOptions};
use rand::seq::SliceRandom;

pub mod home;
pub mod kalama_sin;
pub mod music;
pub mod nasin_nanpa;
pub mod pakala;
pub mod portfolio;
pub mod tp;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="etbcor's website"/>
        <Stylesheet id="leptos" href="/pkg/personal_site.css"/>

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
                    <Route path="/"               view=home::HomePageWrap/>
                    <Route path="/portfolio"      view=portfolio::PortfolioPage/>
                    <Route path="/music"          view=music::MusicPage/>
                    <Route path="/tp"             view=tp::TokiPonaPage/>
                    <Route path="/tp/kalama_sin"  view=kalama_sin::KalamaSinPage/>
                    <Route path="/tp/nasin_nanpa" view=nasin_nanpa::NasinNanpaPage/>
                    <Route path="/pakala"         view=pakala::PakalaPage/>
                    <Route path="/*any"           view=NotFoundPage/>
                </Routes>
                <Cyberpunk/>
            </main>
        </Router>
    }
}

#[component]
fn GoatCounter(path: &'static str) -> impl IntoView {
    let settings = format!("{{\"path\": \"{}\"}}", path);
    view! {
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

#[derive(Copy, Clone)]
pub enum WindowPos {
    Val((i32, i32)),
    Sig(RwSignal<(i32, i32)>),
    OffsetSignal(RwSignal<(i32, i32)>),
}

#[component]
fn Window(
    id: &'static str,
    title: String,
    content: WindowContent,
    pos: WindowPos,
    size: RwSignal<(u32, u32)>,
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    #[prop(default = true)] expandable: bool,
    #[prop(default = false)] expanded: bool,
    #[prop(default = None)] min_button: Option<(RwSignal<bool>, RwSignal<(u32, u32)>)>,
    #[prop(default = false)] diag: bool,
    #[prop(default = false)] scroll: bool,
    #[prop(default = false)] rainbow: bool,
    #[prop(default = false)] diag_tp: bool,
) -> impl IntoView {
    let mut offset = false;
    let pos = match pos {
        WindowPos::Val(v) => create_rw_signal(v),
        WindowPos::Sig(s) => s,
        WindowPos::OffsetSignal(s) => {
            offset = true;
            s
        }
    };
    let dpos = create_rw_signal((0, 0));

    let expanded = create_rw_signal(expanded);
    let this_z_idx = create_rw_signal(
        if id.eq("ad-win") || id.eq("john-win") || !z_idx.is_some() {
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
        let drag_cleanup = use_event_listener(document(), ev::mousemove, move |e| {
            if !expanded.get_untracked() {
                let (dx, dy) = dpos.get_untracked();
                pos.set((e.client_x() + dx, e.client_y() + dy))
            }
        });

        let once_opt = UseEventListenerOptions::default();
        once_opt.once(true);
        let _ = use_event_listener_with_options(
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
            view! { <p class="title">
                "Loading "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else if title.starts_with("Obtain") {
            let split: Vec<_> = title.split_whitespace().collect();
            view! { <p class="title">
                "Obtain "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else if title.starts_with("o pona") {
            let split: Vec<_> = title.split_whitespace().collect();
            view! { <p class="title">
                "o "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else {
            view! { <p class="title">{&title}</p> }
        }
    };

    let get_pos_size = move || {
        if !expanded() {
            format!(
                "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {}",
                pos().0,
                pos().1 + if offset { 35 } else { 0 }, // add space for meta title
                size().0,
                size().1 + 39, // add space for title
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
        WindowContent::Page(content) => view! {
            <div class="win-content" style=get_content_size class:diag={diag} class:diag-tp={diag_tp} class:scroll={scroll} class:rainbow={rainbow}>
                { content }
            </div>
        },
        WindowContent::Tabs((active_tab, combined_vec)) => {
            let (titles, tabs): (Vec<_>, Vec<_>) = combined_vec
                .into_iter()
                .map(|(title, content)| {
                    (
                        view! {
                            <div
                                class="title"
                                class:active=move || active_tab().eq(title)
                                on:mousedown=move |_| active_tab.set(title)
                                tabindex=0
                                on:keydown=move |k| if k.key() == "Enter" { active_tab.set(title) }>
                                { title }
                            </div>
                        },
                        view! {
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

            view! {
                <div class="win-content" style=get_content_size>
                    <div class="tab-titlebar">{titles}</div>
                    <div class="tab-outer" style=get_tab_size class:scroll={scroll} class:diag={diag} class:diag-tp={diag_tp} class:rainbow={rainbow}>{tabs}</div>
                </div>
            }
        }
    };

    view! {
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
                    { match min_button {
                        Some((deeper, size)) => { Some(view! { <a
                            class="win-min"
                            title="minimize window"
                            on:mousedown=move |_| {deeper.set(false); size.set((200, 437))}
                            on:keydown=move |k| if k.key() == "Enter" {deeper.set(false); size.set((200, 437))}
                            tabindex=0
                        ></a> }) }
                        None => { None } }
                    }
                    { if expandable { Some(view! { <a
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
fn NotFoundPage() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    let loading = create_rw_signal(false);

    view! {
        <LoadingWindow pos=WindowPos::Val((20, 20)) size=(500, 500) hidden=loading variant=LoadingWindowVariant::PageNotFound/>
    }
}

#[component]
fn Footer(
    items: Vec<(&'static str, RwSignal<bool>)>,
    #[prop(default = false)] nasa: bool,
) -> impl IntoView {
    view! {
        <div id="ale-li-pona"></div>
        <div id="nasa-a-a-a" class:hidden={!nasa}></div>
        <footer>
            {
                items
                    .into_iter()
                    .map(|(title, hidden)| view! {
                        <div
                            class="title win-minimized"
                            on:mousedown=move |_| hidden.set(false)
                            class:hidden=move || !hidden()
                            tabindex=0
                            on:keydown=move |k| if k.key() == "Enter" { hidden.set(false) }
                            title="open window"
                        >{title}</div>
                    })
                    .collect::<Vec<_>>()
            }
            <a class="title win-minimized favicon" href="/"></a>
        </footer>
    }
}

#[component]
fn Cyberpunk() -> impl IntoView {
    view! { <div id="cyberpunk">
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
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    variant: LoadingWindowVariant,
) -> impl IntoView {
    let size = create_rw_signal(size);

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

    let content = WindowContent::Page(view! {
        <div
            class="loading-img"
            class:wait={variant == LoadingWindowVariant::Default}
            on:mousedown=move |_| leptos_router::use_navigate()(if variant == LoadingWindowVariant::StackOverflow { "/pakala" } else { "/" }, Default::default(),)
            on:keydown=move |k| if k.key() == "Enter" { leptos_router::use_navigate()(if variant == LoadingWindowVariant::StackOverflow { "/pakala" } else { "/" }, Default::default(),) }
            tabindex=0
            title="ale li pona"
        ></div>
    });

    view! {
        <Window id="loading-win" title=title content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx rainbow=true/>
    }
}

#[component]
fn AdWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let content = WindowContent::Page(view! { <div style="cursor: wait">
        <img src="/assets/ur-ad-here.png" draggable="false"/>
    </div> });

    view! {
        <Window id="ad-win" title="Advertisement".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

enum Webring {
    Bucket,
    SikePona,
}

#[component]
fn WebringWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    webring: Webring,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let id = match webring {
        Webring::Bucket => "bucket-webring-win",
        Webring::SikePona => "sike-pona-webring-win",
    };
    let title = match webring {
        Webring::Bucket => "Bucket Webring",
        Webring::SikePona => "sike pona",
    }
    .to_string();

    let content = WindowContent::Page(match webring {
        Webring::Bucket => view! { <div style="margin-left: 16px; margin-right: 16px">
            <iframe
                src="https://webring.bucketfish.me/embed.html?name=etbcor"
                id="bucket-webring"
                style="width: 100%; height: 63px; border: none"
            ></iframe>
        </div> },
        Webring::SikePona => {
            view! { <div id="sike-pona" style="margin-left: 16px; margin-right: 16px; height: 90%; color: #c8ace5">
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

    view! {
        <Window id=id title=title content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

#[component]
fn JohnWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let content = WindowContent::Page(view! { <div class="rainbow">
       <iframe
            src="https://john.citrons.xyz/embed?ref=etbcor.com"
            style="max-height: 94px; width: 100%; aspect-ratio: 732 / 94; border:none"
        ></iframe>
    </div> });

    view! {
        <Window id="john-win" title="Johnvertisement".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

#[component]
fn LonelyWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let content = WindowContent::Page(view! { <div tabindex=0>
    </div> });
    view! {
        <Window id="lonely-win" title="A bit lonely...".to_string() content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx/>
    }
}

#[component]
fn LinkWindow(
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
    let size = create_rw_signal(size);
    let content = WindowContent::Page(if external {
        view! { <div style="cursor: alias; text-align: center; height: 100%">
            <a href=src target="_blank" style="height: 100%">
                <img
                    src=bg_img
                    style="padding: 0px; height: 100%; max-width: 100%"
                    draggable=false
                />
            </a>
        </div> }
    } else {
        view! { <div style="cursor: pointer; text-align: center; height: 100%">
            <img
                src=bg_img
                style="padding: 0px; height: 100%; max-width: 100%"
                draggable=false
                on:mousedown=move |_| leptos_router::use_navigate()(src, Default::default())
                on:keydown=move |k| if k.key() == "Enter" { leptos_router::use_navigate()(src, Default::default()) }
                tabindex=0
            />
        </div> }
    });

    view! {
        <Window id=id title=title content=content pos=pos size=size hidden=hidden expandable=false z_idx=z_idx rainbow={!diag && !diag_tp} diag={diag} diag_tp={diag_tp}/>
    }
}

#[component]
fn ExternalLink(
    href: &'static str,
    display: &'static str,
    #[prop(default = false)] title_style: bool,
    #[prop(default = false)] bold: bool,
) -> impl IntoView {
    if bold {
        view! {
            <a class="external-link" target="_blank" href=href class:title=title_style>
                <b style="color: black">{display}</b>
                <span></span>
            </a>
        }
    } else {
        view! {
            <a class="external-link" target="_blank" href=href class:title=title_style>
                {display}
                <span></span>
            </a>
        }
    }
}

#[component]
fn FileWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    src: ReadSignal<Option<&'static str>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let content = WindowContent::Page(view! { <div style="width: 100%; height: 100%">
        <iframe
            src=move || { if src().is_some() { hidden.set(false); } src().unwrap_or("") }
            allow="autoplay"
            style="width: 100%; height: 100%"></iframe>
    </div> });

    view! {
        <Window id="file-win" title="File Viewer".to_string() content=content pos=pos size=size hidden=hidden expanded=true z_idx=z_idx/>
    }
}

#[component]
fn FileLink(
    src: &'static str,
    display: &'static str,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    view! { <a href="" on:mousedown=move |_| file_win_src.set(Some(src)) on:keydown=move |k| if k.key() == "Enter" { file_win_src.set(Some(src)) }>{display}</a> }
}
