use dioxus::prelude::*;

#[component]
pub fn AdSense(banner_type: String, ad_slot: String) -> Element {
    let class_name = if banner_type == "infeed" {
        "ads-container ads-infeed"
    } else {
        "ads-container ads-banner"
    };

    let (data_ad_format, data_ad_layout_key) = if banner_type == "infeed" {
        ("fluid", "-fb+5w+4e-db+86")
    } else {
        ("auto", "")
    };

    rsx! {
        div { class: "{class_name}",
            div { class: "ads-label", "AD" }
            ins {
                class: "adsbygoogle",
                style: "display:block; width:100%; height:100%;",
                "data-ad-client": "ca-pub-5369604889706863",
                "data-ad-slot": "{ad_slot}",
                "data-ad-format": "{data_ad_format}",
                "data-full-width-responsive": "true",
                "data-ad-layout-key": if banner_type == "infeed" { data_ad_layout_key } else { "" }
            }
        }
    }
}
