// TODO: use super::page;
// TODO: use crate::core::prelude::*;
// TODO: use maud::{html, Markup};
// TODO: 
// TODO: pub fn place_history(user: &User, h: &PlaceHistory) -> Markup {
// TODO:     page(
// TODO:         "Place History",
// TODO:         Some(&user.email),
// TODO:         None,
// TODO:         None,
// TODO:         html! {
// TODO:             div class="revisions" {
// TODO:                 table {
// TODO:                     thead {
// TODO:                         tr {
// TODO:                             th{ "Revision" }
// TODO:                             th{ "Log"  }
// TODO: 
// TODO:                             th{ "Title" }
// TODO:                             th{ "Description" }
// TODO: 
// TODO:                             th{ "Position" }
// TODO: 
// TODO:                             th{ "Street" }
// TODO:                             th{ "ZIP" }
// TODO:                             th{ "City" }
// TODO:                             th{ "Country" }
// TODO: 
// TODO:                             th{ "E-Mail" }
// TODO:                             th{ "Phone" }
// TODO: 
// TODO:                             th{ "Homepage" }
// TODO:                             th{ "Image" }
// TODO:                             th{ "Image Link" }
// TODO: 
// TODO:                             th{ "Tags" }
// TODO:                         }
// TODO:                     }
// TODO:                     tbody {
// TODO:                         @for (r,logs) in &h.revisions {
// TODO:                             tr {
// TODO:                                 td{ (u64::from(r.revision)) }
// TODO:                                 td{
// TODO:                                     ul class="log" {
// TODO:                                         @for l in logs {
// TODO:                                             li { (review_status_log(r.revision, &l)) }
// TODO:                                         }
// TODO:                                     }
// TODO:                                 }
// TODO: 
// TODO:                                 td{ (r.title) }
// TODO:                                 td{ (r.description) }
// TODO: 
// TODO:                                 td{
// TODO:                                     (format!("{:.2}/{:0.2}",
// TODO:                                         r.location.pos.lat().to_deg(),
// TODO:                                         r.location.pos.lng().to_deg()
// TODO:                                     ))
// TODO:                                 }
// TODO:                                 @if let Some(a) = &r.location.address {
// TODO:                                     td{ @if let Some(x) = &a.street  { (x) } }
// TODO:                                     td{ @if let Some(x) = &a.zip     { (x) } }
// TODO:                                     td{ @if let Some(x) = &a.city    { (x) } }
// TODO:                                     td{ @if let Some(x) = &a.country { (x) } }
// TODO:                                 } @else {
// TODO:                                     td{ }
// TODO:                                     td{ }
// TODO:                                     td{ }
// TODO:                                     td{ }
// TODO:                                 }
// TODO:                                 @if let Some(c) = &r.contact {
// TODO:                                     td{ @if let Some(x) = &c.email  { a href=(format!("mailto:{}",x)) { (x) } } }
// TODO:                                     td{ @if let Some(x) = &c.phone  { (x) } }
// TODO:                                 } @else {
// TODO:                                     td {}
// TODO:                                     td {}
// TODO:                                 }
// TODO:                                 @if let Some(l) = &r.links {
// TODO:                                     td{ @if let Some(x) = &l.homepage   { a href=(x) { (x) } } }
// TODO:                                     td{ @if let Some(x) = &l.image      { img src=(x); } }
// TODO:                                     td{ @if let Some(x) = &l.image_href { a href=(x) { (x) } } }
// TODO:                                 } @else {
// TODO:                                     td {}
// TODO:                                     td {}
// TODO:                                     td {}
// TODO:                                 }
// TODO:                                 td{
// TODO:                                     ul class="tags" {
// TODO:                                         @for t in &r.tags {
// TODO:                                             li { (t) }
// TODO:                                         }
// TODO:                                     }
// TODO:                                 }
// TODO:                             }
// TODO:                         }
// TODO:                     }
// TODO:                 }
// TODO:             }
// TODO:         },
// TODO:     )
// TODO: }
// TODO: 
// TODO: fn review_status_log(place_rev: Revision, l: &ReviewStatusLog) -> Markup {
// TODO:     use ReviewStatus as S;
// TODO:     let status = match l.status {
// TODO:         S::Rejected => "Rejected",
// TODO:         S::Archived => "Archived",
// TODO:         S::Created => {
// TODO:             if place_rev.is_initial() {
// TODO:                 "Created"
// TODO:             } else {
// TODO:                 "Modified"
// TODO:             }
// TODO:         }
// TODO:         S::Confirmed => "Confirmed",
// TODO:     };
// TODO:     html! {
// TODO:         span class="status" { (status) }
// TODO:         " at "
// TODO:         (l.activity.activity.at)
// TODO:         " by "
// TODO:         @if let Some(email) = &l.activity.activity.by {
// TODO:             (email)
// TODO:         } @else {
// TODO:             "anonymous visitor"
// TODO:         }
// TODO:         @if let Some(c) = &l.activity.comment {
// TODO:             span class="comment" { " \"" (c) "\"" }
// TODO:         }
// TODO:         @if let Some(c) = &l.activity.context {
// TODO:             span class="context" { " (" (c) ")" }
// TODO:         }
// TODO:     }
// TODO: }
// TODO: 
// TODO: pub fn place_review(email: &str, place: &Place, status: ReviewStatus) -> Markup {
// TODO:     use ReviewStatus as S;
// TODO:     let options = [
// TODO:         (S::Rejected, "reject"),
// TODO:         (S::Archived, "archive"),
// TODO:         (S::Confirmed, "confirm"),
// TODO:     ];
// TODO:     page(
// TODO:         "Place Review",
// TODO:         Some(&email),
// TODO:         None,
// TODO:         None,
// TODO:         html! {
// TODO:             div class="review" {
// TODO:                 h2 { (format!("Add Review for \"{}\"", place.title)) }
// TODO:                 form action=(format!("/places/{}/review", place.id)) method="POST" {
// TODO:                     fieldset {
// TODO:                         label {
// TODO:                             "Comment:"
// TODO:                             br;
// TODO:                             input required? name="comment" placeholder="Comment";
// TODO:                         }
// TODO:                         br;
// TODO:                         label { "Action:"
// TODO:                             br;
// TODO:                             select name="status" {
// TODO:                                 @for (o, label) in options.iter() {
// TODO:                                     option value=(i16::from(*o)) disabled?[*o==status] { (label) }
// TODO:                                 }
// TODO:                             }
// TODO:                         }
// TODO:                     }
// TODO:                     input type="submit" value="change";
// TODO:                 }
// TODO:             }
// TODO:         },
// TODO:     )
// TODO: }
