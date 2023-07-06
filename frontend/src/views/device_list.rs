use sycamore::prelude::*;
use fireplace::devices::Device;

use crate::components::DeviceField;

#[component(inline_props)]
pub fn DeviceList<'a, G: Html>(
    cx: Scope<'a>,
    devices: &'a ReadSignal<Vec<Device>>,
) -> View<G> {

    view! { cx,
        main(class="tripple-column"){ 
            h2{"all devices"}
            Indexed(
                iterable=devices,
                view=|cx, x| view! { cx,
                    DeviceField(device=x)
                },
            )
        }
    }
}
