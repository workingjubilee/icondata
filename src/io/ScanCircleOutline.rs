#[cfg(feature = "IoScanCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoScanCircleOutline")]
/// *This icon requires the feature* `IoScanCircleOutline` *to be enabled*.
#[component]
pub fn ScanCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><path d="M296,352h28a28,28,0,0,0,28-28V296" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M352,216V188a28,28,0,0,0-28-28H296" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M216,352H188a28,28,0,0,1-28-28V296" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M160,216V188a28,28,0,0,1,28-28h28" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}