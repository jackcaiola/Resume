use yew::prelude::*;
use yewtil::NeqAssign;

use crate::date_range::DateRangeComponent;
use crate::location::LocationComponent;
use crate::{Duty, Experience};

#[derive(Clone, Properties, PartialEq)]
pub struct ExperienceProps {
    pub experience: Vec<Experience>,
    pub selected_tags: Vec<String>,
    pub on_tags: Callback<Vec<String>>,
}

pub enum Msg {
    Hover(Vec<String>),
    Clear,
}

pub struct ExperienceComponent {
    props: ExperienceProps,
    link: ComponentLink<Self>,
}

impl Component for ExperienceComponent {
    type Message = Msg;
    type Properties = ExperienceProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover(tags) => self.props.on_tags.emit(tags),
            Msg::Clear => self.props.on_tags.emit(Vec::new()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2>{ "EXPERIENCE" }</h2>
            <ul class="experience-list">
                { for self.props.experience.iter().filter(|exp| !exp.archived).map(|edu| self.view_entry(edu)) }
            </ul>
            </>
        }
    }
}

impl ExperienceComponent {
    fn view_entry(&self, exp: &Experience) -> Html {
        let period = exp.period.clone();
        let location = match exp.parsed_location.clone() {
            Some(location) => html! {
                <span class="detail-loc"><LocationComponent location=location/></span>
            },
            None => html! {},
        };
        let link = if let Some(site) = &exp.website {
            html! { <h4><a href=site.clone()>{ &exp.organization }</a></h4> }
        } else {
            html! { <h4><a>{ &exp.organization }</a></h4> }
        };

        let heading_tags = exp.tags.clone();
        let hover = self
            .link
            .callback(move |_| Msg::Hover(heading_tags.clone()));
        let focus_tags = exp.tags.clone();
        let focus = self.link.callback(move |_| Msg::Hover(focus_tags.clone()));
        let clear = self.link.callback(|_| Msg::Clear);
        let blur = self.link.callback(|_| Msg::Clear);
        let heading_class = if exp
            .tags
            .iter()
            .any(|tag| self.props.selected_tags.contains(tag))
        {
            "duty-selected"
        } else {
            ""
        };
        html! {
            <li>
                <h3 tabindex="0" class=heading_class onmouseover=hover onmouseout=clear onfocus=focus onblur=blur>{ &exp.title }</h3>
                { link }
                <div class="detail">
                    <span class="detail-date"><DateRangeComponent period=period/></span>
                    { location }
                </div>
                <ul class="duty-list">
                    { for exp.duty.iter().map(|duty| self.view_duty(duty)) }
                </ul>
            </li>
        }
    }

    fn view_duty(&self, duty: &Duty) -> Html {
        let tags = duty.tags.clone();
        let mouseover = self.link.callback(move |_| Msg::Hover(tags.clone()));
        let focus_tags = duty.tags.clone();
        let focus = self.link.callback(move |_| Msg::Hover(focus_tags.clone()));
        let mouseout = self.link.callback(|_| Msg::Clear);
        let blur = self.link.callback(|_| Msg::Clear);
        let class = if duty
            .tags
            .iter()
            .any(|tag| self.props.selected_tags.contains(tag))
        {
            "duty-item duty-selected"
        } else {
            "duty-item"
        };
        let project = match &duty.project {
            Some(title) => html! { <div class="project-title">{ title }</div> },
            None => html! {},
        };
        let detail = match &duty.project_detail {
            Some(detail) => html! { <div class="project-detail">{ detail }</div> },
            None => html! {},
        };
        html! {
            <li tabindex="0" class=class onmouseover=mouseover onmouseout=mouseout onfocus=focus onblur=blur>
                { project }
                { detail }
                { &duty.description }
            </li>
        }
    }
}
