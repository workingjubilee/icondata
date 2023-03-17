#[cfg(feature = "TbPolygon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPolygon")]
/// *This icon requires the feature* `TbPolygon` *to be enabled*.
#[component]
pub fn Polygon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-polygon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 5m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M19 8m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M5 11m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M15 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M6.5 9.5l3.5 -3" /><path d="M14 5.5l3 1.5" /><path d="M18.5 10l-2.5 7" /><path d="M13.5 17.5l-7 -5" /></svg>
   }
}