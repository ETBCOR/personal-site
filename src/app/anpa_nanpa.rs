use crate::app::{Footer, GoatCounter, LoadingWindow, LoadingWindowVariant, WindowPos};
use leptos::*;

const MESSAGES: [&str; 2] = ["a. toki. sina seme", "mi ijo tan anpa nanpa"];

#[component]
pub fn AnpaNanpaPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);

    let footer_items = vec![("\"Inspiration\"", loading_hidden)];
    // let z_idx = Some(create_rw_signal(cx, 1));

    let msg = create_rw_signal(cx, MESSAGES[0]);
    let msg_idx: RwSignal<usize> = create_rw_signal(cx, 0);
    let next_msg = move |_| {
        msg_idx.update(|i| *i = *i + 1);
        let idx = msg_idx();

        use std::cmp::Ordering;
        msg.set(match idx.cmp(&MESSAGES.len()) {
            Ordering::Less => MESSAGES[idx],
            Ordering::Equal => "pini",
            Ordering::Greater => {
                msg_idx.set(0);
                MESSAGES[msg_idx()]
            }
        });
    };

    view! { cx,
        // <LoadingWindow pos=WindowPos::Val((20, 20)) size=(225, 170) hidden=loading_hidden z_idx=z_idx variant=LoadingWindowVariant::HomePageLink/>
        <div style="background-color: black; position: absolute; top: 0px; bottom: 0px; left: 0px; right: 0px; z-index: -3"></div>
        <div id="chat-bubble" on:mousedown=next_msg><div>
            { move || msg() }
        </div></div>
        <Footer items=footer_items nasa=true/>
        <GoatCounter path="/tp/anpa_nanpa"/>
    }
}
