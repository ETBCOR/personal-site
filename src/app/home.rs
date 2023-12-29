use crate::app::music::MusicLinkWindow;
use crate::app::{
    AdWindow, Footer, GoatCounter, JohnWindow, LinkWindow, LoadingWindow, LoadingWindowVariant,
    Webring, WebringWindow, Window, WindowContent, WindowPos,
};
use leptos::*;

#[component]
fn HomePage(recursions: usize, mut sigs: Vec<(WindowPos, RwSignal<bool>)>) -> impl IntoView {
    let (mut loading_pos, loading_hidden) = sigs[0];
    let (mut portfolio_pos, portfolio_hidden) = sigs[1];
    let (mut music_pos, music_hidden) = sigs[2];
    let (mut tp_pos, tp_hidden) = sigs[3];
    let (mut webring_pos, webring_hidden) = sigs[4];
    let (mut meta_pos, meta_hidden) = sigs[5];
    let (mut ad_pos, ad_hidden) = sigs[6];
    let (mut john_pos, john_hidden) = sigs[7];
    let (mut guestbook_pos, guestbook_hidden) = sigs[8];

    let footer_items = vec![
        ("\"Inspiration\"", loading_hidden),
        ("Portfolio", portfolio_hidden),
        ("Music", music_hidden),
        ("toki pona", tp_hidden),
        ("Webring", webring_hidden),
        ("Meta", meta_hidden),
        ("Johnvertisement", john_hidden),
        ("Guestbook", guestbook_hidden),
    ];
    let z_idx = if recursions == 0 {
        Some(create_rw_signal(1))
    } else {
        None
    };

    if recursions == 1 {
        if let WindowPos::Sig(p) = loading_pos {
            loading_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = portfolio_pos {
            portfolio_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = music_pos {
            music_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = tp_pos {
            tp_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = webring_pos {
            webring_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = meta_pos {
            meta_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = ad_pos {
            ad_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = john_pos {
            john_pos = WindowPos::OffsetSignal(p);
        };
        if let WindowPos::Sig(p) = guestbook_pos {
            guestbook_pos = WindowPos::OffsetSignal(p);
        };
        sigs = vec![
            (loading_pos, loading_hidden),
            (portfolio_pos, portfolio_hidden),
            (music_pos, music_hidden),
            (tp_pos, tp_hidden),
            (webring_pos, webring_hidden),
            (meta_pos, meta_hidden),
            (ad_pos, ad_hidden),
            (john_pos, john_hidden),
            (guestbook_pos, guestbook_hidden),
        ];
    }

    view! {
        <LoadingWindow   pos=loading_pos   size=(225, 170) hidden=loading_hidden   z_idx=z_idx variant=LoadingWindowVariant::Default/>
        <LinkWindow      pos=portfolio_pos size=(170, 220) hidden=portfolio_hidden z_idx=z_idx id="portfolio-link-win" title="Portfolio".to_string() bg_img="/assets/file-icon.svg" src="/portfolio"/>
        <MusicLinkWindow pos=music_pos     size=(225, 225) hidden=music_hidden     z_idx=z_idx/> // music link window
        <LinkWindow      pos=tp_pos        size=(170, 178) hidden=tp_hidden        z_idx=z_idx id="tp-link-win"        title="toki pona".to_string() bg_img="/assets/itan.svg" src="/tp" diag_tp=true/>
        <WebringWindow   pos=webring_pos   size=(430, 70)  hidden=webring_hidden   z_idx=z_idx webring=Webring::Bucket/>
        <AdWindow        pos=ad_pos        size=(200, 100) hidden=ad_hidden        z_idx=z_idx/>
        <MetaWindow      pos=meta_pos      size=(200, 437) hidden=meta_hidden      z_idx=z_idx recursions={recursions + 1} sigs=sigs/>
        <JohnWindow      pos=john_pos      size=(665, 82)  hidden=john_hidden      z_idx=z_idx/>
        <div class:hidden=move || {recursions > 0}>
            <div style="height: 65px"></div> // large spacer
            <Footer items=footer_items/>     // footer
        </div>
        <div class:hidden=move || {recursions > 0} style="height: 20px"></div> // small spacer
    }
}

#[component]
pub fn HomePageEntry() -> impl IntoView {
    let loading_hidden = create_rw_signal(false);
    let portfolio_hidden = create_rw_signal(false);
    let music_hidden = create_rw_signal(false);
    let tp_hidden = create_rw_signal(false);
    let webring_hidden = create_rw_signal(false);
    let meta_hidden = create_rw_signal(false);
    let ad_hidden = create_rw_signal(false);
    let john_hidden = create_rw_signal(false);
    let guestbook_hidden = create_rw_signal(false);

    let loading_pos = WindowPos::Sig(create_rw_signal((20, 20)));
    let portfolio_pos = WindowPos::Sig(create_rw_signal((280, 20)));
    let music_pos = WindowPos::Sig(create_rw_signal((20, 262)));
    let tp_pos = WindowPos::Sig(create_rw_signal((280, 309)));
    let webring_pos = WindowPos::Sig(create_rw_signal((20, 559)));
    let meta_pos = WindowPos::Sig(create_rw_signal((485, 192)));
    let ad_pos = WindowPos::Sig(create_rw_signal((485, 20)));
    let john_pos = WindowPos::Sig(create_rw_signal((20, 701)));
    let guestbook_pos = WindowPos::Sig(create_rw_signal((720, 20)));

    let sigs = vec![
        (loading_pos, loading_hidden),
        (portfolio_pos, portfolio_hidden),
        (music_pos, music_hidden),
        (tp_pos, tp_hidden),
        (webring_pos, webring_hidden),
        (meta_pos, meta_hidden),
        (ad_pos, ad_hidden),
        (john_pos, john_hidden),
        (guestbook_pos, guestbook_hidden),
    ];

    let z_idx = Some(create_rw_signal(1));

    view! {
        <HomePage recursions=0 sigs=sigs/>
        <LinkWindow      pos=guestbook_pos size=(200, 100) hidden=guestbook_hidden z_idx=z_idx id="guestbook-link-win" title="Guestbook".to_string() bg_img="assets/itan.svg" src="https://etbcor.123guestbook.com/" external=true/>
        <GoatCounter path="/"/>
    }
}

const STACK_OVERFLOW_LIMIT: usize = 8;
#[component]
fn MetaWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    recursions: usize,
    sigs: Vec<(WindowPos, RwSignal<bool>)>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let deeper = create_rw_signal(false);
    let content = WindowContent::Page(view! { <div style="width: 100%; height: 100%">
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
                    view! { <div> <HomePage recursions=recursions sigs=sigs/> </div> }
                } else {
                    view! { <div> <LoadingWindow pos=WindowPos::Val((20, 55)) size=(300, 100) hidden=hidden variant=LoadingWindowVariant::StackOverflow/> </div> }
                }
            }
        </div>
    </div> });

    view! {
        <Window id="meta-win" title="Meta...".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx min_button=Some((deeper, size)) rainbow=true/>
    }
}
