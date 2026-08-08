//! Direct GameKit access via [`objc2`].
//!
//! Every entry point returns immediately. GameKit invokes the completion blocks on arbitrary
//! queues, so results are pushed into the `bevy_channel_message` channel and picked up by the
//! Bevy schedule from there.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy_channel_message::ChannelMessageSender;
use block2::RcBlock;
use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{AnyThread, MainThreadMarker, define_class, msg_send};
use objc2_foundation::{
    NSArray, NSData, NSDate, NSError, NSInteger, NSNumber, NSObject, NSObjectNSKeyValueCoding,
    NSObjectProtocol, NSString, NSUInteger, NSURL,
};
// `GKChallengeListener` is deprecated, but `GKLocalPlayerListener` still inherits from it.
#[allow(deprecated)]
use objc2_game_kit::GKChallengeListener;
use objc2_game_kit::{
    GKAccessPoint, GKAchievement, GKGameActivityListener, GKGameCenterViewControllerState,
    GKInviteEventListener, GKLeaderboard, GKLocalPlayer, GKLocalPlayerListener, GKPlayer,
    GKSavedGame, GKSavedGameListener, GKTurnBasedEventListener,
};
use objc2_ui_kit::{UIApplication, UISceneActivationState, UIViewController, UIWindowScene};

use crate::{
    IosGCAchievement, IosGCAchievementProgressResponse, IosGCAchievementsResetResponse,
    IosGCAuthResult, IosGCDeleteSaveGameResponse, IosGCFetchItemsForSignatureVerification,
    IosGCFetchItemsForSignatureVerificationResponse, IosGCLoadGamesResponse, IosGCPlayer,
    IosGCResolvedConflictsResponse, IosGCSaveGame, IosGCSaveGames, IosGCSaveGamesResponse,
    IosGCSavedGameResponse, IosGCScoreSubmitResponse, IosGCViewState, plugin::IosGamecenterEvents,
};

/// GameKit contract: a completion handler is called with either a result or an error. Should it
/// ever hand us neither we still owe the caller a response.
const NO_RESULT: &str = "gamecenter returned neither a result nor an error";

static SENDER: OnceLock<ChannelMessageSender<IosGamecenterEvents>> = OnceLock::new();

pub fn set_sender(sender: ChannelMessageSender<IosGamecenterEvents>) {
    let _ = SENDER.set(sender);
}

/// A callback can arrive before the plugin was built; dropping the response is the only option,
/// since unwinding out of an Objective-C block would be undefined behavior.
fn send(msg: IosGamecenterEvents) {
    let Some(sender) = SENDER.get() else {
        bevy_log::warn!("gamecenter response dropped: plugin not initialized");
        return;
    };
    sender.send(msg);
}

fn error_message(error: &NSError) -> String {
    error.localizedDescription().to_string()
}

fn convert_save_game(game: &GKSavedGame) -> IosGCSaveGame {
    // SAFETY: property reads on a `GKSavedGame` GameKit handed us.
    unsafe {
        IosGCSaveGame {
            name: game.name().map(|n| n.to_string()).unwrap_or_default(),
            device_name: game.deviceName().map(|n| n.to_string()).unwrap_or_default(),
            modification_date: game
                .modificationDate()
                .map(|d| d.timeIntervalSince1970() as u64)
                .unwrap_or_default(),
        }
    }
}

fn convert_save_games(games: &NSArray<GKSavedGame>) -> IosGCSaveGames {
    IosGCSaveGames(games.iter().map(|g| convert_save_game(&g)).collect())
}

/// `GKAchievement` forwards most of its accessors rather than implementing them, so objc2's
/// debug-only method check aborts on a direct send. KVC reaches them via `valueForKey:`, which
/// the class does implement. `isCompleted` is a real method and needs no detour.
fn achievement_value(achievement: &GKAchievement, key: &str) -> Option<Retained<AnyObject>> {
    achievement.valueForKey(&NSString::from_str(key))
}

