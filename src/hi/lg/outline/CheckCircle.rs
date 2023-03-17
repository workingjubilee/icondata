#[cfg(feature = "HiLgOutlineCheckCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineCheckCircle")]
/// *This icon requires the feature* `HiLgOutlineCheckCircle` *to be enabled*.
#[component]
pub fn CheckCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9 12.75L11.25 15L15 9.75M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}