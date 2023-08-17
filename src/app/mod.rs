use leptos::{ev::MouseEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_event_listener, use_event_listener_with_options};
use rand::seq::SliceRandom;
use web_sys::AddEventListenerOptions;

pub mod home;
pub mod music;
pub mod portfolio;
pub mod tp;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio_site.css"/>

        <Title text="etbcor's website"/>

        // google fonts
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        <Link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,600;0,700;1,600;1,700&display=swap" rel="stylesheet"/>
        <Link href="https://fonts.googleapis.com/css2?family=Caveat:wght@700&display=swap" rel="stylesheet"/>

        // main router
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=home::HomePageEntry/>
                    <Route path="/portfolio" view=portfolio::PortfolioPage/>
                    <Route path="/tp" view=tp::TokiPonaPage/>
                    <Route path="/music" view=music::MusicPage/>
                    <Route path="/*any" view=NotFoundPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Window(
    cx: Scope,
    id: &'static str,
    title: String,
    content: HtmlElement<html::Div>,
    #[prop(default = None)] tabs: Tabs,
    pos: (i32, i32),
    size: RwSignal<(u32, u32)>,
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    #[prop(default = false)] scroll: bool,
    #[prop(default = false)] rainbow: bool,
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
        } else if title.starts_with("Obtain") {
            let split: Vec<_> = title.split_whitespace().collect();
            view! { cx, <p class="title">
                "Obtain "
                <span style="font-family: 'Cedarville Cursive', cursive; font-size: 12pt; font-style: oblique">{
                    split[1].to_string()
                }</span>
            </p> }
        } else {
            view! { cx, <p class="title">{&title}</p> }
        }
    };

    let get_pos = move || format!("left: {}px; top: {}px; z-index: {}", x(), y(), this_z_idx());
    let get_size = move || format!("width: {}px; height: {}px", size().0, size().1);

    let get_content = match tabs {
        Some((active_tab, combined_vec)) => {
            let (titles, tabs): (Vec<_>, Vec<_>) = combined_vec
                .into_iter()
                .map(|(title, content)| {
                    (
                        view! { cx,
                            <div
                                class="title"
                                class:active=move || active_tab().eq(title)
                                on:click=move |_| active_tab.set(title)>
                                { title }
                            </div>
                        },
                        view! { cx,
                            <div
                                class="tab-content"
                                class:hidden=move || !active_tab().eq(title)>
                                { content }
                            </div>
                        },
                    )
                })
                .unzip();

            view! { cx,
                <div class="win-content" class:scroll={scroll} class:rainbow={rainbow} style=get_size>
                    <div class="tab-titlebar">{titles}</div>
                    <div class="tab-outer">{tabs}</div>
                </div>
            }
        }
        None => view! { cx,
            <div class="win-content" class:scroll={scroll} class:rainbow={rainbow} style=get_size>
                { content }
            </div>
        },
    };

    view! { cx,
        <div
            id=id
            class="win-outer"
            class:hidden={move || hidden()}
            style=get_pos>
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
fn NotFoundPage(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    let loading = create_rw_signal(cx, false);

    view! { cx,
        <LoadingWindow pos=(20, 20) size=(500, 500) hidden=loading variant=LoadingWindowVariant::PageNotFound/>
        <Cyberpunk/>
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
                        on:mousedown=move |_| hidden.update(|h| *h = !*h)
                        class:hidden=move || !hidden()
                    >{title}</div>
                })
                .collect::<Vec<_>>()
        }
        <a class="title win-minimized favicon" href="/"></a>
    </footer> }
}
type Tabs = Option<(
    RwSignal<&'static str>,
    Vec<(&'static str, HtmlElement<html::Div>)>,
)>;

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

enum LoadingWindowVariant {
    Default,
    HomePageLink,
    PageComingSoon,
    PageNotFound,
    StackOverflow,
}

#[component]
fn LoadingWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    variant: LoadingWindowVariant,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let mut rng = rand::thread_rng();
    let noun: &'static str = ABSTRACT_NOUNS.choose(&mut rng).unwrap();
    let mut title = format!("Loading {}", noun);
    let nav = leptos_router::use_navigate(cx);

    let content = match variant {
        LoadingWindowVariant::Default => {
            view! { cx, <div class="loading-img" on:click=move |_| nav("/", Default::default()).unwrap() title="ale li pona" style="cursor: wait"></div> }
        }
        LoadingWindowVariant::HomePageLink => {
            title = format!("Obtain {}", noun);
            view! { cx, <div class="loading-img" on:click=move |_| nav("/", Default::default()).unwrap() title="ale li pona" style="cursor: pointer"></div> }
        }
        LoadingWindowVariant::PageComingSoon => {
            title = "Page Coming Soon".to_string();
            view! { cx, <div class="loading-img" on:click=move |_| nav("/", Default::default()).unwrap() title="ale li pona" style="cursor: pointer"></div> }
        }
        LoadingWindowVariant::PageNotFound => {
            title = "Page Not Found".to_string();
            view! { cx, <div class="loading-img" on:click=move |_| nav("/", Default::default()).unwrap() title="ale li pona" style="cursor: pointer"></div> }
        }
        LoadingWindowVariant::StackOverflow => {
            title = "Uh-oh! The stack overflowed".to_string();
            view! { cx, <div class="loading-img" on:click=move |_| nav("/", Default::default()).unwrap() title="ale li pona" style="cursor: pointer"></div> }
        }
    };

    view! { cx,
        <Window id="loading-win" title=title content=content pos=pos size=size hidden=hidden z_idx=z_idx rainbow=true/>
    }
}

#[component]
fn AdWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = view! { cx, <div>
        <img class="rainbow" src="/assets/ur-ad-here.png" draggable="false" />
    </div> };

    view! { cx,
        <Window id="ad-win" title="Advertisement".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn WebringWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = view! { cx, <div style="margin-left: 16px; margin-right: 16px">
       <iframe
        src="https://webring.bucketfish.me/embed.html?name=etbcor"
        id="bucket-webring"
        style="width: 100%; height: 63px; border: none"
    ></iframe>
    </div> };

    view! { cx,
        <Window id="webring-win" title="Webring".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn JohnWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = view! { cx, <div>
       <iframe
            src="https://john.citrons.xyz/embed?ref=example.com"
            style="padding: 0px; width: 100%; height: 100%; border:none"
        ></iframe>
    </div> };

    view! { cx,
        <Window id="john-win" title="Johnvertisement".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn LinkWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    id: &'static str,
    title: String,
    bg_img: &'static str,
    src: &'static str,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let nav = leptos_router::use_navigate(cx);
    let content = view! { cx, <div style="cursor: pointer; text-align: center" on:click=move |_| nav(src, Default::default()).unwrap()>
        <img src=bg_img style="padding: 10px" draggable=false/>
    </div> };

    view! { cx,
        <Window id=id title=title content=content pos=pos size=size hidden=hidden z_idx=z_idx rainbow=true/>
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
