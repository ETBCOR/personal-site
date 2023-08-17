use crate::app::{
    AdWindow, Cyberpunk, Footer, JohnWindow, LoadingWindow, LoadingWindowVariant, WebringWindow,
    Window,
};
use leptos::*;

#[component]
fn HomePage(cx: Scope, recursions: usize) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let portfolio_hidden = create_rw_signal(cx, false);
    let music_hidden = create_rw_signal(cx, false);
    let tp_hidden = create_rw_signal(cx, false);
    let webring_hidden = create_rw_signal(cx, false);
    let meta_hidden = create_rw_signal(cx, false);
    let ad_hidden = create_rw_signal(cx, false);
    let john_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("\"Inspiration\"", loading_hidden),
        ("Portfolio", portfolio_hidden),
        ("Music", music_hidden),
        ("toki pona", tp_hidden),
        ("Webring", webring_hidden),
        ("Meta", meta_hidden),
        ("Johnvertisement", john_hidden),
    ];
    let z_idx = if recursions == 0 {
        Some(create_rw_signal(cx, 1))
    } else {
        None
    };
    let y_offset = if recursions == 0 { 0 } else { 35 };

    view! { cx,
        <LoadingWindow pos=(20, 20    +y_offset) size=(225, 170) hidden=loading_hidden   z_idx=z_idx variant=LoadingWindowVariant::Default/>
        <LinkWindow    pos=(280, 20   +y_offset) size=(170, 220) hidden=portfolio_hidden z_idx=z_idx id="portfolio-link-win" title="Portfolio".to_string() bg_img="/assets/file-icon.svg"       src="/portfolio"/>
        <MusicLinkWindow pos=(20, 262 +y_offset) size=(225, 225) hidden=music_hidden     z_idx=z_idx/> // music link window
        <LinkWindow    pos=(280, 309  +y_offset) size=(170, 178) hidden=tp_hidden        z_idx=z_idx id="tp-link-win"        title="toki pona".to_string() bg_img="/assets/itan.svg"            src="/tp"/>
        <WebringWindow pos=(20, 559   +y_offset) size=(430, 70)  hidden=webring_hidden   z_idx=z_idx/>
        <AdWindow      pos=(485, 20   +y_offset) size=(200, 100) hidden=ad_hidden        z_idx=z_idx/>
        <JohnWindow    pos=(20, 701   +y_offset) size=(665, 82)  hidden=john_hidden      z_idx=z_idx/>
        <MetaWindow    pos=(485, 192  +y_offset) size=(200, 437) hidden=meta_hidden      z_idx=z_idx recursions={recursions + 1}/>
        <div style="height: 65px"></div> // spacer in narrow view
        <div class:hidden=move || {recursions > 0}><Footer items=footer_items/></div>
    }
}

#[component]
pub fn HomePageEntry(cx: Scope) -> impl IntoView {
    view! { cx,
        // <audio autoplay><source src="/assets/Aquarius.wav" type="audio/wav"/></audio>
        <HomePage recursions=0/>
        <Cyberpunk/>
    }
}

#[component]
fn MetaWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    recursions: usize,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = if recursions < 8 {
        let deeper = create_rw_signal(cx, false);
        view! { cx, <div>
            <video
                style="cursor: alias"
                muted
                autoplay
                loop="true"
                poster="/assets/o-tawa-insa.svg"
                class:hidden=move || deeper()
                on:click=move |_| {deeper.set(true); size.set((720, 844))}>
                on:contextmenu=move |e| e.prevent_default()>
                <source src="/assets/o-tawa-insa.webm" type="video/webm"/>
            </video>
            <div class:hidden=move || !deeper()>
                <HomePage recursions=recursions/>
            </div>
        </div> }
    } else {
        view! { cx, <div>
            <LoadingWindow pos=(0, 150) size=(200, 500) hidden=hidden variant=LoadingWindowVariant::StackOverflow/>
        </div> }
    };

    view! { cx,
        <Window id="meta-win" title="Meta, man...".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx rainbow=true/>
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
fn MusicLinkWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let nav = leptos_router::use_navigate(cx);
    let content = view! { cx, <div style="height: 227px; cursor: pointer">
        <video
            style="width: 100%"
            muted
            autoplay
            loop="true"
            poster="/assets/music-icon.png"
            on:click=move |_| nav("/music", Default::default()).unwrap()
            on:contextmenu=move |e| e.prevent_default()>
            <source src="/assets/music-icon.webm" type="video/webm"/>
        </video>
    </div> };

    view! { cx,
        <Window id="music-link-win" title="Music".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx rainbow=true/>
    }
}