fn convert_achievement(achievement: &GKAchievement) -> IosGCAchievement {
    IosGCAchievement {
        identifier: achievement_value(achievement, "identifier")
            .and_then(|v| v.downcast::<NSString>().ok())
            .map(|v| v.to_string())
            .unwrap_or_default(),
        progress: achievement_value(achievement, "percentComplete")
            .and_then(|v| v.downcast::<NSNumber>().ok())
            .map(|v| v.doubleValue())
            .unwrap_or_default(),
        // SAFETY: property read on an achievement we just built and reported.
        is_completed: unsafe { achievement.isCompleted() },
        last_reported_date: achievement_value(achievement, "lastReportedDate")
            .and_then(|v| v.downcast::<NSDate>().ok())
            .map(|v| v.timeIntervalSince1970() as u64)
            .unwrap_or_default(),
    }
}

/// `fetchSavedGames` is the only way into the save game store - GameKit has no API to look up a
/// single [`GKSavedGame`] by name.
fn fetch_saved_games<F>(on_result: F)
where
    F: Fn(Result<&NSArray<GKSavedGame>, String>) + 'static,
{
    let handler = RcBlock::new(
        move |games: *mut NSArray<GKSavedGame>, error: *mut NSError| {
            // SAFETY: GameKit passes valid pointers or null.
            match unsafe { (games.as_ref(), error.as_ref()) } {
                (Some(games), _) => on_result(Ok(games)),
                (None, Some(error)) => on_result(Err(error_message(error))),
                (None, None) => on_result(Err(NO_RESULT.into())),
            }
        },
    );

    // SAFETY: the block is copied by GameKit, so dropping it after this call is fine.
    unsafe { GKLocalPlayer::local().fetchSavedGamesWithCompletionHandler(Some(&handler)) };
}

// Own module so the deprecation of `GKChallengeListener` can be allowed for the whole class -
// `define_class!` drops `#[allow]` on the individual protocol impls.
mod listener {
    #![allow(deprecated)]

    use super::*;

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = AnyThread]
        #[name = "BevyIosGamecenterSavedGameListener"]
        pub struct SavedGameListener;

        unsafe impl NSObjectProtocol for SavedGameListener {}

        unsafe impl GKSavedGameListener for SavedGameListener {
            #[unsafe(method(player:hasConflictingSavedGames:))]
            fn player_has_conflicting_saved_games(
                &self,
                _player: &GKPlayer,
                saved_games: &NSArray<GKSavedGame>,
            ) {
                send(IosGamecenterEvents::ConflictingSaveGames(
                    convert_save_games(saved_games),
                ));
            }
        }

        // `GKLocalPlayerListener` inherits from these; all of their methods are optional.
        unsafe impl GKChallengeListener for SavedGameListener {}
        unsafe impl GKGameActivityListener for SavedGameListener {}
        unsafe impl GKInviteEventListener for SavedGameListener {}
        unsafe impl GKTurnBasedEventListener for SavedGameListener {}
        unsafe impl GKLocalPlayerListener for SavedGameListener {}
    );
}

use listener::SavedGameListener;

/// Registering the same listener twice is undefined behavior per GameKit's docs.
static LISTENER_REGISTERED: AtomicBool = AtomicBool::new(false);

pub fn init_listeners() {
    if LISTENER_REGISTERED.swap(true, Ordering::SeqCst) {
        return;
    }

    // SAFETY: standard alloc/init pair on a class without ivars; the listener conforms to
    // `GKLocalPlayerListener`.
    unsafe {
        let listener: Retained<SavedGameListener> = msg_send![SavedGameListener::alloc(), init];
        GKLocalPlayer::local().registerListener(ProtocolObject::from_ref(&*listener));
        // GameKit does not keep the listener alive; it has to outlive the app like the plugin does.
        std::mem::forget(listener);
    }
}

/// `UIApplication::keyWindow` is deprecated, so walk the connected scenes instead.
fn root_view_controller(mtm: MainThreadMarker) -> Option<Retained<UIViewController>> {
    let mut background = None;

    for scene in UIApplication::sharedApplication(mtm)
        .connectedScenes()
        .iter()
    {
        let Some(scene) = scene.downcast_ref::<UIWindowScene>() else {
            continue;
        };

        let root = scene
            .keyWindow()
            .and_then(|window| window.rootViewController())
            .or_else(|| {
                scene
                    .windows()
                    .iter()
                    .find_map(|window| window.rootViewController())
            });

        let Some(root) = root else { continue };

        if scene.activationState() == UISceneActivationState::ForegroundActive {
            return Some(root);
        }
        background.get_or_insert(root);
    }

    background
}

