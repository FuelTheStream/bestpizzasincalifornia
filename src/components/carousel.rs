use dioxus::prelude::*;

#[component]
pub fn Carousel(slides: Vec<String>) -> Element {
    let mut current_index = use_signal(|| 0);
    let slide_count = slides.len();
    let max_index = calculate_max_carousel_index(slide_count);

    let go_prev = move |_| {
        if current_index() > 0 {
            current_index.set(current_index() - 1);
        }
    };

    let go_next = move |_| {
        if current_index() < max_index {
            current_index.set(current_index() + 1);
        }
    };

    let transform_style = format!("transform: translateX(calc(-{} * var(--slide-width-percent, 25%)))", current_index());

    rsx! {
        div { class: "carousel-container",
            div {
                class: "carousel-track",
                style: "{transform_style}",
                {slides.iter().map(|slide| render_carousel_slide(slide))}
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
                    let indicator_class = if is_active {
                        "carousel-indicator carousel-indicator-active"
                    } else {
                        "carousel-indicator"
                    };
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

fn calculate_max_carousel_index(slide_count: usize) -> usize {
    if slide_count > 1 {
        slide_count - 1
    } else {
        0
    }
}

fn render_carousel_slide(slide: &String) -> Element {
    rsx! {
        div { class: "carousel-slide",
            if slide.contains('/') || slide.ends_with(".png") || slide.ends_with(".jpg") {
                img {
                    src: "{slide}",
                    alt: "Pizza",
                    style: "width: 100%; height: 100%; object-fit: cover; pointer-events: none; user-select: none;",
                    draggable: "false"
                }
            } else {
                div { class: "carousel-pizza-placeholder", "{slide}" }
            }
        }
    }
}
