#[cfg(feature = "VsExport")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsExport")]
/// *This icon requires the feature* `VsExport` *to be enabled*.
#[component]
pub fn Export(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.086 7l-2.39-2.398.702-.704L15 7.5l-3.602 3.602-.703-.704 2.383-2.382V8H3V7h10.086zM1 4h1v7H1V4z" /></svg>
   }
}