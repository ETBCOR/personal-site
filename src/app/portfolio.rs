use crate::app::{
    AdWindow, ExternalLink, FileLink, FileWindow, Footer, GoatCounter, LoadingWindow,
    LoadingWindowVariant, Window, WindowContent, WindowPos,
};
use leptos::*;

#[component]
pub fn PortfolioPage() -> impl IntoView {
    let loading_hidden = create_rw_signal(false);
    let about_hidden = create_rw_signal(false);
    let education_hidden = create_rw_signal(false);
    let skills_hidden = create_rw_signal(false);
    let projects_hidden = create_rw_signal(false);
    let file_hidden = create_rw_signal(true);
    let ad_hidden = create_rw_signal(false);

    let footer_items = vec![
        ("\"Inspiration\"", loading_hidden),
        ("About Me", about_hidden),
        ("Education", education_hidden),
        ("Projects", projects_hidden),
        ("Skills", skills_hidden),
    ];
    let (file_src, set_file_src) = create_signal(None);
    let z_idx = create_rw_signal(1);

    view! {
        <LoadingWindow   pos=WindowPos::Val((435, 204)) size=(225, 202) hidden=loading_hidden   z_idx=Some(z_idx) variant=LoadingWindowVariant::HomePageLink/>
        <AboutWindow     pos=WindowPos::Val((20, 20))   size=(640, 112) hidden=about_hidden     z_idx=Some(z_idx)/>
        <EducationWindow pos=WindowPos::Val((20, 204))  size=(380, 572) hidden=education_hidden z_idx=Some(z_idx)/>
        <SkillsWindow    pos=WindowPos::Val((695, 20))  size=(550, 386) hidden=skills_hidden    z_idx=Some(z_idx)/>
        <ProjectsWindow  pos=WindowPos::Val((435, 478)) size=(810, 298) hidden=projects_hidden  z_idx=Some(z_idx) file_win_src=set_file_src/>
        <FileWindow      pos=WindowPos::Val((1278, 20)) size=(500, 756) hidden=file_hidden      z_idx=Some(z_idx) src=file_src/>
        <AdWindow        pos=WindowPos::Val((100, 600)) size=(200, 100) hidden=ad_hidden        z_idx=Some(z_idx)/>
        <div style="height: 65px"></div> // spacer in narrow view
        <Footer items=footer_items/>
        <GoatCounter path="/portfolio"/>
    }
}

#[component]
fn AboutWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let content = WindowContent::Page(view! { <div style="padding: 5px" tabindex=0><p>
        "Hello! I'm Ethan, and was born in April 2001. "
        "I'm passionate about making software, writing music, and learning languages. Links: "
        <ExternalLink href="http://www.discordapp.com/users/207897365141520384" display="discord"/>", "
        <ExternalLink href="mailto:etbcor@gmail.com" display="email"/>", "
        <ExternalLink href="https://www.github.com/ETBCOR" display="GitHub"/>". "
        "Some names I use: "<span class="title">"etbcor"</span><i>" (username)"</i>", "<span class="title">"Friday"</span><i>" (in-person friends)"</i>", "
        <span class="title">"jan Itan"</span><i>" (toki pona community)"</i>". "<b>"Thanks for coming to my site!"</b>
    </p> </div> });

    view! {
        <Window id="about-win" title="About Me".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn EducationWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let content = WindowContent::Page(view! { <div style="padding: 5px" tabindex=0>
        <h4>"Bachelor's Degree in Computer Science"</h4>
        <div class="spaced">
            "I spent 2019-2023 at the "<ExternalLink href="https://www.uidaho.edu/" display="University of Idaho"/>
            " getting my "<ExternalLink href="https://catalog.uidaho.edu/courses/cs/" display="B.S.C.S."/>
            " as well as my "<ExternalLink href="https://catalog.uidaho.edu/courses/span/" display="Spanish minor"/>"."
        </div>

        <div>"CS Classes I took at UI:"</div>
        <div style="border: 1px black solid; max-height: 110px; overflow-y: scroll" tabindex=0>
            <ul  style="font-family: consolas; font-size: 10pt; font-style: bold; line-height: 110%">
                <li>"CS120 | Computer Science I"</li>
                <li>"CS121 | Computer Science II"</li>
                <li>"CS150 | Computer Organization and Architecture"</li>
                <li>"CS210 | Programming Languages"</li>
                <li>"CS240 | Computer Operating Systems"</li>
                <li>"CS270 | System Software"</li>
                <li>"CS360 | Database Systems"</li>
                <li>"CS383 | Software Engineering"</li>
                <li>"CS385 | Theory of Computation"</li>
                <li>"CS395 | Analysis of Algorithms"</li>
                <li>"CS400 | Contemporary Issues in CS"</li>
                <li>"CS415 | Computational Biology: Sequence Alignment"</li>
                <li>"CS445 | Compiler Design"</li>
                <li>"CS452 | Real-Time Operating Systems"</li>
                <li>"CS470 | Artificial Intelligence"</li>
                <li>"CS475 | Machine Learning"</li>
                <li>"CS480 | CS Senior Capstone Design I"</li>
                <li>"CS481 | CS Senior Capstone Design II"</li>
            </ul>
        </div>
        <div class="spaced"></div>

        <h4>"K thru 12"</h4>
        "I was homeschooled from kindergarten through high school, with two exceptions:"
        <ol>
            <li>"I did a year of Montessori in 5th grade"</li>
            <li>"in high school, I was half-time homeschooled and half-time public school (at Idaho Falls High School)"</li>
        </ol>

        <p>"I gained an interest for coding around the age of 10. A friend of mine showed me "
        <ExternalLink href="https://www.codecademy.com/" display="codecademy.com"/>
        " (back when it was still completely free!), which was very influential for me starting out."</p>
    </div> });

    view! {
        <Window id="education-win" title="Education".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx/>
    }
}

