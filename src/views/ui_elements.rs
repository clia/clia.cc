/*
* @Date: 2022-10-14 16:04:10
 * @LastEditTime: 2024-07-05 20:22:52
* @Description:
*/

use dioxus::prelude::*;

pub fn view() -> Element {
    rsx! {
        div {
            h3 { class: "text-3xl font-medium text-gray-700", "UI Elements" }
            // alerts
            div { class: "mt-4",
                h4 { class: "text-gray-600", "Alerts" }
                div { class: "mt-4",
                    div { class: "px-4 py-4 overflow-x-auto bg-white rounded-md whitespace-nowrap",
                        div { class: "inline-flex w-full max-w-sm ml-3 overflow-hidden bg-white rounded-lg shadow-md",
                            div { class: "flex items-center justify-center w-12 bg-green-500",
                                icons::icon_1 {}
                            }
                            div { class: "px-4 py-2 -mx-3",
                                div { class: "mx-3",
                                    span { class: "font-semibold text-green-500", "Success" }
                                    p { class: "text-sm text-gray-600",
                                        "Your account was registered!"
                                    }
                                }
                            }
                        }
                        div { class: "inline-flex w-full max-w-sm ml-3 overflow-hidden bg-white rounded-lg shadow-md",
                            div { class: "flex items-center justify-center w-12 bg-blue-500",
                                icons::icon_2 {}
                            }
                            div { class: "px-4 py-2 -mx-3",
                                div { class: "mx-3",
                                    span { class: "font-semibold text-blue-500", "Info" }
                                    p { class: "text-sm text-gray-600",
                                        "Channel archived by the owner."
                                    }
                                }
                            }
                        }
                        div { class: "inline-flex w-full max-w-sm ml-3 overflow-hidden bg-white rounded-lg shadow-md",
                            div { class: "flex items-center justify-center w-12 bg-yellow-500",
                                icons::icon_3 {}
                            }
                            div { class: "px-4 py-2 -mx-3",
                                div { class: "mx-3",
                                    span { class: "font-semibold text-yellow-500", "Warning" }
                                    p { class: "text-sm text-gray-600", "Image size is too large." }
                                }
                            }
                        }
                        div { class: "inline-flex w-full max-w-sm ml-3 overflow-hidden bg-white rounded-lg shadow-md",
                            div { class: "flex items-center justify-center w-12 bg-red-500",
                                icons::icon_4 {}
                            }
                            div { class: "px-4 py-2 -mx-3",
                                div { class: "mx-3",
                                    span { class: "font-semibold text-red-500", "Error" }
                                    p { class: "text-sm text-gray-600", "Your email is already used!" }
                                }
                            }
                        }
                    }
                }
            }
            // Inputs
            div { class: "mt-8",
                h4 { class: "text-gray-600", "Inputs" }
                div { class: "mt-4",
                    div { class: "flex items-center px-4 py-4 space-x-4 overflow-x-auto bg-white rounded-md",
                        label {
                            input {
                                class: "w-5 h-5 text-indigo-600 focus:ring-indigo-500",
                                name: "radio",
                                r#type: "radio"
                            }
                            span { class: "ml-2 text-gray-700", "Radio" }
                        }
                        label {
                            input {
                                class: "w-5 h-5 text-indigo-600 rounded-md focus:ring-indigo-500",
                                name: "radio",
                                r#type: "checkbox"
                            }
                            span { class: "ml-2 text-gray-700", "Checkbox" }
                        }
                        label { class: "block",
                            input {
                                class: "block w-full mt-1 c-input",
                                placeholder: "Email",
                                r#type: "email"
                            }
                        }
                        div { class: "relative mx-4 lg:mx-0",
                            span { class: "absolute inset-y-0 left-0 flex items-center pl-3",
                                icons::icon_5 {}
                            }
                            input {
                                class: "w-32 pl-10 pr-4 text-indigo-600 sm:w-64 c-input",
                                placeholder: "Search",
                                r#type: "text"
                            }
                        }
                    }
                }
            }
            // Buttons
            div { class: "mt-8",
                h4 { class: "text-gray-600", "Buttons" }
                div { class: "mt-4",
                    div { class: "flex px-4 py-4 space-x-4 overflow-x-auto bg-white rounded-md",
                        button { class: "px-4 py-2 font-medium tracking-wide text-white capitalize transition-colors duration-200 transform bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none focus:bg-indigo-500",
                            "Primary"
                        }
                        button { class: "flex items-center px-2 py-2 font-medium tracking-wide text-white capitalize transition-colors duration-200 transform bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none focus:bg-indigo-500",
                            icons::icon_6 {}
                            span { class: "mx-1", "Refresh" }
                        }
                        div { class: "flex items-center",
                            button { class: "px-4 py-2 font-medium tracking-wide text-white capitalize transition-colors duration-200 transform bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none focus:bg-indigo-500",
                                "Download"
                            }
                            span { class: "border border-transparent" }
                            div { class: "relative",
                                button { class: "relative z-10 block p-2 transition-colors duration-200 transform bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none focus:bg-indigo-500",
                                    icons::icon_7 {}
                                }
                                div { class: "absolute right-0 z-20 w-48 mt-2 overflow-hidden bg-white rounded-md shadow-xl dark:bg-gray-700" }
                            }
                        }
                    }
                }
            }
            // Paginations
            div { class: "mt-8",
                h4 { class: "text-gray-600", "Paginations" }
                div { class: "mt-4",
                    div { class: "flex px-4 py-4 overflow-x-auto bg-white rounded-md",
                        div { class: "flex mr-4 rounded",
                            a {
                                class: "px-3 py-2 ml-0 leading-tight text-indigo-700 bg-white border border-r-0 border-gray-200 rounded-l hover:bg-indigo-500 hover:text-white",
                                href: "#",
                                span { "Previous" }
                            }
                            a {
                                class: "px-3 py-2 leading-tight text-indigo-700 bg-white border border-r-0 border-gray-200 hover:bg-indigo-500 hover:text-white",
                                href: "#",
                                span { "1" }
                            }
                            a {
                                class: "px-3 py-2 leading-tight text-indigo-700 bg-white border border-r-0 border-gray-200 hover:bg-indigo-500 hover:text-white",
                                href: "#",
                                span { "2" }
                            }
                            a {
                                class: "px-3 py-2 leading-tight text-indigo-700 bg-white border border-r-0 border-gray-200 hover:bg-indigo-500 hover:text-white",
                                href: "#",
                                span { "3" }
                            }
                            a {
                                class: "px-3 py-2 leading-tight text-indigo-700 bg-white border border-gray-200 rounded-r hover:bg-indigo-500 hover:text-white",
                                href: "#",
                                span { "Next" }
                            }
                        }
                    }
                }
            }
        }
    }
}

