use dioxus::prelude::*;

#[component]
pub fn WebsiteHeader() -> Element {
    rsx! {
        div { class: "layout-website-header",
            h1 { class: "layout-website-title", "BESTPIZZASINCALIFORNIA.COM" }
            div { class: "layout-website-tagline", "DISCOVER CALIFORNIA'S FINEST PIZZA" }
        }
    }
}

#[component]
pub fn Footer(
    on_advertise_click: EventHandler<MouseEvent>,
    on_about_click: EventHandler<MouseEvent>,
    on_contact_click: EventHandler<MouseEvent>,
    on_terms_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "layout-footer-overlay",
            div { class: "layout-footer-content",
                div { class: "layout-footer-main",
                    div { class: "layout-footer-links",
                        a {
                            href: "#",
                            class: "layout-footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_about_click.call(e);
                            },
                            "ABOUT"
                        }
                        a {
                            href: "#",
                            class: "layout-footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_contact_click.call(e);
                            },
                            "CONTACT"
                        }
                        a {
                            href: "#",
                            class: "layout-footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_advertise_click.call(e);
                            },
                            "ADVERTISE"
                        }
                        a {
                            href: "#",
                            class: "layout-footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_terms_click.call(e);
                            },
                            "TERMS"
                        }
                    }
                }
                div { class: "layout-footer-trademark",
                    div { class: "layout-footer-text", "© 2025 BESTPIZZASINCALIFORNIA.COM" }
                }
            }
        }
    }
}
