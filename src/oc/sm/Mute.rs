#[cfg(feature = "OcSmMute")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmMute")]
/// *This icon requires the feature* `OcSmMute` *to be enabled*.
#[component]
pub fn Mute(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 2.75v10.5a.751.751 0 0 1-1.238.57L3.473 11H1.75A1.75 1.75 0 0 1 0 9.25v-2.5C0 5.784.784 5 1.75 5h1.722l3.29-2.82A.75.75 0 0 1 8 2.75Zm3.28 2.47L13 6.94l1.72-1.72a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042L14.06 8l1.72 1.72a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L13 9.06l-1.72 1.72a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734L11.94 8l-1.72-1.72a.749.749 0 0 1 .326-1.275.749.749 0 0 1 .734.215Zm-7.042 1.1a.752.752 0 0 1-.488.18h-2a.25.25 0 0 0-.25.25v2.5c0 .138.112.25.25.25h2c.179 0 .352.064.488.18L6.5 11.62V4.38Z" /></svg>
   }
}