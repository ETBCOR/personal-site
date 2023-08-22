use crate::app::{
    Footer, GoatCounter, LinkWindow, LoadingWindow, LoadingWindowVariant, Webring, WebringWindow,
    WindowPos,
};
use leptos::*;

#[component]
pub fn TokiPonaPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let kalama_sin_hidden = create_rw_signal(cx, false);
    let webring_hidden = create_rw_signal(cx, false);
    // let john_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("o pona", loading_hidden),
        ("kalama sin", kalama_sin_hidden),
        ("sike ilo", webring_hidden),
        // ("Johnvertisement", john_hidden),
    ];
    let z_idx = Some(create_rw_signal(cx, 1));

    view! { cx,
        <LoadingWindow   pos=WindowPos::Val((20, 20))  size=(255, 255) hidden=loading_hidden    z_idx=z_idx variant=LoadingWindowVariant::TP/>
        <LinkWindow      pos=WindowPos::Val((20, 347)) size=(255, 255) hidden=kalama_sin_hidden z_idx=z_idx id="kalama-sin-link-win" title="kalama sin".to_string() bg_img="/assets/kalama-sin.webp" src="/tp/kalama_sin"/>
        <WebringWindow   pos=WindowPos::Val((20, 674)) size=(430, 70)  hidden=webring_hidden z_idx=z_idx webring=Webring::Kulupu/>
        // <JohnWindow      pos=(20, 674) size=(730, 90)  hidden=john_hidden       z_idx=z_idx/>
        <Footer items=footer_items/>
        <GoatCounter path="/tp"/>
    }
}
