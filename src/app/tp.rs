use crate::app::{
    Footer, GoatCounter, LinkWindow, LoadingWindow, LoadingWindowVariant, Webring, WebringWindow,
    Window, WindowContent, WindowPos,
};
use leptos::*;

#[component]
pub fn TokiPonaPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let nasin_nanpa_hidden = create_rw_signal(cx, false);
    let kalama_sin_hidden = create_rw_signal(cx, false);
    let lipu_mi_hidden = create_rw_signal(cx, false);
    let webring_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("o pona", loading_hidden),
        ("nasin nanpa", nasin_nanpa_hidden),
        ("kalama sin", kalama_sin_hidden),
        ("lipu mi", lipu_mi_hidden),
        ("sike pona", webring_hidden),
    ];
    let z_idx = Some(create_rw_signal(cx, 1));

    view! { cx,
        <LoadingWindow    pos=WindowPos::Val((20, 20))   size=(255, 255) hidden=loading_hidden     z_idx=z_idx variant=LoadingWindowVariant::TP/>
        <LinkWindow       pos=WindowPos::Val((310, 20))  size=(300, 255) hidden=nasin_nanpa_hidden z_idx=z_idx id="nasin-nanpa-link-win" title="nasin nanpa".to_string() bg_img="/assets/nasin-nanpa.png" src="/tp/nasin_nanpa"/>
        <LinkWindow       pos=WindowPos::Val((20, 347))  size=(255, 255) hidden=kalama_sin_hidden  z_idx=z_idx id="kalama-sin-link-win" title="kalama sin".to_string() bg_img="/assets/kalama-sin.webp" src="/tp/kalama_sin"/>
        <LipuMiWindow     pos=WindowPos::Val((310, 347)) size=(300, 255) hidden=lipu_mi_hidden     z_idx=z_idx/>
        <WebringWindow    pos=WindowPos::Val((20, 674))  size=(590, 70)  hidden=webring_hidden     z_idx=z_idx webring=Webring::SikePona/>
        <Footer items=footer_items/>
        <GoatCounter path="/tp"/>
    }
}

#[component]
fn LipuMiWindow(
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
        <Window id="lipu-mi-win" title="lipu mi".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}
