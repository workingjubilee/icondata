#[cfg(feature = "VsDebugReverseContinue")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugReverseContinue")]
/// *This icon requires the feature* `VsDebugReverseContinue` *to be enabled*.
#[component]
pub fn DebugReverseContinue(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.5 2H12v12h1.5V2zm-4.936.39L9.75 3v10l-1.186.61-7-5V7.39l7-5zM3.29 8l4.96 3.543V4.457L3.29 8z" /></svg>
   }
}