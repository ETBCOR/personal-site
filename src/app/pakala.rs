use crate::app::{Footer, GoatCounter};
use leptos::*;

const MESSAGES: [&str; 12] = [
    "a. toki. sina seme",
    "mi 󱤌󱥧󱤅󱤽︀anu jan[ijotananpananpa]",
    "ken la sina toki: te mi lon seme to",
    "pona. sina lukin ala lukin e nanpa-suli ni<",
    "ona li lon sewi-mi. mi-tu li lon(anpaona)",
    "mi tan ni a. nimi-mi kin",
    "a. sina pakala e ilo anu seme",
    "ale li pona. mi ken pona e ilo",
    "mi pali. o awen-lili",
    "pona. mi sin e ilo",
    "o tawa pona",
    "ale li pona",
];

#[component]
pub fn PakalaPage(cx: Scope) -> impl IntoView {
    let chat_hidden = create_rw_signal(cx, false);

    let footer_items = vec![];

    let msg = create_rw_signal(cx, MESSAGES[0]);
    let msg_idx: RwSignal<usize> = create_rw_signal(cx, 0);
    let next_msg = move || {
        msg_idx.update(|i| *i = *i + 1);
        let idx = msg_idx();

        msg.set(if idx < MESSAGES.len() {
            MESSAGES[idx]
        } else {
            let _ = leptos_router::use_navigate(cx)("/", Default::default());
            ""
        });
    };

    view! { cx,
        <div style="background-color: black; position: absolute; top: 0px; bottom: 0px; left: 0px; right: 0px; z-index: -3"></div>
        <div id="nanpa-suli">
            <video
                muted
                autoplay
                loop="true"
                poster="/assets/nanpa-suli.png"
                on:contextmenu=move |e| e.prevent_default()>
                <source src="/assets/nanpa-suli.webm" type="video/webm"/>
            </video>
        </div>
        <div id="chat-bubble" class:hidden=move || chat_hidden() on:mousedown=move |_| next_msg() on:keydown=move |k| if k.key() == "Enter" { next_msg() } tabindex=0><div>
            { move || msg() }
        </div></div>
        <Footer items=footer_items nasa=true/>
        <GoatCounter path="/pakala"/>
    }
}
