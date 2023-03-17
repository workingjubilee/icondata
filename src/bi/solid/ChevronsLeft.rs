#[cfg(feature = "BiSolidChevronsLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronsLeft")]
/// *This icon requires the feature* `BiSolidChevronsLeft` *to be enabled*.
#[component]
pub fn ChevronsLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m8.121 12 4.94-4.939-2.122-2.122L3.879 12l7.06 7.061 2.122-2.122z" /><path d="M17.939 4.939 10.879 12l7.06 7.061 2.122-2.122L15.121 12l4.94-4.939z" /></svg>
   }
}