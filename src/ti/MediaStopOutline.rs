#[cfg(feature = "TiMediaStopOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaStopOutline")]
/// *This icon requires the feature* `TiMediaStopOutline` *to be enabled*.
#[component]
pub fn MediaStopOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M16 8v8h-8v-8h8m0-2h-8c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2z" /></svg>
   }
}