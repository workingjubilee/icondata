#[cfg(feature = "BiRegularX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularX")]
/// *This icon requires the feature* `BiRegularX` *to be enabled*.
#[component]
pub fn X(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m16.192 6.344-4.243 4.242-4.242-4.242-1.414 1.414L10.535 12l-4.242 4.242 1.414 1.414 4.242-4.242 4.243 4.242 1.414-1.414L13.364 12l4.242-4.242z" /></svg>
   }
}