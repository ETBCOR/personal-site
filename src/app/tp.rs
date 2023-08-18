use crate::app::{GoatCounter, LoadingWindow, LoadingWindowVariant};
use leptos::*;

#[component]
pub fn TokiPonaPage(cx: Scope) -> impl IntoView {
    let loading = create_rw_signal(cx, false);

    view! { cx,
        <LoadingWindow pos=(20, 20) size=(500, 500) hidden=loading variant=LoadingWindowVariant::PageComingSoon/>
        <GoatCounter path="/tp"/>
    }
}