fn present(view_controller: &UIViewController) -> bool {
    let Some(mtm) = MainThreadMarker::new() else {
        bevy_log::error!("gamecenter: authentication handler ran off the main thread");
        return false;
    };
    let Some(root) = root_view_controller(mtm) else {
        bevy_log::error!("gamecenter: found no root view controller to present the login on");
        return false;
    };

    root.presentViewController_animated_completion(view_controller, true, None);
    true
}

pub fn authenticate(request: i64) {
    // Installing the handler can present UI, so it belongs on the main thread.
    DispatchQueue::main().exec_async(move || {
        let player = unsafe { GKLocalPlayer::local() };

        // Re-assigning an existing `authenticateHandler` crashes on iOS < 17.2.
        if unsafe { player.isAuthenticated() } {
            send(IosGamecenterEvents::Authentication((
                request,
                IosGCAuthResult::authenticated(),
            )));
            return;
        }

        let handler = RcBlock::new(
            move |view_controller: *mut UIViewController, error: *mut NSError| {
                // SAFETY: GameKit passes valid pointers or null.
                let (view_controller, error) =
                    unsafe { (view_controller.as_ref(), error.as_ref()) };

                let result = if unsafe { GKLocalPlayer::local().isAuthenticated() } {
                    IosGCAuthResult::authenticated()
                } else if let Some(view_controller) = view_controller {
                    if present(view_controller) {
                        IosGCAuthResult::login_presented()
                    } else {
                        IosGCAuthResult::error("could not present the login".into())
                    }
                } else if let Some(error) = error {
                    IosGCAuthResult::error(error_message(error))
                } else {
                    IosGCAuthResult::error(NO_RESULT.into())
                };

                send(IosGamecenterEvents::Authentication((request, result)));
            },
        );

        // SAFETY: objc2-game-kit only binds the macOS/`NSViewController` flavor of this property,
        // so the iOS selector is sent by hand. The property is declared `copy`.
        unsafe {
            let _: () = msg_send![&*player, setAuthenticateHandler: &*handler];
        }
    });
}

/// GameKit hands back nil for the player strings until the local player is authenticated, while
/// objc2 declares them non-null and panics on nil - so ask for them as optionals.
macro_rules! player_string {
    ($player:expr, $selector:ident) => {{
        let value: Option<Retained<NSString>> = unsafe { msg_send![$player, $selector] };
        value.map(|v| v.to_string()).unwrap_or_default()
    }};
}

pub fn get_player(request: i64) {
    let player = unsafe { GKLocalPlayer::local() };

    let player = IosGCPlayer {
        game_id: player_string!(&*player, gamePlayerID),
        team_id: player_string!(&*player, teamPlayerID),
        // SAFETY: property read on the local player singleton.
        is_authenticated: unsafe { player.isAuthenticated() },
        alias: player_string!(&*player, alias),
        display_name: player_string!(&*player, displayName),
    };

    send(IosGamecenterEvents::Player((request, player)));
}

pub fn save_game(request: i64, name: String, data: &[u8]) {
    let handler = RcBlock::new(move |game: *mut GKSavedGame, error: *mut NSError| {
        // SAFETY: GameKit passes valid pointers or null.
        let response = match unsafe { (game.as_ref(), error.as_ref()) } {
            (Some(game), _) => IosGCSavedGameResponse::done(convert_save_game(game)),
            (None, Some(error)) => IosGCSavedGameResponse::error(error_message(error)),
            (None, None) => IosGCSavedGameResponse::error(NO_RESULT.into()),
        };
        send(IosGamecenterEvents::SavedGame((request, response)));
    });

    // SAFETY: the block is copied by GameKit.
    unsafe {
        GKLocalPlayer::local().saveGameData_withName_completionHandler(
            &NSData::with_bytes(data),
            &NSString::from_str(&name),
            Some(&handler),
        );
    }
}

pub fn fetch_save_games(request: i64) {
    fetch_saved_games(move |result| {
        let response = match result {
            Ok(games) => IosGCSaveGamesResponse::done(convert_save_games(games)),
            Err(error) => IosGCSaveGamesResponse::error(error),
        };
        send(IosGamecenterEvents::SaveGames((request, response)));
    });
}

