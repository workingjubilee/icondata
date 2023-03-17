#[cfg(feature = "TbArrowUpTail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowUpTail")]
/// *This icon requires the feature* `TbArrowUpTail` *to be enabled*.
#[component]
pub fn ArrowUpTail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-up-tail" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 18l0 -15" /><path d="M15 6l-3 -3l-3 3" /><path d="M15 21l-3 -3l-3 3" /></svg>
   }
}