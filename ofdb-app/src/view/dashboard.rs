// TODO: use super::page;
// TODO: use crate::core::entities::*;
// TODO: use maud::{html, Markup};
// TODO: 
// TODO: pub struct DashBoardPresenter {
// TODO:     pub user: User,
// TODO:     pub place_count: usize,
// TODO:     pub event_count: usize,
// TODO:     pub tag_count: usize,
// TODO:     pub user_count: usize,
// TODO: }
// TODO: 
// TODO: pub fn dashboard(data: DashBoardPresenter) -> Markup {
// TODO:     page(
// TODO:         "Admin Dashboard",
// TODO:         Some(&data.user.email),
// TODO:         None,
// TODO:         None,
// TODO:         html! {
// TODO:             main class="dashboard" {
// TODO:                 h3 { "Database Statistics" }
// TODO:                 table {
// TODO:                     tr {
// TODO:                         td {"Number of Places"}
// TODO:                         td {(data.place_count)}
// TODO:                     }
// TODO:                     tr {
// TODO:                         td {"Number of Events"}
// TODO:                         td {(data.event_count)}
// TODO:                     }
// TODO:                     tr {
// TODO:                         td {"Number of Users"}
// TODO:                         td {(data.user_count)}
// TODO:                     }
// TODO:                     tr {
// TODO:                         td {"Number of Tags"}
// TODO:                         td {(data.tag_count)}
// TODO:                     }
// TODO:                 }
// TODO:                 h3 { "User Management" }
// TODO:                 (super::search_users_form())
// TODO:             }
// TODO:         },
// TODO:     )
// TODO: }
