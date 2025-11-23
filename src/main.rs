use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut show_advertise_modal = use_signal(|| false);
    let mut show_about_modal = use_signal(|| false);
    let mut show_contact_modal = use_signal(|| false);
    let mut show_terms_modal = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Tektur:wght@400;500;700;900&display=swap"
        }
        document::Title { "BestPizzasInCalifornia.com" }

        // Google AdSense Script
        document::Script {
            src: "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-5369604889706863",
            crossorigin: "anonymous",
            async: true
        }

        LocationBar {}
        WebsiteHeader {}

        div { class: "restaurants-container",

            // AdSense Banner Top
            AdSense {
                banner_type: "banner",
                ad_slot: "1111111111"  // Replace with your ad slot ID
            }

            // Restaurant 1: Pitfire Pizza
            RestaurantSection {
                name: "PITFIRE PIZZA",
                restaurant_type: "ARTISAN PIZZA",
                address: "9 LOCATIONS ACROSS CALIFORNIA",
                slides: vec![
                    "THE NEW MARGHERITA".to_string(),
                    "ZOE'S PEPPERONI".to_string(),
                    "HONEY BEAR".to_string(),
                    "EPIC BBQ CHICKEN".to_string(),
                ],
                menu_items: vec![
                    ("The New Margherita".to_string(), "$17".to_string()),
                    ("Vegan Margherita".to_string(), "$17".to_string()),
                    ("Zoe's Pepperoni".to_string(), "$18".to_string()),
                    ("Field Mushroom".to_string(), "$18.50".to_string()),
                    ("Say Cheese".to_string(), "$16".to_string()),
                    ("Honey Bear".to_string(), "$18".to_string()),
                    ("Pineapple Express".to_string(), "$18.50".to_string()),
                    ("Epic BBQ Chicken".to_string(), "$19".to_string()),
                    ("Sausage Party".to_string(), "$21".to_string()),
                    ("The Burrata".to_string(), "$18".to_string()),
                    ("Smashing Pumpkin".to_string(), "$18".to_string()),
                ]
            }

            // AdSense In-Feed
            AdSense {
                banner_type: "infeed",
                ad_slot: "2222222222"  // Replace with your in-feed ad slot ID
            }

            // Restaurant 2: Ciao Pasta
            RestaurantSection {
                name: "CIAO PASTA",
                restaurant_type: "THIN CRUST ITALIAN PIZZA",
                address: "SAN JUAN CAPISTRANO, CA 92675",
                slides: vec![
                    "MARGHERITA".to_string(),
                    "CRUDO".to_string(),
                    "DIAVOLA".to_string(),
                    "QUATTRO STAGIONI".to_string(),
                ],
                menu_items: vec![
                    ("Margherita".to_string(), "$22".to_string()),
                    ("Crudo".to_string(), "$23".to_string()),
                    ("Diavola".to_string(), "$23".to_string()),
                    ("Quattro Stagioni".to_string(), "$23".to_string()),
                ]
            }

            // AdSense Banner Middle
            AdSense {
                banner_type: "banner",
                ad_slot: "3333333333"  // Replace with your ad slot ID
            }

            // Restaurant 3: Mr Motto
            RestaurantSection {
                name: "MR MOTTO",
                restaurant_type: "WHITE & RED PIES",
                address: "SAN DIEGO, CA",
                slides: vec![
                    "PIECE OF ART".to_string(),
                    "MAMMA MOTO".to_string(),
                    "THE HULK".to_string(),
                    "CALIFORNIA LOVE".to_string(),
                ],
                menu_items: vec![
                    ("Smoky Greens".to_string(), "$29".to_string()),
                    ("California Love".to_string(), "$28.50".to_string()),
                    ("Furious Diablo".to_string(), "$25".to_string()),
                    ("Mamma Moto".to_string(), "$29".to_string()),
                    ("Mr. Moto".to_string(), "$28.50".to_string()),
                    ("Da Italian".to_string(), "$28.50".to_string()),
                    ("Mission Blvd".to_string(), "$29".to_string()),
                    ("Rocky Mountains".to_string(), "$28.50".to_string()),
                    ("The Hulk".to_string(), "$32".to_string()),
                    ("Extra Virgin".to_string(), "$28.50".to_string()),
                    ("Piece of Art".to_string(), "$32".to_string()),
                ]
            }

            // AdSense Banner Bottom
            AdSense {
                banner_type: "banner",
                ad_slot: "4444444444"  // Replace with your ad slot ID
            }

            // Restaurant 4: California Pizza Kitchen
            RestaurantSection {
                name: "CALIFORNIA PIZZA KITCHEN (CPK)",
                restaurant_type: "ORIGINAL BBQ CHICKEN PIZZA",
                address: "53 LOCATIONS ACROSS CALIFORNIA",
                slides: vec![
                    "ORIGINAL BBQ CHICKEN".to_string(),
                    "THAI CHICKEN".to_string(),
                    "CARNE ASADA".to_string(),
                    "WILD MUSHROOM".to_string(),
                ],
                menu_items: vec![
                    ("Original BBQ Chicken".to_string(), "★".to_string()),
                    ("Burnt Ends BBQ".to_string(), "★".to_string()),
                    ("Wild Mushroom".to_string(), "★".to_string()),
                    ("California Club".to_string(), "★".to_string()),
                    ("California Veggie".to_string(), "★".to_string()),
                    ("The Works".to_string(), "★".to_string()),
                    ("Mushroom Pepperoni".to_string(), "★".to_string()),
                    ("Five Cheese".to_string(), "★".to_string()),
                    ("Margherita".to_string(), "★".to_string()),
                    ("Hawaiian".to_string(), "★".to_string()),
                    ("Neapolitan Burrata".to_string(), "★".to_string()),
                    ("Habanero Carnitas".to_string(), "★".to_string()),
                    ("Cacio E Pepe".to_string(), "★".to_string()),
                    ("Thai Chicken".to_string(), "★".to_string()),
                    ("Sicilian".to_string(), "★".to_string()),
                    ("Spicy Chipotle Chicken".to_string(), "★".to_string()),
                    ("Carne Asada".to_string(), "★".to_string()),
                    ("Tostada Pizza".to_string(), "★".to_string()),
                ]
            }
        }

        Footer {
            on_advertise_click: move |_| show_advertise_modal.set(true),
            on_about_click: move |_| show_about_modal.set(true),
            on_contact_click: move |_| show_contact_modal.set(true),
            on_terms_click: move |_| show_terms_modal.set(true),
        }

        // Modals
        AdvertiseModal {
            show: show_advertise_modal(),
            on_close: move |_| show_advertise_modal.set(false)
        }

        AboutModal {
            show: show_about_modal(),
            on_close: move |_| show_about_modal.set(false)
        }

        ContactModal {
            show: show_contact_modal(),
            on_close: move |_| show_contact_modal.set(false)
        }

        TermsModal {
            show: show_terms_modal(),
            on_close: move |_| show_terms_modal.set(false)
        }
    }
}

