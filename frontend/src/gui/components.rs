use yew::prelude::*;

// all of these these should ideally be moved somewhere else
#[derive(PartialEq, Properties)]
pub struct AccordionProps {
    pub style: Option<String>,
    pub id: Option<String>,
    pub children: Children,
}

#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    html! {
        <div class="accordion" style={props.style.clone()} id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

#[function_component]
pub fn AccordionItem(props: &AccordionProps) -> Html {
    html! {
        <div class="accordion-item" style={props.style.clone()} id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct AccordionButtonProps {
    pub style: Option<String>,
    pub id: Option<String>,
    pub children: Children,
    pub data_bs_target: String,
}

#[function_component]
pub fn AccordionButton(props: &AccordionButtonProps) -> Html {
    html! {
        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target={props.data_bs_target.clone()} aria-expanded="false" aria-controls={props.id.clone()}>
            { props.children.clone() }
        </button>
    }
}

#[derive(PartialEq, Properties)]
pub struct AccordionHeaderProps {
    pub style: Option<String>,
    pub id: Option<String>,
    pub children: Children,
    pub data_bs_target: String,
}

#[function_component]
pub fn AccordionHeader(props: &AccordionHeaderProps) -> Html {
    html! {
        <h2 class="accordion-header" style={props.style.clone()} id={props.id.clone()}>
            <AccordionButton id={props.id.clone()} style={props.style.clone()} data_bs_target={props.data_bs_target.clone()}>
                { props.children.clone() }
            </AccordionButton>
        </h2>
    }
}

#[derive(PartialEq, Properties)]
pub struct AccordionCollapseProps {
    pub style: Option<String>,
    pub id: Option<String>,
    pub children: Children, // why does this have to be pub if the struct is pub
    pub data_bs_parent: String,
}

#[function_component]
pub fn AccordionCollapse(props: &AccordionCollapseProps) -> Html {
    html! {
        <div class="accordion-collapse collapse" id={props.id.clone()} aria-labelledby={props.id.clone()} data-bs-parent={props.data_bs_parent.clone()}>
            <div class="accordion-body">
                { props.children.clone() }
            </div>
        </div>
    }
}
