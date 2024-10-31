use dioxus::prelude::*;

pub fn view() -> Element {

    rsx! {
        footer { class: "flex items-center justify-end px-6 py-4 bg-white border-b-4 border-indigo-600",
            div { class: "flex text-end",
                a { class: "flex text-end", href: "https://github.com/clia/clia.cc",
                    "Source code for this site"
                }
            }
        }
    }
}
