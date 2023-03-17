#[cfg(feature = "SiChartmogul")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiChartmogul")]
/// *This icon requires the feature* `SiChartmogul` *to be enabled*.
#[component]
pub fn Chartmogul(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M10.621 19.89V8.75L2.867 19.89H0V4.11h2.758v11.112l7.754-11.113h2.867v11.14L21.16 4.11H24v15.782h-2.73V8.75l-7.755 11.14Z" /></svg>
   }
}