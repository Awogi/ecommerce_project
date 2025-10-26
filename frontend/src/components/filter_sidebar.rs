use yew::prelude::*;
use web_sys::MouseEvent;
use crate::models::{School, Grade, UniformCategory, FilterState};

#[derive(Properties, PartialEq)]
pub struct FilterSidebarProps {
    pub schools: Vec<School>,
    pub grades: Vec<Grade>,
    pub categories: Vec<UniformCategory>,
    pub filters: FilterState,
    pub on_filter_change: Callback<FilterState>,
}

#[function_component(FilterSidebar)]
pub fn filter_sidebar(props: &FilterSidebarProps) -> Html {
    let FilterSidebarProps { schools, grades, categories, filters, on_filter_change } = props;
    
    let filter_state = use_state(|| filters.clone());

    let on_school_change = {
        let filter_state = filter_state.clone();
        let on_filter_change = on_filter_change.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            let school_id = if value.is_empty() { None } else { value.parse().ok() };
            
            let mut new_filters = (*filter_state).clone();
            new_filters.school_id = school_id;
            filter_state.set(new_filters.clone());
            on_filter_change.emit(new_filters);
        })
    };

    let on_grade_change = {
        let filter_state = filter_state.clone();
        let on_filter_change = on_filter_change.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            let grade_id = if value.is_empty() { None } else { value.parse().ok() };
            
            let mut new_filters = (*filter_state).clone();
            new_filters.grade_id = grade_id;
            filter_state.set(new_filters.clone());
            on_filter_change.emit(new_filters);
        })
    };

    let on_category_change = {
        let filter_state = filter_state.clone();
        let on_filter_change = on_filter_change.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            let category_id = if value.is_empty() { None } else { value.parse().ok() };
            
            let mut new_filters = (*filter_state).clone();
            new_filters.category_id = category_id;
            filter_state.set(new_filters.clone());
            on_filter_change.emit(new_filters);
        })
    };

    let on_search_change = {
        let filter_state = filter_state.clone();
        let on_filter_change = on_filter_change.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let search_term = if value.is_empty() { None } else { Some(value) };
            
            let mut new_filters = (*filter_state).clone();
            new_filters.search_term = search_term;
            filter_state.set(new_filters.clone());
            on_filter_change.emit(new_filters);
        })
    };

    let clear_filters = {
        let filter_state = filter_state.clone();
        let on_filter_change = on_filter_change.clone();
        Callback::from(move |_: MouseEvent| {
            let new_filters = FilterState::default();
            filter_state.set(new_filters.clone());
            on_filter_change.emit(new_filters);
        })
    };

    html! {
        <div class="filter-sidebar">
            <div class="filter-header">
                <h3>{"Filters"}</h3>
                <button class="clear-filters-btn" onclick={clear_filters}>{"Clear All"}</button>
            </div>

            <div class="filter-section">
                <label class="filter-label">{"Search"}</label>
                <input
                    type="text"
                    class="filter-input"
                    placeholder="Search uniforms..."
                    value={filter_state.search_term.clone().unwrap_or_default()}
                    onchange={on_search_change}
                />
            </div>

            <div class="filter-section">
                <label class="filter-label">{"School"}</label>
                <select class="filter-select" onchange={on_school_change}>
                    <option value="">{"All Schools"}</option>
                    {for schools.iter().map(|school| {
                        let selected = filter_state.school_id == Some(school.id);
                        html! {
                            <option 
                                value={school.id.to_string()} 
                                selected={selected}
                            >
                                {&school.name}
                            </option>
                        }
                    })}
                </select>
            </div>

            <div class="filter-section">
                <label class="filter-label">{"Grade"}</label>
                <select class="filter-select" onchange={on_grade_change}>
                    <option value="">{"All Grades"}</option>
                    {for grades.iter().map(|grade| {
                        let selected = filter_state.grade_id == Some(grade.id);
                        html! {
                            <option 
                                value={grade.id.to_string()} 
                                selected={selected}
                            >
                                {&grade.name}
                            </option>
                        }
                    })}
                </select>
            </div>

            <div class="filter-section">
                <label class="filter-label">{"Category"}</label>
                <select class="filter-select" onchange={on_category_change}>
                    <option value="">{"All Categories"}</option>
                    {for categories.iter().map(|category| {
                        let selected = filter_state.category_id == Some(category.id);
                        html! {
                            <option 
                                value={category.id.to_string()} 
                                selected={selected}
                            >
                                {&category.name}
                            </option>
                        }
                    })}
                </select>
            </div>

            <div class="filter-section">
                <label class="filter-label">{"Size"}</label>
                <select class="filter-select">
                    <option value="">{"All Sizes"}</option>
                    <option value="XS">{"XS"}</option>
                    <option value="S">{"S"}</option>
                    <option value="M">{"M"}</option>
                    <option value="L">{"L"}</option>
                    <option value="XL">{"XL"}</option>
                    <option value="XXL">{"XXL"}</option>
                </select>
            </div>
        </div>
    }
}