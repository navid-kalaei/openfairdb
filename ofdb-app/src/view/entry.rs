// TODO: use super::{address_to_html, leaflet_css_link, map_scripts, page};
// TODO: use crate::core::prelude::*;
// TODO: use maud::{html, Markup};
// TODO: use std::collections::HashMap;
// TODO: 
// TODO: type Ratings = Vec<(Rating, Vec<Comment>)>;
// TODO: 
// TODO: pub struct EntryPresenter {
// TODO:     pub place: Place,
// TODO:     pub ratings: HashMap<RatingContext, Ratings>,
// TODO:     pub allow_archiving: bool,
// TODO: }
// TODO: 
// TODO: impl From<(Place, Vec<(Rating, Vec<Comment>)>, Role)> for EntryPresenter {
// TODO:     fn from((place, rtngs, role): (Place, Vec<(Rating, Vec<Comment>)>, Role)) -> EntryPresenter {
// TODO:         let mut p: EntryPresenter = (place, rtngs).into();
// TODO:         p.allow_archiving = match role {
// TODO:             Role::Admin | Role::Scout => true,
// TODO:             _ => false,
// TODO:         };
// TODO:         p
// TODO:     }
// TODO: }
// TODO: 
// TODO: impl From<(Place, Vec<(Rating, Vec<Comment>)>)> for EntryPresenter {
// TODO:     fn from((place, rtngs): (Place, Vec<(Rating, Vec<Comment>)>)) -> EntryPresenter {
// TODO:         let mut ratings: HashMap<RatingContext, Ratings> = HashMap::new();
// TODO: 
// TODO:         for (r, comments) in rtngs {
// TODO:             if let Some(x) = ratings.get_mut(&r.context) {
// TODO:                 x.push((r, comments));
// TODO:             } else {
// TODO:                 ratings.insert(r.context, vec![(r, comments)]);
// TODO:             }
// TODO:         }
// TODO:         let allow_archiving = false;
// TODO:         EntryPresenter {
// TODO:             place,
// TODO:             ratings,
// TODO:             allow_archiving,
// TODO:         }
// TODO:     }
// TODO: }
// TODO: 
// TODO: pub fn entry(email: Option<&str>, e: EntryPresenter) -> Markup {
// TODO:     page(
// TODO:         &format!("{} | OpenFairDB", e.place.title),
// TODO:         email,
// TODO:         None,
// TODO:         Some(leaflet_css_link()),
// TODO:         entry_detail(e),
// TODO:     )
// TODO: }
// TODO: 
// TODO: fn entry_detail(e: EntryPresenter) -> Markup {
// TODO:     let rev = format!("v{}", u64::from(e.place.revision));
// TODO:     html! {
// TODO:         h3 {
// TODO:             (e.place.title)
// TODO:             " "
// TODO:             span class="rev" {
// TODO:                 "("
// TODO:                 @if e.allow_archiving {
// TODO:                      a href=(format!("/places/{}/history", e.place.id)) { (rev) }
// TODO:                      " | "
// TODO:                      a href=(format!("/places/{}/review", e.place.id)) { "review" }
// TODO:                 } @else {
// TODO:                     (rev)
// TODO:                 }
// TODO:                 ")"
// TODO:             }
// TODO:         }
// TODO:         p {(e.place.description)}
// TODO:         p {
// TODO:             table {
// TODO:                 @if let Some(ref l) = e.place.links {
// TODO:                     @if let Some(ref h) = l.homepage {
// TODO:                         tr {
// TODO:                             td { "Homepage" }
// TODO:                             td { a href=(h) { (h) } }
// TODO:                         }
// TODO:                     }
// TODO:                 }
// TODO:                 @if let Some(ref c) = e.place.contact {
// TODO:                     @if let Some(ref m) = c.email {
// TODO:                         tr {
// TODO:                             td { "eMail" }
// TODO:                             td { a href=(format!("mailto:{}",m)) { (m) } }
// TODO:                         }
// TODO:                     }
// TODO:                     @if let Some(ref t) = c.phone {
// TODO:                         tr {
// TODO:                             td { "Phone" }
// TODO:                             td { a href=(format!("tel:{}",t)) { (t) } }
// TODO:                         }
// TODO:                     }
// TODO:                 }
// TODO:                 @if let Some(ref a) = e.place.location.address {
// TODO:                     @if !a.is_empty() {
// TODO:                         tr {
// TODO:                             td { "Address" }
// TODO:                             td { (address_to_html(&a)) }
// TODO:                         }
// TODO:                     }
// TODO:                 }
// TODO:             }
// TODO:         }
// TODO:         p {
// TODO:             ul {
// TODO:                 @for t in &e.place.tags{
// TODO:                     li{ (format!("#{}", t)) }
// TODO:                 }
// TODO:             }
// TODO:         }
// TODO:         h3 { "Ratings" }
// TODO: 
// TODO:         @for (ctx, ratings) in e.ratings {
// TODO:             h4 { (format!("{:?}",ctx)) }
// TODO:             ul {
// TODO:                 @for (r,comments) in ratings {
// TODO:                     li {
// TODO:                         (rating(e.place.id.as_ref(), e.allow_archiving, &r, &comments))
// TODO:                     }
// TODO:                 }
// TODO:             }
// TODO:         }
// TODO:         div id="map" style="height:300px;" { }
// TODO:         (map_scripts(&[e.place.into()]))
// TODO:     }
// TODO: }
// TODO: 
// TODO: fn rating(place_id: &str, archive: bool, r: &Rating, comments: &[Comment]) -> Markup {
// TODO:     html! {
// TODO:       h5 { (r.title) " " span { (format!("({})",i8::from(r.value))) } }
// TODO:       @if archive {
// TODO:         form action = "/ratings/actions/archive" method = "POST" {
// TODO:             input type="hidden" name="ids" value=(r.id.to_string());
// TODO:             input type="hidden" name="place_id" value=(place_id);
// TODO:             input type="submit" value="archive rating";
// TODO:         }
// TODO:       }
// TODO:       @if let Some(ref src) = r.source {
// TODO:           p { (format!("source: {}",src)) }
// TODO:       }
// TODO:       ul {
// TODO:           @for c in comments {
// TODO:               li {
// TODO:                   p { (c.text) }
// TODO:                   @if archive {
// TODO:                     form action = "/comments/actions/archive" method = "POST" {
// TODO:                         input type="hidden" name="ids" value=(c.id.to_string());
// TODO:                         input type="hidden" name="place_id" value=(place_id);
// TODO:                         input type="submit" value="archive comment";
// TODO:                     }
// TODO:                   }
// TODO:               }
// TODO:           }
// TODO:       }
// TODO:     }
// TODO: }
