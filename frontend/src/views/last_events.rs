use crate::{Model, Msg};
use seed::{prelude::*, *};

pub fn events(model: &Model) -> Node<Msg> {
    let mut last_events: Vec<Node<Msg>> = Vec::new();
    for event in &model.last_events {
        last_events.push(li! {
            event.event.id.to_string(),
            br!(),
            ul!(
                li!(
                    event.timestamp.to_string(),
                ),
                li!(
                    event.event.event.to_string(),
                )
            )
        });
    }

    let mut last_actions: Vec<Node<Msg>> = Vec::new();
    for event in &model.last_actions {
        last_actions.push(li! {
            event.event.id.to_string(),
            br!(),
            ul!(
                li!(
                    event.timestamp.to_string(),
                ),
                li!(
                    event.event.event.to_string(),
                )
            )
        });
    }

    main![
        C!("dual-column"),
        h2!("Last Events/Actions"),
        article!(h3!("Events"), ul!(last_events)),
        article!(h3!("Actions"), ul!(last_actions))
    ]
}
