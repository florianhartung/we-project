use std::ops::Range;

use leptos::{
    component, create_effect, create_local_resource, create_rw_signal, create_server_action, view, IntoView, SignalGet, SignalSet, Suspense
};

use crate::{api::game::{end_game, start_game, EndGameAction}, app::components::common::Mandelbrot};

#[component]
pub fn Game() -> impl IntoView {
    // TODO: start game with request
    let game_info = create_local_resource(|| (), |()| async { start_game().await.unwrap() });

    let initial_position = create_rw_signal((0.0, 0.0));
    let initial_zoom_exponent = create_rw_signal(0.0);

    let position = create_rw_signal::<(f32, f32)>((0.0, 0.0));
    let zoom_exponent = create_rw_signal::<f32>(0.0);

    let finder_position = create_rw_signal((0.0, 0.0));
    let finder_zoom_exponent = create_rw_signal(0.0);

    let score = create_rw_signal(None::<u32>);

    let end_game_action = create_server_action::<EndGameAction>();

    create_effect(move |_| {
        if let Some(game_info) = game_info.get() {
            initial_position.set(game_info.position);
            initial_zoom_exponent.set(game_info.zoom_exponent);
            position.set(game_info.position);
            zoom_exponent.set(game_info.zoom_exponent);
        }
    });

    view! {
        {move || {
            if let Some(score_value) = score.get() {
                view!{
                    <div class="flex flex-row h-full justify-center mt-20 space-x-8">
                        <h1 class="text-white text-3xl font-bold">"🎉 Du hast "{score_value}" Punkte erziehtl! 🎉"</h1>
                        <br/>
                        <button class="text-white rounded-full text-2xl font-bold bg-[#600070]" on:click=move |_| {
                            score.set(None);
                            game_info.refetch();
                            finder_position.set((0.0, 0.0));
                            finder_zoom_exponent.set(0.0);
                        }>
                            Nächstes Spiel
                        </button>
                    </div>
                }
            } else {
                view! {
                    <div class="flex flex-row h-full justify-center mt-20 space-x-8">
                        <div class="flex flex-col ">
                            <div class="w-full text-center font-bold text-4xl text-white">Suche dieses Muster</div>
                            <div class="w-full text-center font-bold text-md text-white">Du kannst die Kamera etwas bewegen,<br/> dann zählt die neu ausgewählte Position als die, welche es zu finden gilt.</div>
                            <Suspense fallback=|| view!{"Lädt neues Spiel..."}>
                                {move || {
                                        game_info.get().map(|_| {
                                            view! {
                                                <Mandelbrot
                                                    size=(500, 375) position=position zoom_exponent=zoom_exponent
                                                    position_bounds=((initial_position.get().0 - 0.3)..(initial_position.get().0 + 0.3), (initial_position.get().1 - 0.3)..(initial_position.get().1 + 0.3))
                                                    zoom_exponent_bounds=(initial_zoom_exponent.get() - 0.3)..(initial_zoom_exponent.get() + 0.3)
                                                    class="rounded-lg shadow-lg w-[500px] h-[375px] self-center"
                                                />
                                            }
                                        })
                                    }
                                }
                            </Suspense>
                            <div class="w-full text-center font-bold text-md text-white">Notiz: Manchmal ist die zufällig gewählte Startposition schlecht gewählt.<br/>In diesen Fällen einfach F5 drücken.</div>
                        </div>
                        <div class="relative">
                            <Mandelbrot
                                size=(800, 600) position=finder_position zoom_exponent=finder_zoom_exponent
                                position_bounds=MANDELBROT_POSITION_BOUNDS
                                zoom_exponent_bounds=(0.0..4.5)
                                class="rounded-lg shadow-lg"
                            />
                            <button on:click=move |_| {
                                let position = position.get();
                                let finder_position = finder_position.get();

                                let position_delta = ((position.0 - finder_position.0).abs(), (position.1 - finder_position.1).abs());
                                let distance = (position_delta.0.powf(2.0) + position_delta.1.powf(2.0)).powf(0.5);
                                let game_score = (100.0 * 50.0_f32.powf(-distance)) as u32;
                                score.set(Some(game_score));
                                end_game_action.dispatch(EndGameAction { score: game_score });
                            } class="absolute top-[520px] left-[250px] bg-[#1AA404]/70 hover:bg-[#1AA404]/100 rounded-full w-[300px] px-4 py-2 font-bold text-white text-xl">
                                "Auswahl bestätigen"<br/>
                                "und Spiel beenden"
                            </button>
                        </div>
                        // TODO: show points from this game. ask for another game
                    </div>
                }
            }
        }}
    }
}

const MANDELBROT_POSITION_BOUNDS: (Range<f32>, Range<f32>) = (-2.0..0.5, -1.2..1.2);
