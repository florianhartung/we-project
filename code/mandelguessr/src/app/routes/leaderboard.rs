use leptos::{component, create_effect, create_resource, create_server_action, view, CollectView, IntoView};

use crate::api::game::{list_games, ListGamesAction};

#[component]
pub fn Leaderboard() -> impl IntoView {
    let games = create_resource(|| (), |()| async { 
        let mut games = list_games().await.unwrap();
        games.sort_by_key(|game| game.score);
        games.reverse();
        games
    });


    view! {
        <h1 class="text-white text-3xl font-bold p-4">Rangliste</h1>
        <ul class="text-white text-md p-4">
        {move || games.map(move |games| {
            games.iter().map(move |game|
                view! {
                    <li> {game.score.clone()}
                    " Punkte: "
                    {game.username.clone()}</li>
                }
            ).collect_view()
        })}
        </ul>
    }
}
