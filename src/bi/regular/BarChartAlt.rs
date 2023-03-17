#[cfg(feature = "BiRegularBarChartAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBarChartAlt")]
/// *This icon requires the feature* `BiRegularBarChartAlt` *to be enabled*.
#[component]
pub fn BarChartAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 6h2v11h-2zm4-3h2v14h-2zM9 9h2v8H9zM4 19h16v2H4zm1-7h2v5H5z" /></svg>
   }
}