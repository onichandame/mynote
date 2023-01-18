use dioxus::prelude::*;
use dioxus_router::Link;

use crate::app::page::_route;

struct Tile {
    to: &'static str,
    title: &'static str,
    subtitle: &'static str,
    icon: &'static str, // url
}

static TILES:&[Tile]=&[
    Tile {
        to: _route::MEMOS,
        title: "Memo",
        subtitle: "Record anything",
        icon: "https://notebook.onichandame.com/static/3a95c03de945575537b880c2e0a882cc/60b4d/memo-icon.webp",
    },
];

pub fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        div{
            class:"tile is-ancestor",
            div{
                class:"tile is-parent",
                TILES.iter().map(|tile|
                    rsx!(
                        div{
                            class:"tile is-child p-2",
                            self::tile{
                                to:tile.to,
                                title:tile.title,
                                subtitle:tile.subtitle,
                                icon:tile.icon
                            }
                        }
                    )
                )
            }
        }
    })
}

#[inline_props]
fn tile(
    cx: Scope,
    to: &'static str,
    title: &'static str,
    subtitle: &'static str,
    icon: &'static str,
) -> Element<'a> {
    cx.render(rsx! {
        Link{
            to:to,
            div{
                class:"card",
                div{
                    class:"card-content",
                    div{
                        class:"media",
                        div{
                            style:"overflow:visible;",
                            class:"media-content",
                            p{
                                class:"title",
                                *title
                            }
                            p{
                                class:"subtitle",
                                *subtitle
                            }
                        }
                        div{
                            class:"media-right",
                            figure{
                                class:"image is-64x64",
                                img{
                                    src:*icon
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
