#[cfg(feature = "RiHealthLineInfraredThermometer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiHealthLineInfraredThermometer")]
/// *This icon requires the feature* `RiHealthLineInfraredThermometer` *to be enabled*.
#[component]
pub fn InfraredThermometer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M21 2v9h-3.001L18 12c0 2.21-1.79 4-4 4h-1.379l-.613 3.111.911 1.321c.314.455.2 1.078-.255 1.391-.167.115-.365.177-.568.177H3l2.313-10.024L3 11l4-9h14zm-2 2H8.3L5.655 9.95l1.985.837L5.514 20h4.678l-.309-.448L11.96 9H19V4zm-3.001 7h-2.394l-.591 3H14c1.105 0 2-.895 2-2l-.001-1z" /></g></svg>
   }
}