use crate::app::music::MusicLinkWindow;
use crate::app::{
    AdWindow, Footer, GoatCounter, JohnWindow, LinkWindow, LoadingWindow, LoadingWindowVariant,
    Webring, WebringWindow, Window, WindowContent, WindowPos,
};
use leptos::*;

#[component]
fn HomePage(
    cx: Scope,
    recursions: usize,
    sigs: Vec<(RwSignal<(i32, i32)>, RwSignal<bool>)>,
) -> impl IntoView {
    let (loading_pos, loading_hidden) = sigs[0];
    let (portfolio_pos, portfolio_hidden) = sigs[1];
    let (music_pos, music_hidden) = sigs[2];
    let (tp_pos, tp_hidden) = sigs[3];
    let (webring_pos, webring_hidden) = sigs[4];
    let (meta_pos, meta_hidden) = sigs[5];
    let (ad_pos, ad_hidden) = sigs[6];
    let (john_pos, john_hidden) = sigs[7];

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

    view! { cx,
        <LoadingWindow   pos=WindowPos::SigOffset(loading_pos)   size=(225, 170) hidden=loading_hidden   z_idx=z_idx variant=LoadingWindowVariant::Default/>
        <LinkWindow      pos=WindowPos::SigOffset(portfolio_pos) size=(170, 220) hidden=portfolio_hidden z_idx=z_idx id="portfolio-link-win" title="Portfolio".to_string() bg_img="/assets/file-icon.svg"       src="/portfolio" diag=true/>
        <MusicLinkWindow pos=WindowPos::SigOffset(music_pos)     size=(225, 225) hidden=music_hidden     z_idx=z_idx/> // music link window
        <LinkWindow      pos=WindowPos::SigOffset(tp_pos)        size=(170, 178) hidden=tp_hidden        z_idx=z_idx id="tp-link-win"        title="toki pona".to_string() bg_img="/assets/itan.svg"            src="/tp" diag_tp=true/>
        <WebringWindow   pos=WindowPos::SigOffset(webring_pos)   size=(430, 70)  hidden=webring_hidden   z_idx=z_idx webring=Webring::Bucket/>
        <AdWindow        pos=WindowPos::SigOffset(ad_pos)        size=(200, 100) hidden=ad_hidden        z_idx=z_idx/>
        <JohnWindow      pos=WindowPos::SigOffset(john_pos)      size=(665, 82)  hidden=john_hidden      z_idx=z_idx/>
        <MetaWindow      pos=WindowPos::SigOffset(meta_pos)      size=(200, 437) hidden=meta_hidden      z_idx=z_idx recursions={recursions + 1} sigs=sigs/>
        <div class:hidden=move || {recursions > 0}>
            <div style="height: 65px"></div> // large spacer
            <Footer items=footer_items/>     // footer
        </div>
        <div class:hidden=move || {recursions > 0} style="height: 20px"></div> // small spacer
    }
}

#[component]
pub fn HomePageEntry(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let portfolio_hidden = create_rw_signal(cx, false);
    let music_hidden = create_rw_signal(cx, false);
    let tp_hidden = create_rw_signal(cx, false);
    let webring_hidden = create_rw_signal(cx, false);
    let meta_hidden = create_rw_signal(cx, false);
    let ad_hidden = create_rw_signal(cx, false);
    let john_hidden = create_rw_signal(cx, false);

    let loading_pos = create_rw_signal(cx, (20, 20));
    let portfolio_pos = create_rw_signal(cx, (280, 20));
    let music_pos = create_rw_signal(cx, (20, 262));
    let tp_pos = create_rw_signal(cx, (280, 309));
    let webring_pos = create_rw_signal(cx, (20, 559));
    let meta_pos = create_rw_signal(cx, (485, 192));
    let ad_pos = create_rw_signal(cx, (485, 20));
    let john_pos = create_rw_signal(cx, (20, 701));

    let sigs = vec![
        (loading_pos, loading_hidden),
        (portfolio_pos, portfolio_hidden),
        (music_pos, music_hidden),
        (tp_pos, tp_hidden),
        (webring_pos, webring_hidden),
        (meta_pos, meta_hidden),
        (ad_pos, ad_hidden),
        (john_pos, john_hidden),
    ];

    view! { cx,
        <HomePage recursions=0 sigs=sigs/>
        <GoatCounter path="/"/>
    }
}

const STACK_OVERFLOW_LIMIT: usize = 8;
#[component]
fn MetaWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    recursions: usize,
    sigs: Vec<(RwSignal<(i32, i32)>, RwSignal<bool>)>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let deeper = create_rw_signal(cx, false);
    let content = WindowContent::Page(view! { cx, <div style="width: 100%; height: 100%">
        <div
            class="meta-preview"
            class:hidden=move || deeper()
            on:mousedown=move |_| {deeper.set(true); size.set((720, 844))}
            on:keydown=move |k| if k.key() == "Enter" {deeper.set(true); size.set((720, 844))}
            tabindex=0
            // style="width: 100%; height: 100%"
        >
            <video
                style="cursor: alias; width: 100%; height: 100%; text-align: center"
                muted
                autoplay
                loop="true"
                poster="/assets/o-tawa-insa.svg"
                on:contextmenu=move |e| e.prevent_default()>
                <source src="/assets/o-tawa-insa.webm" type="video/webm"/>
            </video>
        </div>
        <div class="meta-meta scroll" style="height: 844px" class:hidden=move || !deeper()>
            {
                if recursions <= STACK_OVERFLOW_LIMIT {
                    view! { cx, <div> <HomePage recursions=recursions sigs=sigs/> </div> }
                } else {
                    view! { cx, <div> <LoadingWindow pos=WindowPos::Val((20, 55)) size=(300, 100) hidden=hidden variant=LoadingWindowVariant::StackOverflow/> </div> }
                }
            }
        </div>
    </div> });

    view! { cx,
        <Window id="meta-win" title="Meta, man...".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx rainbow=true/>
    }
}