#[component]
fn SkillsWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let active_tab = create_rw_signal("Technical");

    let content = WindowContent::Tabs((
        active_tab,
        vec![
            (
                "Technical",
                view! { <div style="padding: 5px"><ul>
                    <li class="spaced">"I'm proficient in multiple "<b>"programming languages"</b>":"<ul>
                        <li><span class="title">"C / C++"</span>" were the primary languages taught at my univirsity, so I'm very comfortable with them."</li>
                        <li><span class="title">"Rust"</span>" is currently my favorite language. I learned about it in 2022 "
                            "and quickly started using it wherever it makes sense, so I'm at an intermediate / advanced level."</li>
                        <li><span class="title">"Python"</span>" isn't usually what I reach to first "
                            "for my projects, but I'm still proficient with it, and have used it for a few."</li>
                        <li><span class="title">""</span>"...and more, including "<span class="title">"JavaScript"</span>", "
                        <span class="title">"Java"</span>", "<span class="title">"SQL"</span>", "<span class="title">"C#"</span>
                        ", and even some "<span class="title">"ML"</span>" and "<span class="title">"Prolog"</span>"."</li>
                    </ul></li>

                    <li class="spaced">
                        <b>"Data structures and algorithms"</b>
                        ": my B.S.C.S. has given me a strong foundation in the fundamentals of Computer Science. "
                        "I am experienced in designing and analyzing various data structures and algorithms."
                    </li>

                    <li class="spaced">
                        "I'm familiar with "<b>"software development concepts"</b>
                        ", including code "<i>"modularity / testing / documentation / version control"</i>" techniques, "
                        <span class="title">"agile"</span>", "<span class="title">"continuous integration and delivery"
                        </span>" and "<span class="title">"the software development life cycle"</span>"."
                    </li>

                    <li class="spaced">
                        "I have a solid understanding of "<b>"networking"</b>" and "<b>"web development"</b>", including how to work with protocols like "
                        <span class="title">"IP"</span>", "<span class="title">"HTTP"</span>", "<span class="title">"TCP"</span>" and "<span class="title">"UDP"</span>
                        ", as well as technologies like "<span class="title">"databases"</span>", "<span class="title">"HTML"</span>", "<span class="title">"CSS"</span>" and "<span class="title">"JavaScript"</span>"."
                    </li>

                    <li class="spaced">
                        "I know how to write code for "<b>"embedded systems"</b>" using the principles of "
                        <span class="title">"real-time operating systems"</span>"."
                    </li>

                    <li>
                        "I also have a solid understanding of "<b>"computer architecture"</b>
                        " and "<b>"operating systems"</b>" concepts in general."
                    </li>
                </ul></div> },
            ),
            (
                "Audio / Visual",
                view! { <div style="padding: 5px"><ul>
                    <li><b>"Audio"</b><ul>
                        <li class="spaced">
                            "I purchased "<ExternalLink href="https://www.ableton.com/en/live/" display="Ableton Live" title_style=true/>
                            " in 2018, and I've been using it to make music in my free time ever since. I've honed my production skills "
                            "quite a bit, including a few years of experimenting with other DAWs before settling on Live. "
                            "I'm planning to release my first album soon!"
                        </li>
                        <li class="spaced">
                            "I volunteered at my church for several years in high school operating the sound booth for the live band, "
                            "so I'm comfortable running a large sound board (analog or digital) and with the basics of audio engineering."
                        </li>
                    </ul></li>

                    <li><b>"Visual"</b><ul>
                        <li class="spaced">
                            "I'm quite experienced with "
                            <ExternalLink href="https://www.adobe.com/products/aftereffects.html" display="After Effects" title_style=true/>
                            ". You can see some of what I've created with it on "
                            <ExternalLink href="https://www.instagram.com/ecridisedits/" display="my IG page"/>"."
                        </li>
                        <li>
                            "I've also volunteered at my church to run slides/lights for sermons, so I'm familiar with "
                            <ExternalLink href="https://renewedvision.com/propresenter/" display="ProPresenter" title_style=true/>
                            " as well as "<br/><span class="title">"DMX lighting systems"</span>"."
                        </li>
                    </ul></li>
                </ul></div> },
            ),
            (
                "Other",
                view! { <div style="padding: 5px"><ul>
                    <li class="spaced">"I speak "<b>"three languages"</b>":"<ul>
                        <li><span class="title">"English"</span>" (native)"</li>
                        <li><span class="title">"Spanish"</span>" (fluent)"</li>
                        <li><ExternalLink href="https://tokipona.org/" display="toki pona" title_style=true/>" (fluent)"</li>
                        <li><span class="title">"Japanese"</span>" (beginner)"</li>
                    </ul></li>

                    <li class="spaced">"I have great "<b>"interpersonal"</b>" and "<b>"conflict-resolution"</b>
                        " skills; I'm able to meaningfully communicate with people, even when we have conflicting views."</li>

                    <li>"I care deeply about my "<b>"work ethic"</b>"; I enjoy locking into my work and getting in the zone."</li>
                </ul></div> },
            ),
        ],
    ));

    view! {
        <Window id="skills-win" title="Skills".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx scroll=true/>
    }
}

