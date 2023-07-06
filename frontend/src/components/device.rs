use fireplace::devices::Device;
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::utils::post;

#[component(inline_props)]
pub fn DeviceField<'a, G: Html>(cx: Scope<'a>, device: Device) -> View<G> {
    let name = if let Some(name) = device.alias.clone() {
        name
    } else {
        device.id.clone()
    };

    let address = format!("http://{}", &device.ip);

    let actions: View<G> = View::new_fragment(
        device
            .available_actions
            .into_iter()
            .map(|x| {
                let event = x.clone();
                view! { cx,
                    button(on:click=move |_|{
                        let bla=event.clone();
                        spawn_local_scoped(cx, async move {
                            post("/api/trigger-action",bla , move |_|{}).await;
                        })
                    }){ (x.event.to_string())}
                }
            })
            .collect(),
    );

    let values = View::new_fragment(
        device
            .values
            .into_iter()
            .map(|x| {
                view! { cx,
                div { (x.0) }
                div{(format!("{}", x.1))}
                }
            })
            .collect(),
    );

    view! { cx,
        article(class="dual-column"){
            h3{(name)}
            div{"IP"}
            div{
               a(
                Href=address){
                    (device.ip.clone())
                }
                br()
               (device.mac)
            }
            div{"RSSI"}
            div{(format!("{}", device.rssi))}
            (values)
            div{"actions"}
            div{(actions)}
        }
    }
}