#[component]
fn LocationBar() -> Element {
    let location = use_signal(|| "CALIFORNIA".to_string());

    // Note: For real geolocation, you would use web-sys or gloo to access navigator.geolocation
    // and then reverse geocode using a service like OpenStreetMap Nominatim API

    rsx! {
        div { class: "location-bar",
            div { class: "location-text",
                div { class: "location-icon" }
                span { id: "user-location", "{location}" }
            }
        }
    }
}

#[component]
fn WebsiteHeader() -> Element {
    rsx! {
        div { class: "website-header",
            h1 { class: "website-title", "BESTPIZZASINCALIFORNIA.COM" }
            div { class: "website-tagline", "DISCOVER CALIFORNIA'S FINEST PIZZA" }
        }
    }
}

#[component]
fn AdSense(banner_type: String, ad_slot: String) -> Element {
    let class_name = if banner_type == "infeed" {
        "adsense-container adsense-infeed"
    } else {
        "adsense-container adsense-banner"
    };

    // Different ad formats for different placements
    let (data_ad_format, data_ad_layout_key) = if banner_type == "infeed" {
        ("fluid", "-fb+5w+4e-db+86")
    } else {
        ("auto", "")
    };

    rsx! {
        div { class: "{class_name}",
            div { class: "adsense-label", "AD" }
            // AdSense ad unit - Using Auto ads
            // Google will automatically optimize ad placement and types
            ins {
                class: "adsbygoogle",
                style: "display:block",
                "data-ad-client": "ca-pub-5369604889706863",
                "data-ad-slot": "{ad_slot}",
                "data-ad-format": "{data_ad_format}",
                "data-full-width-responsive": "true",
                "data-ad-layout-key": if banner_type == "infeed" { data_ad_layout_key } else { "" }
            }
        }
    }
}

