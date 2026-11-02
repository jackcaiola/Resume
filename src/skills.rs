use yew::prelude::*;
use yewtil::NeqAssign;

use crate::SkillCategory;

#[derive(Clone, Properties, PartialEq)]
pub struct SkillProps {
    pub skills: Vec<SkillCategory>,
    pub selected_tags: Vec<String>,
    pub on_tags: Callback<Vec<String>>,
    pub on_toggle: Callback<String>,
}

pub enum Msg {
    Hover(String),
    Clear,
    Toggle(String),
}

pub struct SkillComponent {
    props: SkillProps,
    hovered: Option<String>,
    link: ComponentLink<Self>,
}

impl Component for SkillComponent {
    type Message = Msg;
    type Properties = SkillProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            hovered: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover(tag) => {
                self.props.on_tags.emit(vec![tag.clone()]);
                self.hovered = Some(tag);
            }
            Msg::Clear => {
                self.hovered = None;
                self.props.on_tags.emit(Vec::new());
            }
            Msg::Toggle(tag) => self.props.on_toggle.emit(tag),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2>{ "SKILLS" }</h2>
            <ul class="skill-list">
                { for self.props.skills.iter().map(|cat| self.view_category(cat)) }
            </ul>
            </>
        }
    }
}

impl SkillComponent {
    fn view_category(&self, cat: &SkillCategory) -> Html {
        let tags = cat.tags.iter();
        html! {
            <li>
                <h3>{ &cat.category }</h3>
                <div class="tag-container">
                    { for tags.map(|tag| self.view_tag(&tag)) }
                </div>
            </li>
        }
    }

    fn view_tag(&self, tag: &str) -> Html {
        let mut class = if self
            .props
            .selected_tags
            .iter()
            .any(|selected| selected == tag)
        {
            "tag-item tag-selected".to_string()
        } else {
            "tag-item".to_string()
        };
        let send_tag = tag.to_string();
        let mouseover = self.link.callback(move |_| Msg::Hover(send_tag.to_owned()));
        let mouseout = self.link.callback(|_| Msg::Clear);
        if let Some(hov_tag) = &self.hovered {
            if hov_tag == tag {
                class = format!("{} tag-selected tag-hover", class);
            }
        }
        let focus_tag = tag.to_string();
        let focus = self.link.callback(move |_| Msg::Hover(focus_tag.clone()));
        let blur = self.link.callback(|_| Msg::Clear);
        let click_tag = tag.to_string();
        let click = self.link.callback(move |_| Msg::Toggle(click_tag.clone()));
        html! {
            <button type="button" class=class onclick=click onfocus=focus onblur=blur onmouseover=mouseover onmouseout=mouseout>
                { tag }
            </button>
        }
    }
}
