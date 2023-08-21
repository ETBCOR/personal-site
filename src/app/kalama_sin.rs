use crate::app::{
    ExternalLink, FileLink, FileWindow, Footer, GoatCounter, JohnWindow, LinkWindow, LoadingWindow,
    LoadingWindowVariant, Window, WindowContent,
};
use leptos::*;

#[component]
pub fn KalamaSinPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let link_win_hidden = create_rw_signal(cx, false);
    let kalama_sin_hidden = create_rw_signal(cx, false);
    let john_hidden = create_rw_signal(cx, false);
    let file_hidden = create_rw_signal(cx, true);

    let footer_items = vec![
        ("\"Inspiration\"", loading_hidden),
        ("lon ilo RedCircle", link_win_hidden),
        ("sitelen toki pi kalama sin", kalama_sin_hidden),
        ("Johnvertisement", john_hidden),
    ];
    let (file_src, set_file_src) = create_signal(cx, None);
    let z_idx = Some(create_rw_signal(cx, 1));

    view! { cx,
        <LoadingWindow   pos=(20, 20)  size=(255, 255) hidden=loading_hidden    z_idx=z_idx variant=LoadingWindowVariant::TP/>
        <LinkWindow      pos=(20, 347) size=(255, 255) hidden=link_win_hidden   z_idx=z_idx id="kalama-sin-link-win" title="lon ilo RedCircle".to_string() bg_img="/assets/kalama-sin.webp" src="https://redcircle.com/shows/kalama-sin" external=true/>
        <KalamaSinWindow pos=(310, 20) size=(440, 582) hidden=kalama_sin_hidden z_idx=z_idx file_win_src=set_file_src/>
        <JohnWindow      pos=(20, 674) size=(730, 90)  hidden=john_hidden       z_idx=z_idx/>
        <FileWindow      pos=(782, 20) size=(600, 744) hidden=file_hidden       z_idx=z_idx src=file_src/>
        <Footer items=footer_items/>
        <GoatCounter path="/tp"/>
    }
}

