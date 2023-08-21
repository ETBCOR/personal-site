use crate::app::music::MusicLinkWindow;
use crate::app::{
    AdWindow, Footer, GoatCounter, JohnWindow, LinkWindow, LoadingWindow, LoadingWindowVariant,
    WebringWindow, Window, WindowContent,
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
        <LinkWindow    pos=(280, 20   +y_offset) size=(170, 220) hidden=portfolio_hidden z_idx=z_idx id="portfolio-link-win" title="Portfolio".to_string() bg_img="/assets/file-icon.svg"       src="/portfolio" diag=true/>
        <MusicLinkWindow pos=(20, 262 +y_offset) size=(225, 225) hidden=music_hidden     z_idx=z_idx/> // music link window
        <LinkWindow    pos=(280, 309  +y_offset) size=(170, 178) hidden=tp_hidden        z_idx=z_idx id="tp-link-win"        title="toki pona".to_string() bg_img="/assets/itan.svg"            src="/tp" diag_tp=true/>
        <WebringWindow pos=(20, 559   +y_offset) size=(430, 70)  hidden=webring_hidden   z_idx=z_idx/>
        <AdWindow      pos=(485, 20   +y_offset) size=(200, 100) hidden=ad_hidden        z_idx=z_idx/>
        <JohnWindow    pos=(20, 701   +y_offset) size=(665, 82)  hidden=john_hidden      z_idx=z_idx/>
        <MetaWindow    pos=(485, 192  +y_offset) size=(200, 437) hidden=meta_hidden      z_idx=z_idx recursions={recursions + 1}/>
        <div class:hidden=move || {recursions > 0}>
            <div style="height: 65px"></div> // large spacer
            <Footer items=footer_items/>     // footer
        </div>
        <div class:hidden=move || {recursions > 0} style="height: 20px"></div> // small spacer
    }
}

#[component]
pub fn HomePageEntry(cx: Scope) -> impl IntoView {
    view! { cx,
        <HomePage recursions=0/>
        <GoatCounter path="/"/>
    }
}

const STACK_OVERFLOW_LIMIT: usize = 8;
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
    let deeper = create_rw_signal(cx, false);
    let content = WindowContent::Page(view! { cx, <div>
        <div
            class:scroll=move || deeper()
            class:hidden=move || deeper()
            on:click=move |_| {deeper.set(true); size.set((720, 844))}
            on:keydown=move |k| if k.key() == "Enter" {deeper.set(true); size.set((720, 844))}
            tabindex=0
        >
            <video
                style="cursor: alias"
                muted
                autoplay
                loop="true"
                poster="/assets/o-tawa-insa.svg"
                on:contextmenu=move |e| e.prevent_default()>
                <source src="/assets/o-tawa-insa.webm" type="video/webm"/>
            </video>
        </div>
        <div class:hidden=move || !deeper()>
            {
                if recursions <= STACK_OVERFLOW_LIMIT {
                    view! { cx, <div> <HomePage recursions=recursions/> </div> }
                } else {
                    view! { cx, <div> <LoadingWindow pos=(20, 55) size=(300, 100) hidden=hidden variant=LoadingWindowVariant::StackOverflow/> </div> }
                }
            }
        </div>
    </div> });

    view! { cx,
        <Window id="meta-win" title="Meta, man...".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx rainbow=true/>
    }
}
