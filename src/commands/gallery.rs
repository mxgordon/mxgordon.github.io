use dioxus::prelude::*;
use crate::commands::utils::CommandProps;

#[derive(Clone, Props, PartialEq)]
struct GalleryImageProps {
    gallery_entry: GalleryEntry,
}

#[allow(non_snake_case)]
pub fn GalleryImage(props: GalleryImageProps) -> Element {
    let mut ending = props.gallery_entry.src.split("/").collect::<Vec<&str>>();
    let len = ending.len();

    let last_two = ending.split_off(len - 2);


    let src = format!("https://mxgordon.com/cdn-cgi/image/format=webp,height=768/img/{}", last_two.join("/"));

    rsx!(
        div {
            class: "gallery-item",
            a {
                href: format!("/view/{}", props.gallery_entry.name),
                target: "_blank",
                rel: "noopener noreferrer",
                img {
                    src: src,
                    alt: props.gallery_entry.description,
                }
            }
            p {
                {props.gallery_entry.description},
            }
        }
    )
}

#[derive(Clone, PartialEq)]
pub struct GalleryEntry {
    pub name: &'static str,
    pub src: &'static str,
    pub description: &'static str,
}

pub const GALLERY: [GalleryEntry; 9] = [
        GalleryEntry {
        name: "beach_sunset",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920374/img2_003_result_ajtqzg.jpg",
        description: "The sunset behind Nags Head beach, Outer Banks, NC.",
    },
    GalleryEntry {
        name: "cameras",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img068_result_nmmeuf.jpg",
        description: "A medium-format film camera compared to a modern DSLR."
    },
    GalleryEntry {
        name: "worcester",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img064_result_ty9x29.jpg",
        description: "Beautiful Worcester before the snow."
    },
    GalleryEntry {
        name: "rainbow_mountain",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920374/img2_041_result_yfeiql.jpg",
        description: "The famous Rainbow Mountain of Peru."
    },
    GalleryEntry {
        name: "red_valley",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920374/img2_042_result_kcgxsi.jpg",
        description: "The Red Valley from the top of the Rainbow Mountain in Peru."
    },
    GalleryEntry {
        name: "macchu_picchu",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730938313/img2_026_result2_tt1qc7.jpg",
        description: "Macchu Picchu from the top of Huayna Picchu, towering 850 ft above."
    },
    GalleryEntry {
        name: "cusco_street",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img2_047_result_jumilw.jpg",
        description: "A view of Cusco's beautiful red tiled roofs, with its distinctive mountainous skyline."
    },
    GalleryEntry {
        name: "cusco_skyline",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920375/img2_045_result_wh7zm3.jpg",
        description: "Gazing across Cusco to the terraces in the distant foothills. "
    },
    GalleryEntry {
        name: "long_beach",
        src: "https://res.cloudinary.com/dtz40humd/image/upload/v1730920373/img2_001_result_qbbbag.jpg",
        description: "A clear view down Topsail Beach, NC."
    },
];


#[component]
pub fn gallery_html() -> Element {
    rsx!(
        div {
            h2 {
                "My Gallery"
            }
            p {
                "I enjoy shooting film photography in my freetime. I mainly shoot in color, but occasionally I'll shoot in black & white, as its easier to enlarge and such. Most of these photos were shot on my Minolta XG-9."
            }
            p {
                "Keep in mind, these are all unedited, straight off the scanner! Additionally, there's a little bit of x-ray damage from airport security."
            }
            p {
                "Here are some of my favorite shots!"
            }
            div {
                class: "gallery",
                for gallery_entry in GALLERY.iter() {
                    GalleryImage {
                        gallery_entry: gallery_entry.clone(),
                    }
                }
            }
        }
    )

    // view! {
    //     <div>
    //         <h2>"My Gallery"</h2>
    //         <p>"I enjoy shooting film photography in my freetime. I mainly shoot in color, but occasionally I'll shoot in black & white, as its easier to enlarge and such. Most of these photos were shot on my Minolta XG-9."</p>
    //         <p>"Keep in mind, these are all unedited, straight off the scanner! Additionally, there's a little bit of x-ray damage from airport security."</p>
    //         <p>"Here are some of my favorite shots!"</p>
    //         <div class="gallery">
    //             <For each=move || get_gallery().into_iter() key=|entry| entry.name.to_string() children=move |entry| {
    //                 view! {
    //                     <GalleryImage gallery_entry=entry />
    //                 }
    //             } />
    //         </div>
    //     </div>
    // }
}

#[component]
pub fn Gallery(props: CommandProps) -> Element {
    //     if !check_cmd_args_empty(&cmd) {
    //         return view! {
    //             <InvalidOption cmd=cmd on_finished=on_finished />
    //         }
    //     }
    //
    //     view! {
    //         <TypeWriter html_to_type=gallery_html() callback=on_finished />
    //     }
    // }
    rsx!{gallery_html{}}
}
