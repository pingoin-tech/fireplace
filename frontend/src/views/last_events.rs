use fireplace::eventhandler::{Event};
use sycamore::prelude::*;

#[component(inline_props)]
pub fn EventView<'a, G: Html>(
    cx: Scope<'a>,
    last_events: &'a ReadSignal<Vec<Event>>,
    last_actions: &'a ReadSignal<Vec<Event>>,
) -> View<G> {
    view! { cx,
        main(class="dual-column"){
            h2{"Last Events/Actions"}
            article{
                h3{"Events"}
                ul{
                    Indexed(
                        iterable=last_events,
                        view=|cx, x| view! { cx,
                            EventElementView(event=x.clone())
                        },
                    )
                }
            }
            article{
                h3{"Actions"}
                ul{
                    Indexed(
                        iterable=last_actions,
                        view=|cx, x| view! { cx,
                            EventElementView(event=x.clone())
                        },
                    )
                }
            }
        }
    }
}

#[component(inline_props)]
pub fn EventElementView<'a, G: Html>(cx: Scope<'a>, event:Event) -> View<G> {
    view! { cx,
    li{
        (event.id)
        br{}
        ul{
            li{
                (event.timestamp)
            }
            li{
                (event.event)
            }
        }
    }
    }
}
