#[cfg(feature = "RiMediaLinePlayList")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaLinePlayList")]
/// *This icon requires the feature* `RiMediaLinePlayList` *to be enabled*.
#[component]
pub fn PlayList(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 18h10v2H2v-2zm0-7h14v2H2v-2zm0-7h20v2H2V4zm17 11.17V9h5v2h-3v7a3 3 0 1 1-2-2.83zM18 19a1 1 0 1 0 0-2 1 1 0 0 0 0 2z" /></g></svg>
   }
}