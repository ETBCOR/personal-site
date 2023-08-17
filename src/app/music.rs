use crate::app::{Cyberpunk, LoadingWindow, LoadingWindowVariant, Window};
use leptos::*;

#[component]
pub fn MusicPage(cx: Scope) -> impl IntoView {
    let loading_hidden = create_rw_signal(cx, false);
    let spotify_hidden = create_rw_signal(cx, false);

    view! { cx,
        <LoadingWindow         pos=(20, 20)  size=(255, 255) hidden=loading_hidden variant=LoadingWindowVariant::HomePageLink/>
        <SpotifyPlaylistWindow pos=(310, 20) size=(440, 400) hidden=spotify_hidden/>
        // <Footer items=footer_items/>
        <Cyberpunk/>
    }
}

#[component]
fn SpotifyPlaylistWindow(
    cx: Scope,
    pos: (i32, i32),
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(cx, size);
    let active_tab = create_rw_signal(cx, "Main");
    let content = view! { cx, <div></div> };

    let tabs = Some((
        active_tab,
        vec![
            (
                "Main",
                view! { cx, <div class="tab-outer" style="padding: 10px">
                    <SpotifyPlaylist src="1QPAKgnxEMYOBJFVmRhwM1"/>
                    <SpotifyPlaylist src="0DXYn6zngiQp5AQNOToO3i"/>
                    <SpotifyPlaylist src="3K8Kg0C1GVI14q3KUBqfUd"/>
                    <SpotifyPlaylist src="2q5WCLRthMkrtOOApVGeYW"/>
                    <SpotifyPlaylist src="0S8eDcRFe43fJHlOUAdiBE"/>
                    <SpotifyPlaylist src="6LwfptFt77pViRyjBR2a3u" spaced=false/>
                </div> },
            ),
            (
                "Genres",
                view! { cx, <div class="tab-outer" style="padding: 10px">
                    <SpotifyPlaylist src="4RCXWsAR5yT7P8pfaYKQK9"/>
                    <SpotifyPlaylist src="0ZarPheYW5A3Ut14uvvCYa"/>
                    <SpotifyPlaylist src="1eYJLMDpgoKD0F4LUH2Ezs"/>
                    <SpotifyPlaylist src="36UOLnWsxJlH7Ms5aF3exW"/>
                    <SpotifyPlaylist src="2is9YFXsfFYtAYliO1Xox3"/>
                    <SpotifyPlaylist src="3aLiFKFvxd4PyC3gfSIs4x"/>
                    <SpotifyPlaylist src="2innGMsDBjt4m4BFWczx1P"/>
                    <SpotifyPlaylist src="2LuztnBxzKkEfjvGAJx3vV"/>
                    <SpotifyPlaylist src="0EujpL7Ux9PdGdVxfxXSSl"/>
                    <SpotifyPlaylist src="2SxZEPs788pkeORbGs0NXj"/>
                    <SpotifyPlaylist src="58cvN9oc4TnTuOKbHkgc85"/>
                    <SpotifyPlaylist src="5yAQt15q8sppI3zbr1onsq"/>
                    <SpotifyPlaylist src="7JLhfvA0evymAzY3TB1Opf"/>
                    <SpotifyPlaylist src="1iZl1yGF0ra18Dh0jmNpjt"/>
                    <SpotifyPlaylist src="7EZXrDDMBTjAtf3nWjWk5q"/>
                    <SpotifyPlaylist src="5cnkxBVOu3Ompr3E7QlKa3"/>
                    <SpotifyPlaylist src="37Zs98sWQAJ5SpS60hVvf1"/>
                    <SpotifyPlaylist src="1fqYiy4hDIsByrWdTTYfYA"/>
                    <SpotifyPlaylist src="77SM9ZJXNZtwZNlISBPz4P"/>
                </div> },
            ),
        ],
    ));

    view! { cx,
        <Window id="spotify-win" title="Some of my Spotify playlists".to_string() content=content tabs=tabs pos=pos size=size hidden=hidden z_idx=z_idx scroll=true rainbow=true/>
    }
}

#[component]
fn SpotifyPlaylist(
    cx: Scope,
    src: &'static str,
    #[prop(default = true)] spaced: bool,
) -> impl IntoView {
    view! { cx,
        <iframe
            src=move || format!("https://open.spotify.com/embed/playlist/{src}?utm_source=generator")
            style="width: 400px; height: 152px; border-radius:12px"
            frameBorder="0"
            allowfullscreen=""
            allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
            loading="lazy"
            class:spaced=spaced
        ></iframe><br/>
    }
}
