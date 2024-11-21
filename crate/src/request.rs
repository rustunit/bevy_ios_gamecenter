use std::marker::PhantomData;

use bevy_app::{App, PreUpdate};
use bevy_ecs::{
    prelude::*,
    system::{EntityCommands, IntoObserverSystem, SystemParam},
};

use crate::{
    IosGCAuthResult, IosGCLoadGamesResponse, IosGCResolvedConflictsResponse, IosGCSaveGame,
    IosGCSaveGames, IosGCSaveGamesResponse, IosGCSavedGameResponse, IosGamecenterEvents,
};

#[derive(Resource, Default)]
struct BevyIosGamecenterState {
    request_id: i64,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestAuthentication;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestSaveGames;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestSaveGame;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestLoadGame;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct RequestResolveConflicts;

#[derive(Component)]
struct RequestId(i64);

#[derive(Component)]
struct RequestEntity;

#[derive(SystemParam)]
pub struct BevyIosGamecenter<'w, 's> {
    commands: Commands<'w, 's>,
    res: ResMut<'w, BevyIosGamecenterState>,
}

impl<'w, 's> BevyIosGamecenter<'w, 's> {
    /// Triggers `Trigger<IosGCAuthResult>` as response
    pub fn authenticate(&mut self) -> BevyIosGCRequestBuilder<'_, IosGCAuthResult> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::authenticate(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestAuthentication,
            RequestId(id),
            RequestEntity,
        )))
    }

    /// Triggers `Trigger<IosGCSaveGamesResponse>` as response
    pub fn fetch_save_games(&mut self) -> BevyIosGCRequestBuilder<'_, IosGCSaveGamesResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::fetch_save_games(id);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestSaveGames,
            RequestId(id),
            RequestEntity,
        )))
    }

    /// Triggers `Trigger<IosGCSavedGameResponse>` as response
    pub fn save_game(
        &mut self,
        name: String,
        data: &[u8],
    ) -> BevyIosGCRequestBuilder<'_, IosGCSavedGameResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::save_game(id, name, data);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestSaveGame,
            RequestId(id),
            RequestEntity,
        )))
    }

    /// Triggers `Trigger<IosGCLoadGamesResponse>` as response
    pub fn load_game(
        &mut self,
        game: IosGCSaveGame,
    ) -> BevyIosGCRequestBuilder<'_, IosGCLoadGamesResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::load_game(id, game);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestLoadGame,
            RequestId(id),
            RequestEntity,
        )))
    }

    /// Triggers `Trigger<IosGCResolvedConflictsResponse>` as response
    pub fn resolve_conflicts(
        &mut self,
        save_games: IosGCSaveGames,
        data: &[u8],
    ) -> BevyIosGCRequestBuilder<'_, IosGCResolvedConflictsResponse> {
        let id = self.res.request_id;
        self.res.request_id += 1;
        crate::methods::resolve_conflicting_games(id, save_games, data);
        BevyIosGCRequestBuilder::new(self.commands.spawn((
            RequestResolveConflicts,
            RequestId(id),
            RequestEntity,
        )))
    }

    // TODO: support remaining methods:
    // * delete_savegame
    // * fetch_signature
    // * achievement_progress
    // * achievements_reset
    // * leaderboards_score
}

pub struct BevyIosGCRequestBuilder<'a, T>((EntityCommands<'a>, PhantomData<T>));

impl<'a, T> BevyIosGCRequestBuilder<'a, T>
where
    T: 'static + Event,
{
    fn new(ec: EntityCommands<'a>) -> Self {
        Self((ec, PhantomData))
    }

    pub fn on_response<RB: Bundle, RM, OR: IntoObserverSystem<T, RB, RM>>(
        &mut self,
        on_response: OR,
    ) -> &mut Self {
        self.0 .0.observe(on_response);
        self
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct BevyIosGamecenterSet;

pub fn plugin(app: &mut App) {
    app.init_resource::<BevyIosGamecenterState>();
    app.add_systems(
        PreUpdate,
        (
            cleanup_finished_requests,
            process_events.run_if(on_event::<IosGamecenterEvents>()),
        )
            .chain()
            .in_set(BevyIosGamecenterSet),
    );
}

fn cleanup_finished_requests(
    mut commands: Commands,
    query: Query<Entity, (With<RequestEntity>, Without<RequestId>)>,
) {
    for e in query.iter() {
        if let Some(mut ec) = commands.get_entity(e) {
            ec.despawn();
        }
    }
}

#[allow(unused_variables, unused_mut)]
fn process_events(
    mut events: EventReader<IosGamecenterEvents>,
    mut commands: Commands,
    query_request_authentication: Query<(Entity, &RequestId), With<RequestAuthentication>>,
    query_request_save_games: Query<(Entity, &RequestId), With<RequestSaveGames>>,
    query_request_save_game: Query<(Entity, &RequestId), With<RequestSaveGame>>,
    query_request_load_game: Query<(Entity, &RequestId), With<RequestLoadGame>>,
    query_request_resolve_conflicts: Query<(Entity, &RequestId), With<RequestResolveConflicts>>,
) {
    for e in events.read() {
        match e {
            IosGamecenterEvents::Authentication((r, response)) => {
                for (e, id) in &query_request_authentication {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::SaveGames((r, response)) => {
                for (e, id) in &query_request_save_games {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::SavedGame((r, response)) => {
                for (e, id) in &query_request_save_game {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::LoadGame((r, response)) => {
                for (e, id) in &query_request_load_game {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }
            IosGamecenterEvents::ResolvedConflicts((r, response)) => {
                for (e, id) in &query_request_resolve_conflicts {
                    if id.0 == *r {
                        commands.trigger_targets(response.clone(), e);
                        if let Some(mut ec) = commands.get_entity(e) {
                            ec.remove::<RequestId>();
                        }
                        break;
                    }
                }
            }

            _ => {}
        }
    }
}