mod icons {
    use dioxus::prelude::*;
    use dioxus_html_macro::html;

    pub fn icon_1() -> Element {
        html! {
                <svg
                      class="w-6 h-6 text-white fill-current"
                      view_box="0 0 40 40"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        d="M20 3.33331C10.8 3.33331 3.33337 10.8 3.33337 20C3.33337 29.2 10.8 36.6666 20 36.6666C29.2 36.6666 36.6667 29.2 36.6667 20C36.6667 10.8 29.2 3.33331 20 3.33331ZM16.6667 28.3333L8.33337 20L10.6834 17.65L16.6667 23.6166L29.3167 10.9666L31.6667 13.3333L16.6667 28.3333Z"
                      />
                    </svg>
        }
    }

    pub fn icon_2() -> Element {
        html! {
            <svg
                  class="w-6 h-6 text-white fill-current"
                  view_box="0 0 40 40"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M20 3.33331C10.8 3.33331 3.33337 10.8 3.33337 20C3.33337 29.2 10.8 36.6666 20 36.6666C29.2 36.6666 36.6667 29.2 36.6667 20C36.6667 10.8 29.2 3.33331 20 3.33331ZM21.6667 28.3333H18.3334V25H21.6667V28.3333ZM21.6667 21.6666H18.3334V11.6666H21.6667V21.6666Z"
                  />
                </svg>
        }
    }

    pub fn icon_3() -> Element {
        html! {
            <svg
                  class="w-6 h-6 text-white fill-current"
                  view_box="0 0 40 40"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M20 3.33331C10.8 3.33331 3.33337 10.8 3.33337 20C3.33337 29.2 10.8 36.6666 20 36.6666C29.2 36.6666 36.6667 29.2 36.6667 20C36.6667 10.8 29.2 3.33331 20 3.33331ZM21.6667 28.3333H18.3334V25H21.6667V28.3333ZM21.6667 21.6666H18.3334V11.6666H21.6667V21.6666Z"
                  />
                </svg>
        }
    }

    pub fn icon_4() -> Element {
        html! {
            <svg
                  class="w-6 h-6 text-white fill-current"
                  view_box="0 0 40 40"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M20 3.36667C10.8167 3.36667 3.3667 10.8167 3.3667 20C3.3667 29.1833 10.8167 36.6333 20 36.6333C29.1834 36.6333 36.6334 29.1833 36.6334 20C36.6334 10.8167 29.1834 3.36667 20 3.36667ZM19.1334 33.3333V22.9H13.3334L21.6667 6.66667V17.1H27.25L19.1334 33.3333Z"
                  />
                </svg>
        }
    }

    pub fn icon_5() -> Element {
        html! {
            <svg class="w-5 h-5 text-gray-500" view_box="0 0 24 24" fill="none">
                  <path
                    d="M21 21L15 15M17 10C17 13.866 13.866 17 10 17C6.13401 17 3 13.866 3 10C3 6.13401 6.13401 3 10 3C13.866 3 17 6.13401 17 10Z"
                    stroke="currentColor"
                    stroke_width="2"
                    stroke_linecap="round"
                    stroke_linejoin="round"
                  />
                </svg>
        }
    }

    pub fn icon_6() -> Element {
        html! {
            <svg class="w-5 h-5 mx-1" xmlns="http://www.w3.org/2000/svg"
                view_box="0 0 20 20" fill="currentColor">
                <path fill_rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z"
                clip_rule="evenodd" />
            </svg>
        }
    }

    pub fn icon_7() -> Element {
        html! {
            <svg class="w-6 h-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" view_box="0 0 24 24" stroke="currentColor">
                <path stroke_linecap="round" stroke_linejoin="round" stroke_width="2"
                d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z" />
            </svg>
        }
    }
}
