// TODO: use super::page::*;
// TODO: use maud::{html, Markup};
// TODO: use rocket::request::FlashMessage;
// TODO: 
// TODO: pub fn reset_password_request(flash: Option<FlashMessage>, action: &str) -> Markup {
// TODO:     page(
// TODO:         "Request password reset",
// TODO:         None,
// TODO:         flash,
// TODO:         None,
// TODO:         html! {
// TODO:           h2 { "Password reset" }
// TODO:           p { "Please enter your email address to reset your password." }
// TODO:           form class="reset-pw-req" action=(action) method="POST" {
// TODO:               fieldset{
// TODO:                 label {
// TODO:                     "eMail:"
// TODO:                     br;
// TODO:                     input type="text" name="email" placeholder="eMail address";
// TODO:                 }
// TODO:                 br;
// TODO:                 input type="submit" value="reset";
// TODO:               }
// TODO:           }
// TODO:         },
// TODO:     )
// TODO: }
// TODO: 
// TODO: pub fn reset_password_request_ack(flash: Option<FlashMessage>) -> Markup {
// TODO:     page(
// TODO:         "Request password reset",
// TODO:         None,
// TODO:         flash,
// TODO:         None,
// TODO:         html! {
// TODO:           h2 { "Password reset" }
// TODO:           p {
// TODO:             "Your request to reset your password was successfully send."
// TODO:             br;
// TODO:             "Please look into your email inbox to continue."
// TODO:           }
// TODO:         },
// TODO:     )
// TODO: }
// TODO: 
// TODO: pub fn reset_password(flash: Option<FlashMessage>, action: &str, token: &str) -> Markup {
// TODO:     page(
// TODO:         "New password",
// TODO:         None,
// TODO:         flash,
// TODO:         None,
// TODO:         html! {
// TODO:           form class="reset-pw" action=(action) method="POST" {
// TODO:               fieldset{
// TODO:                 label{
// TODO:                     "New password:"
// TODO:                     br;
// TODO:                     input type="password" name="new_password" placeholder="new password";
// TODO:                 }
// TODO:                 br;
// TODO:                 label{
// TODO:                     "New password (repeated):"
// TODO:                     br;
// TODO:                     input type="password" name="new_password_repeated" placeholder="repeat new password";
// TODO:                 }
// TODO:                 br;
// TODO:                 input type="hidden" name="token" value=(token);
// TODO:                 input type="submit" value="save";
// TODO:               }
// TODO:           }
// TODO:         },
// TODO:     )
// TODO: }
// TODO: 
// TODO: pub fn reset_password_ack(flash: Option<FlashMessage>) -> Markup {
// TODO:     page(
// TODO:         "New password",
// TODO:         None,
// TODO:         flash,
// TODO:         None,
// TODO:         html! {
// TODO:           h2 { "New password" }
// TODO:           p { "Your password changed successfully." }
// TODO:           p { a href = "/login" { "Login with your new password."} }
// TODO:         },
// TODO:     )
// TODO: }