pub fn load_game(request: i64, save_game: IosGCSaveGame) {
    fetch_saved_games(move |result| {
        let games = match result {
            Ok(games) => games,
            Err(error) => {
                send(IosGamecenterEvents::LoadGame((
                    request,
                    IosGCLoadGamesResponse::error(error),
                )));
                return;
            }
        };

        let Some(game) = games
            .iter()
            .find(|game| convert_save_game(game) == save_game)
        else {
            send(IosGamecenterEvents::LoadGame((
                request,
                IosGCLoadGamesResponse::unknown(save_game.clone()),
            )));
            return;
        };

        let found = convert_save_game(&game);
        let handler = RcBlock::new(move |data: *mut NSData, error: *mut NSError| {
            // SAFETY: GameKit passes valid pointers or null.
            let response = match unsafe { (data.as_ref(), error.as_ref()) } {
                (Some(data), _) => IosGCLoadGamesResponse::done(found.clone(), data.to_vec()),
                (None, Some(error)) => IosGCLoadGamesResponse::error(error_message(error)),
                (None, None) => IosGCLoadGamesResponse::error(NO_RESULT.into()),
            };
            send(IosGamecenterEvents::LoadGame((request, response)));
        });

        // SAFETY: the block is copied by GameKit.
        unsafe { game.loadDataWithCompletionHandler(Some(&handler)) };
    });
}

pub fn delete_game(request: i64, name: String) {
    let deleted = name.clone();
    let handler = RcBlock::new(move |error: *mut NSError| {
        // SAFETY: GameKit passes a valid pointer or null.
        let response = match unsafe { error.as_ref() } {
            Some(error) => IosGCDeleteSaveGameResponse::error(error_message(error)),
            None => IosGCDeleteSaveGameResponse::done(deleted.clone()),
        };
        send(IosGamecenterEvents::DeletedSaveGame((request, response)));
    });

    // SAFETY: the block is copied by GameKit.
    unsafe {
        GKLocalPlayer::local()
            .deleteSavedGamesWithName_completionHandler(&NSString::from_str(&name), Some(&handler));
    }
}

pub fn resolve_conflicting_games(request: i64, save_games: IosGCSaveGames, data: &[u8]) {
    let data = data.to_vec();

    fetch_saved_games(move |result| {
        let games = match result {
            Ok(games) => games,
            Err(error) => {
                send(IosGamecenterEvents::ResolvedConflicts((
                    request,
                    IosGCResolvedConflictsResponse::error(error),
                )));
                return;
            }
        };

        let conflicting: Vec<_> = games
            .iter()
            .filter(|game| save_games.0.contains(&convert_save_game(game)))
            .collect();

        let handler = RcBlock::new(
            move |games: *mut NSArray<GKSavedGame>, error: *mut NSError| {
                // SAFETY: GameKit passes valid pointers or null.
                let response = match unsafe { (games.as_ref(), error.as_ref()) } {
                    (Some(games), _) => {
                        IosGCResolvedConflictsResponse::done(convert_save_games(games))
                    }
                    (None, Some(error)) => {
                        IosGCResolvedConflictsResponse::error(error_message(error))
                    }
                    (None, None) => IosGCResolvedConflictsResponse::error(NO_RESULT.into()),
                };
                send(IosGamecenterEvents::ResolvedConflicts((request, response)));
            },
        );

        // SAFETY: the block is copied by GameKit.
        unsafe {
            GKLocalPlayer::local().resolveConflictingSavedGames_withData_completionHandler(
                &NSArray::from_retained_slice(&conflicting),
                &NSData::with_bytes(&data),
                Some(&handler),
            );
        }
    });
}

