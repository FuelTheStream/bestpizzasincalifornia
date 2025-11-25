use dioxus::prelude::*;
use crate::components::layout::{WebsiteHeader, Footer};
use crate::components::ads::AdSense;
use crate::components::modals::{AdvertiseModal, AboutModal, ContactModal, TermsModal};
use crate::components::restaurant::RestaurantSection;

#[component]
pub fn App() -> Element {
    let mut show_advertise_modal = use_signal(|| false);
    let mut show_about_modal = use_signal(|| false);
    let mut show_contact_modal = use_signal(|| false);
    let mut show_terms_modal = use_signal(|| false);

    rsx! {
        // Global styles
        document::Link { rel: "stylesheet", href: "/assets/global.css" }
        document::Link { rel: "stylesheet", href: "/assets/layout.css" }
        document::Link { rel: "stylesheet", href: "/assets/carousel.css" }
        document::Link { rel: "stylesheet", href: "/assets/restaurant.css" }
        document::Link { rel: "stylesheet", href: "/assets/ads.css" }
        document::Link { rel: "stylesheet", href: "/assets/modals.css" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Tektur:wght@400;500;700;900&display=swap"
        }

        // SEO Meta Tags
        document::Title { "Best Pizzas in California | Top Pizza Restaurants & Artisan Pizzerias" }
        document::Meta {
            name: "description",
            content: "Discover California's finest pizza restaurants including Pitfire Pizza, CPK, and award-winning artisan pizzerias. Find the best pizza near you with authentic recipes and premium ingredients."
        }
        document::Meta {
            name: "keywords",
            content: "best pizza california, california pizza, pitfire pizza, cpk, artisan pizza, pizza restaurants california, pizza near me, best pizzerias"
        }
        document::Meta { name: "author", content: "BestPizzasInCalifornia.com" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta { charset: "UTF-8" }

        // Canonical URL
        document::Link { rel: "canonical", href: "https://bestpizzasincalifornia.com" }

        // Open Graph / Facebook
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "https://bestpizzasincalifornia.com" }
        document::Meta { property: "og:title", content: "Best Pizzas in California | Top Pizza Restaurants" }
        document::Meta {
            property: "og:description",
            content: "Discover California's finest pizza restaurants including Pitfire Pizza, CPK, and award-winning artisan pizzerias."
        }
        document::Meta { property: "og:site_name", content: "BestPizzasInCalifornia.com" }

        // Twitter Card
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:url", content: "https://bestpizzasincalifornia.com" }
        document::Meta { name: "twitter:title", content: "Best Pizzas in California | Top Pizza Restaurants" }
        document::Meta {
            name: "twitter:description",
            content: "Discover California's finest pizza restaurants including Pitfire Pizza, CPK, and award-winning artisan pizzerias."
        }

        // Google AdSense Meta Tag for Verification
        document::Meta {
            name: "google-adsense-account",
            content: "ca-pub-5369604889706863"
        }

        // Structured Data (JSON-LD) - WebSite Schema
        document::Script {
            r#type: "application/ld+json",
            dangerous_inner_html: r#"{{
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": "Best Pizzas in California",
                "url": "https://bestpizzasincalifornia.com",
                "description": "California's premier guide to the best pizza restaurants and artisan pizzerias",
                "potentialAction": {{
                    "@type": "SearchAction",
                    "target": "https://bestpizzasincalifornia.com/?s={{search_term_string}}",
                    "query-input": "required name=search_term_string"
                }}
            }}"#
        }

        // Structured Data - Restaurant Collection
        document::Script {
            r#type: "application/ld+json",
            dangerous_inner_html: r#"{{
                "@context": "https://schema.org",
                "@type": "ItemList",
                "name": "Top Pizza Restaurants in California",
                "description": "Curated list of the best pizza restaurants across California",
                "itemListElement": [
                    {{
                        "@type": "ListItem",
                        "position": 1,
                        "item": {{
                            "@type": "Restaurant",
                            "name": "Pitfire Pizza",
                            "servesCuisine": "Pizza",
                            "priceRange": "$$",
                            "address": {{
                                "@type": "PostalAddress",
                                "addressRegion": "CA",
                                "addressCountry": "US"
                            }},
                            "description": "Artisan pizza with 9 locations across California featuring wood-fired pizzas with premium ingredients"
                        }}
                    }},
                    {{
                        "@type": "ListItem",
                        "position": 2,
                        "item": {{
                            "@type": "Restaurant",
                            "name": "Ciao Pasta",
                            "servesCuisine": "Italian, Pizza",
                            "priceRange": "$$",
                            "address": {{
                                "@type": "PostalAddress",
                                "addressLocality": "San Juan Capistrano",
                                "addressRegion": "CA",
                                "postalCode": "92675",
                                "addressCountry": "US"
                            }},
                            "description": "Authentic thin crust Italian pizza in San Juan Capistrano"
                        }}
                    }},
                    {{
                        "@type": "ListItem",
                        "position": 3,
                        "item": {{
                            "@type": "Restaurant",
                            "name": "Mr Moto",
                            "servesCuisine": "Pizza",
                            "priceRange": "$$$",
                            "address": {{
                                "@type": "PostalAddress",
                                "addressLocality": "San Diego",
                                "addressRegion": "CA",
                                "addressCountry": "US"
                            }},
                            "description": "Innovative white and red pies in San Diego with unique flavor combinations"
                        }}
                    }},
                    {{
                        "@type": "ListItem",
                        "position": 4,
                        "item": {{
                            "@type": "Restaurant",
                            "name": "California Pizza Kitchen",
                            "alternateName": "CPK",
                            "servesCuisine": "Pizza, American",
                            "priceRange": "$$",
                            "address": {{
                                "@type": "PostalAddress",
                                "addressRegion": "CA",
                                "addressCountry": "US"
                            }},
                            "description": "Famous for Original BBQ Chicken Pizza with 53 locations across California"
                        }}
                    }}
                ]
            }}"#
        }

        // Google AdSense Script
        document::Script {
            src: "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-5369604889706863",
            crossorigin: "anonymous",
            async: true
        }

        WebsiteHeader {}

        div { class: "restaurant-container",
            // AdSense Banner Top
            AdSense {
                banner_type: "banner",
                ad_slot: "1111111111"
            }

            // Restaurant 1: PITFIRE PIZZA
            RestaurantSection {
                name: "PITFIRE PIZZA",
                restaurant_type: "ARTISAN PIZZA",
                address: "9 LOCATIONS ACROSS CALIFORNIA",
                slides: vec![
                    "/assets/pitfire/the-new-margherita.png".to_string(),
                    "/assets/pitfire/vegan-margherita.png".to_string(),
                    "/assets/pitfire/zoes-pepperoni.png".to_string(),
                    "/assets/pitfire/field-mushroom.png".to_string(),
                    "/assets/pitfire/mushroom-extra.png".to_string(),
                    "/assets/pitfire/say-cheese.png".to_string(),
                    "/assets/pitfire/honey-bear.png".to_string(),
                    "/assets/pitfire/pineapple-express.png".to_string(),
                    "/assets/pitfire/epic-bbq-chicken.png".to_string(),
                    "/assets/pitfire/sausage-party.png".to_string(),
                    "/assets/pitfire/the-burrata.png".to_string(),
                    "/assets/pitfire/smashing-pumpkin.png".to_string(),
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

            // Ad 1
            AdSense {
                banner_type: "infeed",
                ad_slot: "2222222222"
            }

            // Restaurant 2: CIAO PASTA
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

            // Ad 2
            AdSense {
                banner_type: "banner",
                ad_slot: "3333333333"
            }

            // Restaurant 3: MR MOTTO
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

            // Ad 3
            AdSense {
                banner_type: "banner",
                ad_slot: "4444444444"
            }

            // Restaurant 4: CPK
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
