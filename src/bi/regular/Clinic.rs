#[cfg(feature = "BiRegularClinic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularClinic")]
/// *This icon requires the feature* `BiRegularClinic` *to be enabled*.
#[component]
pub fn Clinic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.707 2.293a.999.999 0 0 0-1.414 0l-9 9A1 1 0 0 0 3 13h1v7c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2v-7h1a.999.999 0 0 0 .707-1.707l-9-9zM18.001 20H6v-9.586l6-6 6 6V15l.001 5z" /><path d="M13 10h-2v3H8v2h3v3h2v-3h3v-2h-3z" /></svg>
   }
}