#[component]
fn KalamaSinWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let fws = file_win_src;

    let content = WindowContent::Page(view! { cx, <div style="padding: 10px" tabindex=0>
        <p>
            "mi pali e sitelen anpa lon "<ExternalLink href="https://redcircle.com/shows/kalama-sin" display="kalama sin"/>". pali mi li pona "
            <ExternalLink href="https://www.youtube.com/playlist?list=PLjOmpMyMxd8Qs2mAXcLk817tQy_AQj09u" display="lon ilo Jutu"/>
            " (o kepeken nena \"CC\"). sina ken lukin e lipu ale tan kalama sin lon ni kin (mi pali e lipu ni pi ale ala):"
        </p>
        <ul>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vTYQAKHmNWVFqnKfr9Z7Zen09agJQUJiQLfMZyTvJ_-0OU9juZ1FNNKgsAvFCRjnPkanc1ud61nI_2X/pub" display="#1: nanpa open tan jan Juli" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vSE37sqz6LfD4F0KgIAh9OQXr5zc9yQJVu8Fxfr3gm89fjMyvk7HCkkPUI6GTb-vf99_p91WURXjWv9/pub" display="#2: pilin ku tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vTPBeABxXHIWTk_i-4csAasUVFdKaAXGUcMi_R0ETo7zs4hW17AoZDA9JmRfJUr1fEiW_aovAGmrpsQ/pub" display="#3: ilo sitelen tan jan Lipamanka" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1zPMpb1-m-ickJakj0933cl3pnApegLPpToGlaJxdxVk/preview" display="#4: musi pi kala ko tan jan Itan, jan nanpa luka tu tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vTXq3OcM8u1_476zrIekPUhZ-biXWhY53_QQZSXp_ADesIdf2Go8PRgIeVvIOOBu5JBG7m9H_jMxDxO/pub" display="#5: tenpo mun monsuta tan jan Teni" file_win_src=fws/></li>
            <li><FileLink src="https://lipukule.org/post/2021/03/14/o-lukin-ala-e-monsi/" display="#6: o lukin ala e monsi tan jan Juli" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vShYto392jps6POf-s8mztAlGCGlBk23L61FS4D-p4yGCBQmGgVI1_r5P1gZEbv5Pvyt7vEiH5mxGYa/pub" display="#7: ma pi lipu Tun tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vQW8Gls6Nds6irxYIiEbuXGf9ea_R_wNU20IdPrCzsK9K0bEEzGbOOyTO08yGSWYjObV-Py8_hGeM7U/pub" display="#8: nasin lipu pona tan jan Lakuse" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1AZFTm30kJyBjsd31UlOrz8ivMlSmULtMlig8x8Pn9qs/preview" display="#9: o toki e ijo pi toki pona ala! tan jTepo tan jLakuse tan jItan" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/17ZDbcq_kKxXUL9jXA9JMJIEpCQrt4uuYfivxt6vqj-c/preview" display="#10: tu lukin lon tan jan Lakuse" file_win_src=fws/></li>
            <li><FileLink src="https://joelthomastr.github.io/tokipona/toki-pi-kon-pona_si" display="#11: pana sona pi nasin toki kepeken sitelen tawa, tan jan Telakoman" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vQfC5lL405CmVLTc9VLoxH5GDGzZMHuGOIHCxIhrFqzmBmtgzBvpuksLXH5W66vgg/pub" display="#12: ike li ken ike ala, tan monsuta pi soweli mun" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vSXG4XS1fH-0GpJJvd79CXfvPXCXi5_Fb-2Grm1cqa0RDoJS54GY6DZvSOFWUpFB3Cn4gUhz0k2qpfL/pub" display="#13: pu Tosi tan jan Juli" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQtasUq60JG-ISBsO1hlEFv5JszjeI57wEyCNEGhnjDq8AeyzKE-tx1qdwWtuMT3FBlyzNcGPvkBntD/pubhtml" display="#14: tenpo Santa li kama, tan jan Itan tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/e/2PACX-1vRGVMKDyMgI18rdv5gCHwu9b7pwDuy8Jth4fdzE30CQg-a-iQX3bp4vkfCPFH3LW9pS4-hh3uI5kf9-/pub" display="#15: musi pi toki pona tan jan Sema" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1AKLB6ddvDsr2SYZ-5W-mf7d48rUrmbrvEpM4cEuGB8s/preview" display="#16: toki Wijosa tan jan Tesa tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/16k38wjGkXUfVYK2Q4fpcyzbf0k_rTQ6oei4IJ9-Xob4/preview" display="#17: nasin pi kama sona tan soweli nata tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/11ZXrWwJ1vedw40sga1T98HfNLaUhA375s5Ffx8rAETg/preview" display="#18: jan mun en nasin waso, tan jan Lakuse tan jan Tepo tan jan Itan" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1nkuIu7QfuDHe_JkBp_cZFOaIBJTSdEz5UGvSl7PcC1c/preview" display="#19: nasin ISO, tan jan Pensa tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1a_zfXHqrSiRb8j5cR4RKfi1ZTx9ksoKyqQmN1OeqBXs/preview" display="#20: ma li supa, tan jan Tepo tan jan Lakuse" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1GftbtvxikDQJKmtmB_CxXItjxTgmg717FeecBS9Qd8M/preview" display="#21: nasin Puta, tan kala Salan tan jan Lakuse tan jan Tepo" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1l36PUgRxwDSWyuKGFBBKyjw20Bi_y11DGxPp74VI-iM/preview" display="#22: poki nasa, tan jan Tepo tan jan Lakuse tan kala Salan" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1eYRLrf4-w2_1VuY9Dc8kPsjPgrTHOr7uv3GHA9bGxEE/preview" display="#23: jan monsuta loje, tan jan Kekan San tan jan Pensa" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1dXmde4rhkUqtGcVrK1d4iC15Yiz2jjmblfuLGt-W0CU/preview" display="#24: ijo sin, tan jan Lakuse" file_win_src=fws/></li>
            <li><FileLink src="https://docs.google.com/document/d/1vKsPFBHWWOTt-eQ0VNkC007ubsvpG_A9xDS7rz7PmpU/preview" display="#25: suno pi toki pona, tan jan Lakuse tan jan Tepo tan palisa jelo Natan tan jan Kepe" file_win_src=fws/></li>
            <li><FileLink src="https://lipumonsuta.neocities.org/mun-monsuta/o-moku-pona" display="#26: o moku pona! tan jan Simiman" file_win_src=fws/></li>
            <li><FileLink src="" display="#27: sona pi toki luka, tan jan Lakuse tan jan Tepo (pini ala!)" file_win_src=fws/></li>
            <li><FileLink src="" display="#28: ma tomo Win, tan jan Ke Tami tan kulupu pi ma Win (pini ala!)" file_win_src=fws/></li>
            <li><FileLink src="" display="#29: pakala li lon telo sijelo loje, tan jan Luke tan jan Kiwisin (pini ala!)" file_win_src=fws/></li>
        </ul>
    </div> });

    view! { cx, <Window id="kalama-sin-win" title="sitelen toki pi kalama sin".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx scroll=true/> }
}

// #[component]
// fn KalamaSinWindow(
//     cx: Scope,
//     pos: (i32, i32),
//     size: (u32, u32),
//     hidden: RwSignal<bool>,
//     #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
// ) -> impl IntoView {
//     let size = create_rw_signal(cx, size);
//     let content = view! { cx, <div style="padding: 10px" tabindex=0>
//         <p>""</p>
//     </div> };

//     view! { cx, <Window id="kalama-sin-win" title="kalama sin transcripts".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/> }
// }