#[component]
fn ProjectsWindow(
    pos: WindowPos,
    size: (u32, u32),
    hidden: RwSignal<bool>,
    #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
    file_win_src: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let size = create_rw_signal(size);
    let fws = file_win_src;
    let active_tab = create_rw_signal("From CS Classes");

    let content = WindowContent::Tabs((
        active_tab,
        vec![
            (
                "From CS Classes",
                view! { <div style="padding: 5px"><ul>
                    <li class="spaced">
                        <ExternalLink href="https://catalog.uidaho.edu/courses/cs/#:~:text=CS%20415" display="CS415 | Computational Biology: Sequence Alignment" bold=true/><br/>
                        "Description: \"Design and analyze algorithms that address the computational problems posed by biological sequence data, "
                        "such as DNA or protein sequences.\""<br/>"Projects:"<br/>
                        <FileLink src="https://drive.google.com/file/d/17M8KI3B6rCj2_WLL-YlbxBoK0WzTyexO/preview" display="GA Simulation Runner" file_win_src=fws/>
                        " | "<ExternalLink href="https://github.com/ETBCOR/cs415/tree/main/project01" display="Github Repository"/>
                        <br/>
                        <FileLink src="https://drive.google.com/file/d/1v9XjTqRlf4iGjHskT7yp_KUyVBUU7WgE/preview" display="Parameter Set Estimation" file_win_src=fws/>
                        " | "<ExternalLink href="https://colab.research.google.com/drive/1zQtt-kDBhycueP_qyhzc9VnFeZe0wPmu?usp=sharing" display="Colab Notebook"/>
                        <br/>
                        <FileLink src="https://drive.google.com/file/d/1n-nyTQzjcGy9lpTvs-WYdBcTaDUbZfap/preview" display="Pairwise Alignment Matrix Calculation" file_win_src=fws/>
                        " | "<ExternalLink href="https://colab.research.google.com/drive/1mMGnMO63KR-wHriGNYxBxF5YNwk_r7AP?usp=sharing" display="Colab Notebook"/>
                    </li>

                    <li class="spaced">
                        <ExternalLink href="https://catalog.uidaho.edu/courses/cs/#:~:text=CS%20445" display="CS445 | Compiler Design" bold=true/><br/>
                        "In "<ExternalLink href="http://www2.cs.uidaho.edu/~mdwilder/cs445/" display="this class"/>" I fully implemented a compiler for the "<span style="white-space: nowrap">"\"C minus\""</span>" langauge (grammar specification "
                        <FileLink src="https://drive.google.com/file/d/12o5aSATedS28eJwsHIOHR7uf3DdZY20V/preview" display="here" file_win_src=fws/>
                        "). This is probably the largest solo project I've completed so far. Repository "
                        <ExternalLink href="https://github.com/ETBCOR/cs445" display="here"/>"."
                    </li>

                    <li class="spaced">
                        <ExternalLink href="https://catalog.uidaho.edu/courses/cs/#:~:text=CS%20452" display="CS452 | Real-Time Operating Systems" bold=true/><br/>
                        "In this class I created multiple programs for embedded systems (Feather RP2040 & ESP32), including a basic IOT device with its own webserver. Repository "
                        <ExternalLink href="https://github.com/ETBCOR/cs452/" display="here"/>"."
                    </li>

                    <li class="spaced">
                        <ExternalLink href="https://catalog.uidaho.edu/courses/cs/#:~:text=CS%20470" display="CS470 | Artificial Intelligence" bold=true/><br/>
                        "This class taugh common concepts and techniques involved in artificial intelligence. Projects:"<br/>
                        <FileLink src="https://drive.google.com/file/d/1ICaQOsGKwJ7RfE21xBHvozQkfQGkw43G/preview" display="Pathfinding Algorithms" file_win_src=fws/>
                        " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj1" display="Github Repository"/>
                        <br/>
                        <FileLink src="https://drive.google.com/file/d/1fK-F2X7uwnOk8CrDosopO1pRl6xlBc1u/preview" display="Connect-4 Bot Using Minmax" file_win_src=fws/>
                        " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj2" display="Github Repository"/>
                        <br/>
                        <FileLink src="https://drive.google.com/file/d/1Qr5B0yZ8s3aY3Ywdd4KCYq_7y5bXfCTg/preview" display="Map Coloring Algorithms" file_win_src=fws/>
                        " | "<ExternalLink href="https://github.com/ETBCOR/cs470/tree/master/proj3" display="Github Repository"/>
                        <br/>
                        <FileLink src="https://drive.google.com/file/d/1ysXZTxxRYNOqZDYkrTWZj6VWc2TndJZR/preview" display="Modeling Genealogy in Prolog" file_win_src=fws/>
                    </li>

                    <li class="spaced">
                        <ExternalLink href="https://catalog.uidaho.edu/courses/cs/#:~:text=CS%20475" display="CS475 | Machine Learning" bold=true/><br/>
                        "In this class I completed 8 assignments machine learning topics of varying difficulty. Although the repository is a bit messy, the link is "
                        <ExternalLink href="https://github.com/ETBCOR/cs475" display="here"/>"."
                    </li>

                    <li>
                        <ExternalLink href="https://catalog.uidaho.edu/courses/cs/#:~:text=CS%20480&text=CS%20481" display="CS480 & CS481 | Senior Capstone Design" bold=true/><br/>
                        "For my capstone project I designed calibration software for a laser communication device made by "
                        <ExternalLink href="https://www.hansenphotonics.com/" display="Hansen Photonics Inc"/>
                        " on a team with three other CS majors. The resulting software is simple yet effective. "
                        "The creation process is well documented, but the repository is private; contact me if you're interested in seeing it."
                    </li>
                </ul></div> },
            ),
            (
                "Other Projects",
                view! { <div style="padding: 5px"><ul>
                    <li class="spaced">
                        "I made "<b>"this very portfolio website"</b>" with "
                        <ExternalLink href="https://leptos.dev/" display="leptos"/>" (a full-stack web framework built in "
                        <ExternalLink href="https://www.rust-lang.org/" display="Rust"/>")."
                    </li>

                    <li class="spaced">
                        "I designed "<b>"a font"</b>" for sitelen pona (the writing system of a constructed language). Repository "
                        <ExternalLink href="https://github.com/ETBCOR/nasin-nanpa" display="here"/>"."
                    </li>

                    <li class="spaced">
                        "I've made hundereds of "<b>"songs"</b>" (varying in completeness) "
                        "with Ableton Live in my free time, but I haven't released anything yet."
                    </li>

                    <li class="spaced">
                        "I have "<ExternalLink href="https://www.instagram.com/ecridisedits/" display="an Instagram page"/>
                        " full of cool audio/visaully synced "<b>"edits"</b>" I made with After Effects."
                    </li>

                    <li>"I have worked on quite a few other projects, both personal projects and projects for school (this list is nonexhaustive)."</li>
                </ul></div> },
            ),
        ],
    ));

    view! {
        <Window id="projects-win" title="Projects".to_string() content=content pos=pos size=size hidden=hidden z_idx=z_idx scroll=true/>
    }
}
