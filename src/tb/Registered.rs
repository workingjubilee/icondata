#[cfg(feature = "TbRegistered")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRegistered")]
/// *This icon requires the feature* `TbRegistered` *to be enabled*.
#[component]
pub fn Registered(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-registered" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M10 15v-6h2a2 2 0 1 1 0 4h-2" /><path d="M14 15l-2 -2" /></svg>
   }
}