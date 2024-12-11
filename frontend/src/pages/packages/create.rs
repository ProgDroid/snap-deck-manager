use crate::components::edit_form::EditForm;

use common::package::Package;
use yew::prelude::*;

#[function_component(PackageCreate)]
pub fn package_create() -> Html {
    return html! {
        <EditForm<Package> given_object={None} />
    };
}
