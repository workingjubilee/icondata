#[cfg(feature = "HiMdSolidBugAnt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidBugAnt")]
/// *This icon requires the feature* `HiMdSolidBugAnt` *to be enabled*.
#[component]
pub fn BugAnt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M6.56061 1.13953C6.8982 1.37954 6.9773 1.84778 6.73729 2.18537C6.54915 2.45 6.39257 2.73822 6.27282 3.04462C6.45824 3.21559 6.6554 3.37394 6.86291 3.51833C7.59496 2.59421 8.72765 2.00006 10 2.00006C11.2723 2.00006 12.405 2.59421 13.1371 3.51832C13.3446 3.37394 13.5418 3.21558 13.7272 3.04462C13.6074 2.73822 13.4509 2.45 13.2627 2.18537C13.0227 1.84778 13.1018 1.37955 13.4394 1.13953C13.777 0.899519 14.2452 0.978621 14.4852 1.31621C14.8539 1.83478 15.1353 2.4206 15.3072 3.05202C15.3752 3.30189 15.3095 3.56919 15.1334 3.75907C14.7474 4.17534 14.3111 4.54466 13.8342 4.85743C13.9421 5.21997 14 5.60366 14 6.00006C14 6.52021 13.6988 6.96322 13.277 7.18703C12.9093 7.38217 12.5221 7.54547 12.1193 7.6733C12.2496 7.88101 12.3502 8.10924 12.4152 8.35204C13.8282 8.17757 15.194 7.85199 16.4961 7.39205C16.4987 7.26171 16.5 7.13104 16.5 7.00006C16.5 6.34965 16.4682 5.70689 16.4061 5.07324C16.3656 4.661 16.6671 4.29405 17.0793 4.25364C17.4915 4.21322 17.8585 4.51464 17.8989 4.92688C17.9658 5.6091 18 6.3007 18 7.00006C18 7.32005 17.9928 7.63842 17.9787 7.95502C17.9653 8.25337 17.7762 8.51537 17.4972 8.62201C15.867 9.24525 14.1395 9.67115 12.3441 9.87064C12.3126 9.9553 12.2767 10.0378 12.2367 10.1178C12.8138 10.179 13.384 10.2635 13.9464 10.3704C15.1681 10.6028 16.3525 10.941 17.4895 11.3751C17.7934 11.4912 17.9879 11.79 17.9709 12.115C17.8783 13.8835 17.5668 15.5949 17.0628 17.222C16.9402 17.6177 16.5201 17.839 16.1244 17.7165C15.7287 17.5939 15.5074 17.1738 15.6299 16.7781C16.0452 15.4376 16.3198 14.0349 16.4364 12.5871C15.941 12.4139 15.4366 12.2602 14.9239 12.1269C14.9739 12.4106 15 12.7024 15 13.0001C15 14.8138 14.4833 16.312 13.5738 17.3694C12.6592 18.4326 11.3874 19.0001 10 19.0001C8.61265 19.0001 7.34085 18.4326 6.42621 17.3694C5.51672 16.312 5 14.8138 5 13.0001C5 12.7024 5.02608 12.4106 5.07611 12.1269C4.56343 12.2602 4.05896 12.4139 3.56361 12.5871C3.68017 14.0349 3.95479 15.4376 4.37006 16.7781C4.49264 17.1738 4.27125 17.5939 3.87559 17.7165C3.47993 17.839 3.05982 17.6177 2.93724 17.222C2.43318 15.5949 2.12168 13.8836 2.02911 12.115C2.0121 11.79 2.20656 11.4912 2.51054 11.3751C3.6475 10.941 4.83186 10.6028 6.05361 10.3704C6.61601 10.2635 7.18625 10.179 7.76335 10.1178C7.72328 10.0378 7.68738 9.9553 7.65593 9.87064C5.86054 9.67116 4.13298 9.24526 2.50277 8.62202C2.2238 8.51537 2.0347 8.25338 2.02134 7.95503C2.00716 7.63843 2 7.32005 2 7.00006C2 6.3007 2.03421 5.6091 2.1011 4.92688C2.14151 4.51464 2.50846 4.21322 2.9207 4.25364C3.33293 4.29405 3.63436 4.661 3.59394 5.07324C3.53182 5.70689 3.5 6.34965 3.5 7.00006C3.5 7.13104 3.50129 7.26171 3.50386 7.39206C4.80604 7.852 6.17184 8.17757 7.5848 8.35204C7.64979 8.10924 7.75041 7.88101 7.88072 7.6733C7.47789 7.54547 7.09072 7.38217 6.72298 7.18703C6.30121 6.96322 6 6.52021 6 6.00006C6 5.60366 6.05787 5.21996 6.16579 4.85743C5.68885 4.54466 5.25263 4.17534 4.86656 3.75907C4.69045 3.56919 4.62477 3.30189 4.69281 3.05202C4.86474 2.4206 5.14609 1.83478 5.51477 1.31621C5.75478 0.978621 6.22302 0.899519 6.56061 1.13953Z" fill="#0F172A" /></svg>
   }
}