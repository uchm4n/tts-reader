//! SVG icon definitions for player controls.

#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn PlayIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M8 5.14v14.72a1 1 0 0 0 1.5.86l11.5-7.36a1 1 0 0 0 0-1.72L9.5 4.28a1 1 0 0 0-1.5.86Z",
                fill: "currentColor",
            }
        }
    }
}

pub fn PauseIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M6 4h4v16H6V4Zm8 0h4v16h-4V4Z",
                fill: "currentColor",
            }
        }
    }
}

pub fn StopIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M6 6h12v12H6V6Z",
                fill: "currentColor",
            }
        }
    }
}

pub fn FastBackwardIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M11 18V6l-8.5 6 8.5 6Zm.5-6 8.5 6V6l-8.5 6Z",
                fill: "currentColor",
            }
        }
    }
}

pub fn FastForwardIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M4 18l8.5-6L4 6v12Zm9-12v12l8.5-6L13 6Z",
                fill: "currentColor",
            }
        }
    }
}

pub fn InfoIcon() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10Zm-1-11v6h2v-6h-2Zm0-4v2h2V7h-2Z",
                fill: "currentColor",
            }
        }
    }
}
