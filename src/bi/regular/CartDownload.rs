#[cfg(feature = "BiRegularCartDownload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCartDownload")]
/// *This icon requires the feature* `BiRegularCartDownload` *to be enabled*.
#[component]
pub fn CartDownload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="10.5" cy="19.5" r="1.5" /><circle cx="17.5" cy="19.5" r="1.5" /><path d="m14 13.99 4-5h-3v-4h-2v4h-3l4 5z" /><path d="M17.31 15h-6.64L6.18 4.23A2 2 0 0 0 4.33 3H2v2h2.33l4.75 11.38A1 1 0 0 0 10 17h8a1 1 0 0 0 .93-.64L21.76 9h-2.14z" /></svg>
   }
}