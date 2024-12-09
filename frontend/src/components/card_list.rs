use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub id: String,
    pub description: String,
    pub art: String,
    pub name: String,
    pub abilities: Vec<String>,
}

#[function_component(CardList)]
pub fn card_list(props: &Props) -> Html {
    let ability = Html::from_html_unchecked(props.description.clone().into());

    html! {
        <tr key={props.id.clone()} class="card-row">
            <td class="card-row-img"><img src={props.art.clone()} /></td>
            <td><h3>{props.name.clone()}</h3></td>
            <td><p>{ability}</p></td>
            <td>
                if !props.abilities.is_empty() {
                    <ul>{
                        props.abilities.clone().into_iter().map(|ability| html! { <li>{ability}</li> }).collect::<Vec<Html>>()
                    }</ul>
                }
            </td>
        </tr>
    }
}
