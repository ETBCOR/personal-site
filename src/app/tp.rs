use crate::app::{
    ExternalLink, FileLink, FileWindow, Footer, GoatCounter, LinkWindow, LoadingWindow,
    LoadingWindowVariant, Webring, WebringWindow, Window, WindowContent, WindowPos,
};
use leptos::*;

#[component]
pub fn TokiPonaPage() -> impl IntoView {
    let loading_hidden = create_rw_signal(false);
    let nasin_nanpa_hidden = create_rw_signal(false);
    let kalama_sin_hidden = create_rw_signal(false);
    let ijo_ante_hidden = create_rw_signal(false);
    let file_hidden = create_rw_signal(true);
    let webring_hidden = create_rw_signal(false);

    let footer_items = vec![
        ("o pona", loading_hidden),
        ("nasin nanpa", nasin_nanpa_hidden),
        ("kalama sin", kalama_sin_hidden),
        ("ijo ante", ijo_ante_hidden),
        ("sike pona", webring_hidden),
    ];
    let (file_src, set_file_src) = create_signal(None);
    let z_idx = Some(create_rw_signal(1));

    view! {
        <LoadingWindow pos=WindowPos::Val((20, 20))   size=(255, 255) hidden=loading_hidden     z_idx=z_idx variant=LoadingWindowVariant::TP/>
        <LinkWindow    pos=WindowPos::Val((310, 20))  size=(300, 255) hidden=nasin_nanpa_hidden z_idx=z_idx id="nasin-nanpa-link-win" title="nasin nanpa".to_string() bg_img="/assets/nasin-nanpa.png" src="/tp/nasin_nanpa"/>
        <LinkWindow    pos=WindowPos::Val((20, 347))  size=(255, 255) hidden=kalama_sin_hidden  z_idx=z_idx id="kalama-sin-link-win" title="kalama sin".to_string() bg_img="/assets/kalama-sin.webp" src="/tp/kalama_sin"/>
        <IjoAnteWindow pos=WindowPos::Val((310, 347)) size=(300, 255) hidden=ijo_ante_hidden    z_idx=z_idx file_win_src=set_file_src/>
        <FileWindow    pos=WindowPos::Val((645, 20))  size=(700, 744) hidden=file_hidden       z_idx=z_idx src=file_src/>
        <WebringWindow pos=WindowPos::Val((20, 674))  size=(590, 70)  hidden=webring_hidden     z_idx=z_idx webring=Webring::SikePona/>
        <Footer items=footer_items/>
        <GoatCounter path="/tp"/>
    }
}

#[component]
fn IjoAnteWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let fws = file_win_src;
    let active_tab = create_rw_signal("mi");

    let content = WindowContent::Tabs((
        active_tab,
        vec![
            (
                "mi",
                view! { <div style="padding: 5px"><p>
                    "ijo mi pi toki pona li lon ni:"
                    <ul>
                        <li>"nasin sitelen tan anpa nanpa (lon sewi ↑)"</li>
                        <li>"sitelen toki pi kalama sin (lon poka ←)"</li>
                        <li><FileLink src="https://docs.google.com/document/d/11rCtIptD3cQybXXYDv2S8ozZINYdIPx96BDM1yE2OU8/preview" display="\"kala pi alasa kala\" — toki musi" file_win_src=fws/></li>
                        <li><FileLink src="https://docs.google.com/document/d/1FgoMDYm_R1mOZxF5fEFpc1-FzUG1lKJLWcIcyRMnzTU/preview" display="\"mu namako\" — toki musi" file_win_src=fws/></li>
                        <li><FileLink src="https://docs.google.com/document/d/163dd0cseeR3qIpuRXlWZQClEbuLEAj3W7KRoXIo0b74/preview" display="ijo (nasa) tan anpa nanpa — toki nasa mute" file_win_src=fws/></li>
                    </ul>
                </p></div> },
            ),
            (
                "jan",
                view! { <div style="padding: 5px"><p>
                    "jan pona pi toki pona li lon ni:"
                    <ul>
                        <li><ExternalLink href="https://tbodt.com/" display="jan Tepo"/></li>
                        <li><ExternalLink href="https://raacz.neocities.org/" display="jan Lakuse"/></li>
                        <li><ExternalLink href="https://www.youtube.com/@jkepe" display="jan Kepe"/></li>
                        <li><ExternalLink href="https://tokipona.org/" display="jan Sonja"/></li>
                        <li><ExternalLink href="https://lipamanka.gay/" display="lipamanka"/></li>
                        <li><ExternalLink href="https://joelthomastr.github.io/tokipona/README_si" display="jan Telakoman"/></li>
                        <li><ExternalLink href="https://kala.li.pona.la/" display="kala pona Tonyu"/></li>
                        <li><ExternalLink href="https://janketami.wordpress.com/" display="jan Ke Tami"/></li>
                        <li><ExternalLink href="https://mun.la/" display="jan Kekan San"/></li>
                    </ul>
                </p></div> },
            ),
            (
                "ante",
                view! { <div style="padding: 5px"><p>
                    "ijo pona mute pi toki pona li lon ni:"
                    <ul>
                        <li><ExternalLink href="https://sona.pona.la/wiki/Main_Page" display="sona pona"/></li>
                        <li><ExternalLink href="https://suno.pona.la/" display="suno pi toki pona"/></li>
                        <li><ExternalLink href="https://kulupu.pona.la/" display="kulupu lipu pona"/></li>
                        <li><ExternalLink href="http://utala.pona.la/" display="utala musi pi ma pona"/></li>
                        <li><ExternalLink href="https://linku.la/" display="lipu Linku (lipu nimi pona)"/></li>
                        <li><ExternalLink href="https://jamesmoulang.itch.io/nasin-sona-musi" display="nasin sona musi"/></li>
                        <li><ExternalLink href="https://davidar.github.io/tp/" display="tomo pi sitelen tawa"/></li>
                        <li><ExternalLink href="https://seka.pona.la/login" display="ma Seka (sama ilo Jutu - taso toki pona taso li lon)"/></li>
                        <li><ExternalLink href="https://www.youtube.com/playlist?list=PLwYL9_SRAk8EXSZPSTm9lm2kD_Z1RzUgm" display="o pilin e toki pona #opetp"/></li>
                        <li><ExternalLink href="https://sona.pona.la/wiki/Where_is_Toki_Pona_used%3F" display="\"seme li kepeken toki pona?\""/></li>
                    </ul>
                </p></div> },
            ),
        ],
    ));

    view! {
        <Window id="ijo-ante-win" title="ijo ante".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx scroll=true/>
    }
}
