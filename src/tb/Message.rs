#[cfg(feature = "TbMessage")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMessage")]
/// *This icon requires the feature* `TbMessage` *to be enabled*.
#[component]
pub fn Message(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-message" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 21v-13a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-9l-4 4" /><path d="M8 9l8 0" /><path d="M8 13l6 0" /></svg>
   }
}