pub fn fetch_signature(request: i64) {
    let handler = RcBlock::new(
        move |url: *mut NSURL,
              signature: *mut NSData,
              salt: *mut NSData,
              timestamp: u64,
              error: *mut NSError| {
            // SAFETY: GameKit passes valid pointers or null.
            let (url, signature, salt, error) = unsafe {
                (
                    url.as_ref(),
                    signature.as_ref(),
                    salt.as_ref(),
                    error.as_ref(),
                )
            };

            let response = if let Some(error) = error {
                IosGCFetchItemsForSignatureVerificationResponse::error(error_message(error))
            } else {
                IosGCFetchItemsForSignatureVerificationResponse::done(
                    IosGCFetchItemsForSignatureVerification {
                        url: url
                            .and_then(|url| url.absoluteString())
                            .map(|url| url.to_string())
                            .unwrap_or_default(),
                        signature: signature.map(NSData::to_vec).unwrap_or_default(),
                        salt: salt.map(NSData::to_vec).unwrap_or_default(),
                        timestamp,
                    },
                )
            };

            send(IosGamecenterEvents::ItemsForSignatureVerification((
                request, response,
            )));
        },
    );

    // SAFETY: the block is copied by GameKit.
    unsafe { GKLocalPlayer::local().fetchItemsForIdentityVerificationSignature(Some(&handler)) };
}

pub fn achievement_progress(request: i64, id: String, progress: f64) {
    // SAFETY: standard alloc/init pair.
    let achievement = unsafe {
        GKAchievement::initWithIdentifier(GKAchievement::alloc(), &NSString::from_str(&id))
    };
    // see `achievement_value`: the setter is forwarded too, so it goes through KVC as well
    unsafe {
        achievement.setValue_forKey(
            Some(&NSNumber::numberWithDouble(progress)),
            &NSString::from_str("percentComplete"),
        );
    }

    let reported = achievement.clone();
    let handler = RcBlock::new(move |error: *mut NSError| {
        // SAFETY: GameKit passes a valid pointer or null.
        let response = match unsafe { error.as_ref() } {
            Some(error) => IosGCAchievementProgressResponse::error(error_message(error)),
            None => IosGCAchievementProgressResponse::done(convert_achievement(&reported)),
        };
        send(IosGamecenterEvents::AchievementProgress((
            request, response,
        )));
    });

    // SAFETY: the block is copied by GameKit.
    unsafe {
        GKAchievement::reportAchievements_withCompletionHandler(
            &NSArray::from_retained_slice(&[achievement]),
            Some(&handler),
        );
    }
}

pub fn reset_achievements(request: i64) {
    let handler = RcBlock::new(move |error: *mut NSError| {
        // SAFETY: GameKit passes a valid pointer or null.
        let response = match unsafe { error.as_ref() } {
            Some(error) => IosGCAchievementsResetResponse::error(error_message(error)),
            None => IosGCAchievementsResetResponse::done(),
        };
        send(IosGamecenterEvents::AchievementsReset((request, response)));
    });

    // SAFETY: the block is copied by GameKit.
    unsafe { GKAchievement::resetAchievementsWithCompletionHandler(Some(&handler)) };
}

pub fn leaderboards_score(request: i64, id: String, score: i64, context: i64) {
    let handler = RcBlock::new(move |error: *mut NSError| {
        // SAFETY: GameKit passes a valid pointer or null.
        let response = match unsafe { error.as_ref() } {
            Some(error) => IosGCScoreSubmitResponse::error(error_message(error)),
            None => IosGCScoreSubmitResponse::done(),
        };
        send(IosGamecenterEvents::LeaderboardScoreSubmitted((
            request, response,
        )));
    });

    let player = unsafe { GKLocalPlayer::local() };

    // SAFETY: the block is copied by GameKit.
    unsafe {
        GKLeaderboard::submitScore_context_player_leaderboardIDs_completionHandler(
            score as NSInteger,
            context as NSUInteger,
            &player,
            &NSArray::from_retained_slice(&[NSString::from_str(&id)]),
            &handler,
        );
    }
}

pub fn trigger_view(state: IosGCViewState) {
    // `GKAccessPoint` presents the Game Center dashboard, so this has to run on the main thread.
    DispatchQueue::main().exec_async(move || {
        // GameKit never calls this handler, but the selector requires one.
        let handler = RcBlock::new(|| {});

        // SAFETY: the block is copied by GameKit.
        unsafe {
            GKAccessPoint::shared().triggerAccessPointWithState_handler(
                GKGameCenterViewControllerState(state as NSInteger),
                &handler,
            );
        }
    });
}
