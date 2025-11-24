use dioxus::prelude::*;

#[component]
pub fn AdvertiseModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal modal-show" } else { "modal" };

    rsx! {
        div {
            class: "{modal_class}",
            div { class: "modal-content",
                button {
                    class: "modal-close",
                    onclick: move |e| on_close.call(e),
                    "×"
                }
                h2 { class: "modal-title", "PARTNER WITH US" }
                div { class: "modal-divider" }
                div { class: "modal-text",
                    "SEE YOUR RESTAURANT HERE?"
                    br {}
                    "LIKE WHAT YOU SEE?"
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "AFFILIATE PROGRAM" }
                    p { class: "modal-description",
                        "Join California's premier pizza discovery platform. We drive hungry customers directly to your door."
                    }
                    ul { class: "modal-features",
                        li { "FEATURED PLACEMENT" }
                        li { "PRIORITY LISTING" }
                        li { "PHOTO GALLERIES" }
                        li { "DIRECT ORDERING LINKS" }
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "CONTACT" }
                    div { class: "modal-contact",
                        div { "EMAIL: PARTNERS@BESTPIZZASINCALIFORNIA.COM" }
                    }
                }
                button { class: "modal-cta", "GET STARTED" }
            }
        }
    }
}

#[component]
pub fn AboutModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal modal-show" } else { "modal" };

    rsx! {
        div {
            class: "{modal_class}",
            div { class: "modal-content",
                button {
                    class: "modal-close",
                    onclick: move |e| on_close.call(e),
                    "×"
                }
                h2 { class: "modal-title", "ABOUT US" }
                div { class: "modal-divider" }
                div { class: "modal-text",
                    "CALIFORNIA'S DEFINITIVE"
                    br {}
                    "PIZZA GUIDE"
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "OUR MISSION" }
                    p { class: "modal-description",
                        "We connect pizza lovers with the finest pizzerias across California. From traditional Neapolitan to innovative artisan creations, we curate the best slice experiences in every neighborhood."
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "WHAT WE DO" }
                    ul { class: "modal-features",
                        li { "CURATE TOP PIZZERIAS" }
                        li { "VERIFY QUALITY STANDARDS" }
                        li { "SHOWCASE AUTHENTIC PHOTOS" }
                        li { "PROVIDE HONEST REVIEWS" }
                        li { "UPDATE REAL-TIME INFO" }
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "ESTABLISHED" }
                    p { class: "modal-description",
                        "Founded in 2025, we've quickly become California's most trusted pizza discovery platform, serving thousands of hungry customers daily."
                    }
                }
            }
        }
    }
}

#[component]
pub fn ContactModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal modal-show" } else { "modal" };

    rsx! {
        div {
            class: "{modal_class}",
            div { class: "modal-content",
                button {
                    class: "modal-close",
                    onclick: move |e| on_close.call(e),
                    "×"
                }
                h2 { class: "modal-title", "CONTACT US" }
                div { class: "modal-divider" }
                div { class: "modal-text",
                    "WE'RE HERE TO HELP"
                    br {}
                    "GET IN TOUCH"
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "GENERAL INQUIRIES" }
                    div { class: "modal-contact",
                        div { "EMAIL: INFO@BESTPIZZASINCALIFORNIA.COM" }
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "RESTAURANT PARTNERSHIPS" }
                    div { class: "modal-contact",
                        div { "EMAIL: PARTNERS@BESTPIZZASINCALIFORNIA.COM" }
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "MEDIA & PRESS" }
                    div { class: "modal-contact",
                        div { "EMAIL: PRESS@BESTPIZZASINCALIFORNIA.COM" }
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "OFFICE" }
                    div { class: "modal-contact",
                        div { "123 PIZZA LANE" }
                        div { "LOS ANGELES, CA 90028" }
                        div { "MONDAY - FRIDAY: 9AM - 6PM PST" }
                    }
                }
                button { class: "modal-cta", "SEND MESSAGE" }
            }
        }
    }
}

#[component]
pub fn TermsModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal modal-show" } else { "modal" };

    rsx! {
        div {
            class: "{modal_class}",
            div { class: "modal-content",
                button {
                    class: "modal-close",
                    onclick: move |e| on_close.call(e),
                    "×"
                }
                h2 { class: "modal-title", "TERMS OF SERVICE" }
                div { class: "modal-divider" }
                div { class: "modal-text", "LAST UPDATED: JANUARY 2025" }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "ACCEPTANCE OF TERMS" }
                    p { class: "modal-description",
                        "By accessing and using BESTPIZZASINCALIFORNIA.COM, you accept and agree to be bound by the terms and provision of this agreement."
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "USE LICENSE" }
                    p { class: "modal-description",
                        "Permission is granted to temporarily access the materials on our website for personal, non-commercial transitory viewing only. This license shall automatically terminate if you violate any of these restrictions."
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "DISCLAIMER" }
                    p { class: "modal-description",
                        "The materials on BESTPIZZASINCALIFORNIA.COM are provided on an 'as is' basis. We make no warranties, expressed or implied, and hereby disclaim and negate all other warranties including, without limitation, implied warranties or conditions of merchantability, fitness for a particular purpose, or non-infringement of intellectual property."
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "LIMITATIONS" }
                    p { class: "modal-description",
                        "In no event shall BESTPIZZASINCALIFORNIA.COM or its suppliers be liable for any damages arising out of the use or inability to use the materials on our website, even if we or our authorized representative has been notified orally or in writing of the possibility of such damage."
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "PRIVACY" }
                    p { class: "modal-description",
                        "Your privacy is important to us. We do not sell, trade, or rent users' personal identification information to others. We may share generic aggregated demographic information not linked to any personal identification information with our business partners and trusted affiliates for the purposes outlined above."
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "CONTACT" }
                    div { class: "modal-contact",
                        div { "EMAIL: LEGAL@BESTPIZZASINCALIFORNIA.COM" }
                    }
                }
            }
        }
    }
}
