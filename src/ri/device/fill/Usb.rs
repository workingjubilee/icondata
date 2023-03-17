#[cfg(feature = "RiDeviceFillUsb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillUsb")]
/// *This icon requires the feature* `RiDeviceFillUsb` *to be enabled*.
#[component]
pub fn Usb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M12 1l3 5h-2v7.381l3-1.499-.001-.882H15V7h4v4h-1.001L18 13.118l-5 2.5v1.553c1.166.412 2 1.523 2 2.829 0 1.657-1.343 3-3 3s-3-1.343-3-3c0-1.187.69-2.213 1.69-2.7L6 14l-.001-2.268C5.402 11.386 5 10.74 5 10c0-1.105.895-2 2-2s2 .895 2 2c0 .74-.402 1.387-1 1.732V13l3 2.086V6H9l3-5z" /></g></svg>
   }
}