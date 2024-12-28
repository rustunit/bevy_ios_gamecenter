use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_ios_gamecenter::{
    view_states, IosGCPlayer, IosGCSaveGame, IosGCSaveGames, IosGCSaveGamesResponse,
    IosGamecenterEvents,
};
use egui_extras::{Column, TableBuilder};

#[derive(Event)]
pub enum IosGamecenterEguiOpen {
    Toggle,
    Open,
    Close,
}

#[derive(Resource, Default)]
struct DebugUiResource {
    open: bool,
}

#[derive(Resource, Default)]
struct DebugIosGamecenter {
    saves: Vec<IosGCSaveGame>,
    conflicting: Vec<IosGCSaveGame>,
    events: Vec<String>,
    player: IosGCPlayer,
    test_achievement_ids: Vec<String>,
    test_ranking_ids: Vec<String>,
}

pub struct IosGamecenterEguiPlugin {
    pub test_achievement_ids: Vec<String>,
    pub test_ranking_ids: Vec<String>,
}

impl Plugin for IosGamecenterEguiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.init_resource::<DebugUiResource>();
        app.insert_resource(DebugIosGamecenter {
            test_achievement_ids: self.test_achievement_ids.clone(),
            test_ranking_ids: self.test_ranking_ids.clone(),
            ..default()
        });

        app.add_systems(Update, update);
        app.add_systems(
            Update,
            process_gc_events.run_if(on_event::<IosGamecenterEvents>),
        );

        app.add_observer(on_toggle);
    }
}

fn on_toggle(trigger: Trigger<IosGamecenterEguiOpen>, mut res: ResMut<DebugUiResource>) {
    match trigger.event() {
        IosGamecenterEguiOpen::Toggle => res.open = !res.open,
        IosGamecenterEguiOpen::Open => res.open = true,
        IosGamecenterEguiOpen::Close => res.open = false,
    }
}

fn process_gc_events(
    mut events: EventReader<IosGamecenterEvents>,
    mut res: ResMut<DebugIosGamecenter>,
) {
    for e in events.read() {
        match e {
            IosGamecenterEvents::SaveGames((_, IosGCSaveGamesResponse::Done(items))) => {
                res.events.push(format!("SaveGames: {}", items.0.len()));
                res.saves.clone_from(&items.0);
            }
            IosGamecenterEvents::Player((_, p)) => res.player = p.clone(),
            IosGamecenterEvents::ConflictingSaveGames(items) => {
                res.events.push(format!(
                    "Conflicting: '{}' [{}]",
                    items.0.first().unwrap().name,
                    items.0.len(),
                ));

                res.conflicting = items.0.clone();
            }
            _ => res.events.push(format!("{:?}", e)),
        }
    }
}

fn update(
    mut contexts: EguiContexts,
    mut res: ResMut<DebugUiResource>,
    mut res_gc: ResMut<DebugIosGamecenter>,
) {
    let mut open_state = res.open;
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("bevy_ios_gamecenter")
        .default_pos((0., 200.))
        .open(&mut open_state)
        .show(ctx, |ui| {
            ios_gamecenter_ui(ui, &mut res_gc);
        });

    res.open = open_state;
}

fn ios_gamecenter_ui(ui: &mut egui::Ui, res: &mut ResMut<DebugIosGamecenter>) {
    ui.collapsing("player", |ui| {
        ui.label(format!("is_authenticated: {}", res.player.is_authenticated));
        ui.label(format!("display_name: {}", res.player.display_name));
        ui.label(format!("id: {}", res.player.game_id));

        if ui.button("auth").clicked() {
            bevy_ios_gamecenter::authenticate(-1);
        }

        if ui.button("player").clicked() {
            bevy_ios_gamecenter::request_player(-1);
        }

        if ui.button("fetch sig").clicked() {
            bevy_ios_gamecenter::fetch_signature(-1);
        }
    });

    ui.collapsing("leaderboards", |ui| {
        if ui.button("show ui").clicked() {
            bevy_ios_gamecenter::trigger_view(view_states::LEADERBOARDS);
        }
        for id in &res.test_ranking_ids {
            ui.collapsing(id, |ui| {
                if ui.button("submit score = 1").clicked() {
                    bevy_ios_gamecenter::leaderboards_score(-1, id.clone(), 1, 2);
                }
                if ui.button("submit score = 100").clicked() {
                    bevy_ios_gamecenter::leaderboards_score(-1, id.clone(), 100, 2);
                }
            });
        }
    });

    ui.collapsing("achievements", |ui| {
        if ui.button("show ui").clicked() {
            bevy_ios_gamecenter::trigger_view(view_states::ACHIEVEMENTS);
        }

        if ui.button("achievement: reset").clicked() {
            bevy_ios_gamecenter::achievements_reset(-1);
        }

        if ui.button("achievement: invalid id").clicked() {
            bevy_ios_gamecenter::achievement_progress(-1, "totally-invalid-id".into(), 100.0);
        }

        for id in &res.test_achievement_ids {
            ui.collapsing(id, |ui| {
                if ui.button("set to 50%").clicked() {
                    bevy_ios_gamecenter::achievement_progress(-1, id.clone(), 50.);
                }

                if ui.button("set to 100%").clicked() {
                    bevy_ios_gamecenter::achievement_progress(-1, id.clone(), 100.);
                }
            });
        }
    });

    ui.collapsing("saves", |ui| {
        ui.label(format!("savegames: {}", res.saves.len()));

        if ui.button("test save").clicked() {
            bevy_ios_gamecenter::save_game(-1, "test".into(), vec![1, 2, 3].as_slice());
        }

        if ui.button("delete invalid").clicked() {
            bevy_ios_gamecenter::delete_savegame(-1, "invalid".into());
        }

        if ui.button("fetch saves").clicked() {
            res.saves.clear();
            bevy_ios_gamecenter::fetch_save_games(-1);
        }

        ui.label(format!("conflicting: {}", res.conflicting.len()));

        if ui.button("resolve conflicts").clicked() {
            bevy_ios_gamecenter::resolve_conflicting_games(
                -1,
                IosGCSaveGames(res.conflicting.clone()),
                vec![3, 2, 1].as_slice(),
            );
            res.conflicting.clear();
        }

        TableBuilder::new(ui)
            .column(Column::auto())
            .column(Column::initial(50.).resizable(true))
            .column(Column::initial(150.).resizable(true))
            .column(Column::auto())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("cmd");
                });
                header.col(|ui| {
                    ui.heading("name");
                });
                header.col(|ui| {
                    ui.heading("device");
                });
                header.col(|ui| {
                    ui.heading("time");
                });
            })
            .body(|mut body| {
                for p in &res.saves {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            if ui.button("get").clicked() {
                                bevy_ios_gamecenter::load_game(-1, p.clone());
                            }
                            if ui.button("-").clicked() {
                                bevy_ios_gamecenter::delete_savegame(-1, p.name.clone());
                            }
                        });
                        row.col(|ui| {
                            ui.label(p.name.clone());
                        });
                        row.col(|ui| {
                            ui.label(p.device_name.clone());
                        });
                        row.col(|ui| {
                            ui.label(p.modification_date.to_string());
                        });
                    });
                }
            });
    });

    ui.collapsing("events", |ui| {
        ui.label(format!("received: {}", res.events.len()));
        if ui.button("clear").clicked() {
            res.events.clear();
        }

        let mut string = res.events.join("\n");
        ui.text_edit_multiline(&mut string);
    });
}
