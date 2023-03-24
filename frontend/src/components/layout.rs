use seed::{prelude::*, *};

use crate::{router::Page, Model, Msg};

pub fn view_head(model: &Model) -> Node<Msg> {
    header![
        div![img![attrs![At::Src=>"logo.svg",At::Height=>"60"]]],
        div! {"Fireplace"},
        div! {&model.version},
    ]
}

pub fn view_nav(model: &Model) -> Node<Msg> {
    let mut links: Vec<Node<Msg>> = Vec::new();
    for link in &model.links {
        links.push(li!(a![
            C!["router-link-active", "router-link-exact-active"],
            attrs!(At::Href=>link.address),
            &link.name
        ]));
    }

    nav![ul![
        li!(a![
            C!["router-link-active", "router-link-exact-active"],
            attrs!(At::Href=>"/"),
            "Home"
        ]),
        li!(a![
            C!["router-link-active", "router-link-exact-active"],
            ev(Ev::Click, |_| Msg::SetView(Page::DeviceList)),
            "Device List"
        ]),
        li!(a![
            C!["router-link-active", "router-link-exact-active"],
            ev(Ev::Click, |_| Msg::SetView(Page::LastAction)),
            "Last Events"
        ]),
        links
    ]]
}

pub fn view_foot() -> Node<Msg> {
    footer!["© Pingoin-Tech"]
}
