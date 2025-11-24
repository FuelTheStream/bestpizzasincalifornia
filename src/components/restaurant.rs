use dioxus::prelude::*;
use super::carousel::Carousel;

#[component]
pub fn RestaurantSection(
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
                div { class: "restaurant-status-badge", "OPEN NOW" }
            }

            Carousel { slides: slides.clone() }
        }
    }
}

#[component]
pub fn MenuGrid(items: Vec<(String, String)>) -> Element {
    rsx! {
        div { class: "restaurant-menu-grid",
            {items.iter().map(|(name, price)| rsx! {
                div { class: "restaurant-menu-item",
                    div { class: "restaurant-menu-item-name", "{name}" }
                    div { class: "restaurant-menu-item-price", "{price}" }
                }
            })}
        }
    }
}
