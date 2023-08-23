use crate::app::{
    Footer, GoatCounter, LinkWindow, LoadingWindow, LoadingWindowVariant, Webring, WebringWindow,
    Window, WindowContent, WindowPos,
};
use leptos::*;

#[component]
pub fn NasinNanpaPage(cx: Scope) -> impl IntoView {
    let tp_hidden = create_rw_signal(cx, false);
    let link_hidden = create_rw_signal(cx, false);
    let nasin_nanpa_hidden = create_rw_signal(cx, false);
    let webring_hidden = create_rw_signal(cx, false);
    let loading_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("lipu pi toki pona", tp_hidden),
        ("lon ilo Github", link_hidden),
        ("nasin nanpa", nasin_nanpa_hidden),
        ("sike pona", webring_hidden),
        ("o pona", loading_hidden),
    ];
    let z_idx = Some(create_rw_signal(cx, 1));

    view! { cx,
        <LinkWindow       pos=WindowPos::Val((20, 20))   size=(255, 255) hidden=tp_hidden          z_idx=z_idx id="tp-link-win"        title="lipu pi toki pona".to_string() bg_img="/assets/itan.svg"            src="/tp" diag_tp=true/>
        <LinkWindow       pos=WindowPos::Val((310, 20))  size=(300, 255) hidden=link_hidden z_idx=z_idx id="nasin-nanpa-link-win" title="lon ilo GitHub".to_string() bg_img="/assets/nasin-nanpa.png" src="https://github.com/ETBCOR/nasin-nanpa" external=true/>
        <NasinNanpaWindow pos=WindowPos::Val((20, 347))  size=(590, 255) hidden=nasin_nanpa_hidden z_idx=z_idx/>
        <WebringWindow    pos=WindowPos::Val((20, 674))  size=(400, 70)  hidden=webring_hidden     z_idx=z_idx webring=Webring::SikePona/>
        <LoadingWindow    pos=WindowPos::Val((455, 674)) size=(155, 70)  hidden=loading_hidden     z_idx=z_idx variant=LoadingWindowVariant::TP/>
        <Footer items=footer_items/>
        <GoatCounter path="/tp/nasin_nanpa"/>
    }
}

#[component]
fn NasinNanpaWindow(
    cx: Scope,
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let content = WindowContent::Page(view! { cx, <div style="padding: 5px" tabindex=0><p>
        "mi awen pali e lipu ni. ale li pona :)"
    </p></div> });

    view! {cx,
        <Window id="nasin-nanpa-win" title="nasin sitelen tan anpa nanpa".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}
