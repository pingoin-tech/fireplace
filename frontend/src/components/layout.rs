//use seed::{prelude::*, *};
use sycamore::prelude::*;
use fireplace::{config::Link};

#[derive(Prop)]
pub struct HeaderProps<'a> {
    version: &'a ReadSignal<String>,
}

#[component]
pub fn ViewHead<'a, G: Html>(cx: Scope<'a>, props: HeaderProps<'a>) -> View<G>{
    view! { cx,
        header{
            div{}
            div{"Fireplace"}
            div{ (props.version.get())}
        }
        
    }
}


#[derive(Prop)]
pub struct NavProps<'a> {
    links: &'a ReadSignal<Vec<Link>>,
}

#[component]
pub fn ViewNav<'a,G: Html>(cx: Scope<'a>, props: NavProps<'a>) -> View<G> {
    view! { cx,
    nav{ul{
        li{a(class="router-link-active router-link-exact-active", href="/"){
            "Home"
        }}
        li{a(class="router-link-active router-link-exact-active", href="/device-list"){
            "Device List"
        }}
        li{a(class="router-link-active router-link-exact-active", href="/last-events"){
            "Last Events"
        }}
        Indexed(
            iterable=props.links,
            view=|cx, Link { name, address }| view! { cx,
                li {
                    a(href=format!("https://www.youtube.com/watch?v={address}")) {
                        (name)
                    }
                }
            }
        )
        }
    }
}
}
#[component]
pub fn ViewFoot<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
    footer{"© Pingoin-Tech"}
    }
}
