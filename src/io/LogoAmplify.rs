#[cfg(feature = "IoLogoAmplify")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoAmplify")]
/// *This icon requires the feature* `IoLogoAmplify` *to be enabled*.
#[component]
pub fn LogoAmplify(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M112.31,268l40.36-68.69,34.65,59-67.54,115h135L289.31,432H16Zm58.57-99.76,33.27-56.67L392.44,432H325.76ZM222.67,80h66.59L496,432H429.32Z" style="fill-rule:evenodd" /></svg>
   }
}