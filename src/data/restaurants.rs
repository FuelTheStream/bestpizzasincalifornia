use dioxus::prelude::*;
use crate::components::restaurant::RestaurantSection;

pub fn render_restaurants() -> Element {
    rsx! {
        RestaurantSection {
            name: "PITFIRE PIZZA",
            restaurant_type: "ARTISAN PIZZA",
            address: "9 LOCATIONS ACROSS CALIFORNIA",
            slides: vec![
                "/assets/pitfire/the-new-margherita.png".to_string(),
                "/assets/pitfire/vegan-margherita.png".to_string(),
                "/assets/pitfire/zoes-pepperoni.png".to_string(),
                "/assets/pitfire/field-mushroom.png".to_string(),
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
}
