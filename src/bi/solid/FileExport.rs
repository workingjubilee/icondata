#[cfg(feature = "BiSolidFileExport")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFileExport")]
/// *This icon requires the feature* `BiSolidFileExport` *to be enabled*.
#[component]
pub fn FileExport(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 22a2 2 0 0 0 2-2v-5l-5 4v-3H8v-2h7v-3l5 4V8l-6-6H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12zM13 4l5 5h-5V4z" /></svg>
   }
}