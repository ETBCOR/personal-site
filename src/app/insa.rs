use crate::app::{Footer, GoatCounter, LoadingWindow, LoadingWindowVariant};
use leptos::*;

#[component]
pub fn InsaPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);

    let footer_items = vec![("\"Inspiration\"", loading_hidden)];
    let z_idx = Some(create_rw_signal(cx, 1));

    view! { cx,
        <LoadingWindow pos=(20, 20) size=(225, 170) hidden=loading_hidden z_idx=z_idx variant=LoadingWindowVariant::HomePageLink/>
        <div style="width: 100%; height: 100%; background-color: black"></div>
        <Footer items=footer_items/>
        <GoatCounter path="/insa"/>
    }
}
