#[cfg(feature = "RiEditorAsterisk")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorAsterisk")]
/// *This icon requires the feature* `RiEditorAsterisk` *to be enabled*.
#[component]
pub fn Asterisk(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13 3v7.267l6.294-3.633 1 1.732-6.293 3.633 6.293 3.635-1 1.732L13 13.732V21h-2v-7.268l-6.294 3.634-1-1.732L9.999 12 3.706 8.366l1-1.732L11 10.267V3z" /></g></svg>
   }
}