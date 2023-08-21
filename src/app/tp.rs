use crate::app::{
    Footer, GoatCounter, JohnWindow, LinkWindow, LoadingWindow, LoadingWindowVariant,
};
use leptos::*;

#[component]
pub fn TokiPonaPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let kalama_sin_hidden = create_rw_signal(cx, false);
    let john_hidden = create_rw_signal(cx, false);

    let footer_items = vec![
        ("\"Inspiration\"", loading_hidden),
        ("kalama sin", kalama_sin_hidden),
        ("Johnvertisement", john_hidden),
    ];
    let z_idx = Some(create_rw_signal(cx, 1));

    view! { cx,
        <LoadingWindow   pos=(20, 20)  size=(255, 255) hidden=loading_hidden    z_idx=z_idx variant=LoadingWindowVariant::TP/>
        <LinkWindow      pos=(20, 347) size=(255, 255) hidden=kalama_sin_hidden z_idx=z_idx id="kalama-sin-link-win" title="kalama sin".to_string() bg_img="/assets/kalama-sin.webp" src="/tp/kalama_sin"/>
        <JohnWindow      pos=(20, 674) size=(730, 90)  hidden=john_hidden       z_idx=z_idx/>
        <Footer items=footer_items/>
        <GoatCounter path="/tp"/>
    }
}