#[component]
fn RestaurantSection(
    name: String,
    restaurant_type: String,
    address: String,
    slides: Vec<String>,
    menu_items: Vec<(String, String)>,
) -> Element {
    rsx! {
        div { class: "restaurant-section",
            div { class: "restaurant-header",
                div { class: "restaurant-info",
                    h2 { "{name}" }
                    div { class: "restaurant-type", "{restaurant_type}" }
                    div { class: "restaurant-address", "{address}" }
                }
                div { class: "status-badge", "OPEN NOW" }
            }

            Carousel { slides: slides.clone() }
        }
    }
}

#[component]
fn Carousel(slides: Vec<String>) -> Element {
    let mut current_index = use_signal(|| 0);
    let slide_count = slides.len();

    // Auto-rotate carousel
    let _ = use_resource(move || async move {
        loop {
            gloo_timers::future::sleep(std::time::Duration::from_millis(5000)).await;
            current_index.set((current_index() + 1) % slide_count);
        }
    });

    let go_prev = move |_| {
        if current_index() > 0 {
            current_index.set(current_index() - 1);
        }
    };

    let go_next = move |_| {
        if current_index() < slide_count - 1 {
            current_index.set(current_index() + 1);
        }
    };

    let transform_style = format!("transform: translateX(-{}%)", current_index() * 25);

    rsx! {
        div { class: "carousel-container",
            div {
                class: "carousel-track",
                style: "{transform_style}",
                {slides.iter().map(|slide| rsx! {
                    div { class: "carousel-slide",
                        div { class: "pizza-placeholder", "{slide}" }
                    }
                })}
            }

            div { class: "carousel-controls",
                button {
                    class: "carousel-btn prev",
                    onclick: go_prev,
                    "<"
                }
                button {
                    class: "carousel-btn next",
                    onclick: go_next,
                    ">"
                }
            }

            div { class: "carousel-indicators",
                {(0..slide_count).map(|i| {
                    let is_active = i == current_index();
                    let indicator_class = if is_active { "indicator active" } else { "indicator" };
                    let idx = i;
                    rsx! {
                        button {
                            key: "{i}",
                            class: "{indicator_class}",
                            onclick: move |_| current_index.set(idx),
                        }
                    }
                })}
            }
        }
    }
}

#[component]
fn MenuGrid(items: Vec<(String, String)>) -> Element {
    rsx! {
        div { class: "menu-grid",
            {items.iter().map(|(name, price)| rsx! {
                div { class: "menu-item",
                    div { class: "menu-item-name", "{name}" }
                    div { class: "menu-item-price", "{price}" }
                }
            })}
        }
    }
}

#[component]
fn Footer(
    on_advertise_click: EventHandler<MouseEvent>,
    on_about_click: EventHandler<MouseEvent>,
    on_contact_click: EventHandler<MouseEvent>,
    on_terms_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "footer-overlay",
            div { class: "footer-content",
                div { class: "footer-main",
                    div { class: "footer-links",
                        a {
                            href: "#",
                            class: "footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_about_click.call(e);
                            },
                            "ABOUT"
                        }
                        a {
                            href: "#",
                            class: "footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_contact_click.call(e);
                            },
                            "CONTACT"
                        }
                        a {
                            href: "#",
                            class: "footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_advertise_click.call(e);
                            },
                            "ADVERTISE"
                        }
                        a {
                            href: "#",
                            class: "footer-link",
                            onclick: move |e| {
                                e.prevent_default();
                                on_terms_click.call(e);
                            },
                            "TERMS"
                        }
                    }
                }
                div { class: "footer-trademark",
                    div { class: "footer-text", "© 2025 BESTPIZZASINCALIFORNIA.COM" }
                }
            }
        }
    }
}

#[component]
fn AdvertiseModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal show" } else { "modal" };

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
                        div { "PHONE: 1-800-PIZZA-CA" }
                    }
                }
                button { class: "modal-cta", "GET STARTED" }
            }
        }
    }
}

#[component]
fn AboutModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal show" } else { "modal" };

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
fn ContactModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal show" } else { "modal" };

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
                        div { "PHONE: 1-800-PIZZA-CA" }
                    }
                }
                div { class: "modal-section",
                    h3 { class: "modal-subtitle", "RESTAURANT PARTNERSHIPS" }
                    div { class: "modal-contact",
                        div { "EMAIL: PARTNERS@BESTPIZZASINCALIFORNIA.COM" }
                        div { "PHONE: 1-800-PIZZA-BZ" }
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
fn TermsModal(show: bool, on_close: EventHandler<MouseEvent>) -> Element {
    let modal_class = if show { "modal show" } else { "modal" };

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
