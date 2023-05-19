use crate::{Model, Msg};
use fireplace::eventhandler::{EventName, EventType};
use seed::{prelude::*, *};

pub fn events(model: &Model) -> Node<Msg> {
    let mut last_events: Vec<Node<Msg>> = Vec::new();
    let mut last_actions: Vec<Node<Msg>> = Vec::new();

    for event in &model.last_events {
        let entry = li! {
            event.id.to_string(),
            br!(),
            ul!(
                li!(
                    event.timestamp.to_string(),
                ),
                li!(
                    event.event.to_string(),
                )
            )
        };
        if event.event != EventName::NewData {
            if event.event_type == EventType::Action {
                last_actions.push(entry);
            } else {
                last_events.push(entry);
            }
        }
    }

    main![
        C!("dual-column"),
        h2!("Last Events/Actions"),
        article!(h3!("Events"), ul!(last_events)),
        article!(h3!("Actions"), ul!(last_actions))
    ]
}
