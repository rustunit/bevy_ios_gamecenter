//
//  File.swift
//  
//
//  Created by Stephan on 26.04.24.
//

import GameKit
import Foundation
import BevyIosGamecenterRust

final class SavedGameListener: NSObject, GKLocalPlayerListener {
    
    func player(_ player: GKPlayer, hasConflictingSavedGames savedGames: [GKSavedGame]) {
        do {
            receive_conflicting_savegames(try convert_save_games(savedGames))
        } catch {
            print("Error reporting conflicting save games: \(error.localizedDescription)")
        }
    }
}

var Listener:SavedGameListener? = nil

public func init_listeners() {
    if Listener == nil {
        print("Install saved game listener")
        Listener = SavedGameListener()
        GKLocalPlayer.local.register(Listener!)
    }
}

public func authenticate(request: Int64) {
    Task {
        if GKLocalPlayer.local.isAuthenticated {
            receive_authentication(request, IosGCAuthResult.authenticated())
            return
        }
        
        GKLocalPlayer.local.authenticateHandler = { gcAuthVC, error in
            if GKLocalPlayer.local.isAuthenticated {
                receive_authentication(request, IosGCAuthResult.authenticated())
            } else if let vc = gcAuthVC {
                UIApplication.shared.keyWindow?.rootViewController?.present(vc, animated: true)
                receive_authentication(request, IosGCAuthResult.login_presented())
            }
            else {
                receive_authentication(request, IosGCAuthResult.error(error!.localizedDescription))
            }
        }
    }
}

public func get_player(request: Int64) {
    Task {
        let p = GKLocalPlayer.local;
        
        receive_player(request, IosGCPlayer.new(p.gamePlayerID, p.teamPlayerID, p.isAuthenticated, p.alias, p.displayName))
    }
}

fileprivate func convert_save_game(_ save: GKSavedGame) throws -> IosGCSaveGame {
    return IosGCSaveGame.new(save.name!, save.deviceName!, UInt64(try save.modificationDate!.timeIntervalSince1970))
}

fileprivate func convert_save_games(_ games: [GKSavedGame]) throws -> IosGCSaveGames {
    var result = RustVec<IosGCSaveGame>()
    for game in games {
        result.push(value: try convert_save_game(game))
    }
    return IosGCSaveGames.new(result)
}

public func save_game(request: Int64, data: RustString, name: RustString) {
    Task{
        do {
            let gameData = try Data.init(base64Encoded:data.toString())!
            let save : GKSavedGame = try await GKLocalPlayer.local.saveGameData(gameData, withName: name.toString())
            let rust_save = try convert_save_game(save)
            
            receive_saved_game(request, IosGCSavedGameResponse.done(rust_save))
        } catch {
            receive_saved_game(request, IosGCSavedGameResponse.error(error.localizedDescription))
        }
    }
}

public func load_game(request: Int64, save_game: IosGCSaveGame) {
    Task{
        do {
            let games = await try GKLocalPlayer.local.fetchSavedGames();
            for game in games {
                let rust_game = try convert_save_game(game)
                if IosGCSaveGame.equals(rust_game,save_game) {
                    var data = try await game.loadData()
                    let result = data.base64EncodedString()
                    receive_load_game(request, IosGCLoadGamesResponse.done(rust_game,result))
                    return
                }
            }
            
            receive_load_game(request, IosGCLoadGamesResponse.unknown(save_game))
        } catch {
            receive_load_game(request, IosGCLoadGamesResponse.error(error.localizedDescription))
        }
    }
}

public func delete_game(request: Int64, name: RustString) {
    Task{
        do {
            try await GKLocalPlayer.local.deleteSavedGames(withName: name.toString())
            receive_deleted_game(request, IosGCDeleteSaveGameResponse.done(name))
        } catch {
            receive_deleted_game(request, IosGCDeleteSaveGameResponse.error(error.localizedDescription))
        }
    }
}

public func resolve_conflicting_games(request: Int64, save_games: IosGCSaveGames, data: RustString) {
    Task{
        do {
            var found:[GKSavedGame] = []
            let games = try await GKLocalPlayer.local.fetchSavedGames()
            
            for game in games {
                let rust_game = try convert_save_game(game)
                if IosGCSaveGames.contains(save_games, rust_game) {
                    found.append(game)
                }
            }
            
            let gameData = try Data.init(base64Encoded:data.toString())!
            
            let result = try await GKLocalPlayer.local.resolveConflictingSavedGames(found, with: gameData)
            
            receive_resolved_conflicts(request, IosGCResolvedConflictsResponse.done(try convert_save_games(result)))
        } catch {
            receive_resolved_conflicts(request, IosGCResolvedConflictsResponse.error(error.localizedDescription))
        }
    }
}

public func fetch_save_games(request: Int64) {
    Task{
        do {
            let games = try await GKLocalPlayer.local.fetchSavedGames()
            
            let result = try convert_save_games(games)
            
            receive_save_games(request, IosGCSaveGamesResponse.done(result))
        } catch {
            receive_save_games(request, IosGCSaveGamesResponse.error(error.localizedDescription))
        }
    }
}

fileprivate func convert_achievement(_ a: GKAchievement) throws -> IosGCAchievement {
    return IosGCAchievement.new(a.identifier, a.percentComplete, a.isCompleted, UInt64(try a.lastReportedDate.timeIntervalSince1970))
}

public func achievement_progress(request: Int64, id: RustString, progress: Double){
    Task {
        do{
            var achievement = GKAchievement.init(identifier: id.toString())
            achievement.percentComplete = progress
            try await GKAchievement.report([achievement])
            let response = try convert_achievement(achievement)
            receive_achievement_progress(request, IosGCAchievementProgressResponse.done(response))
        } catch {
            receive_achievement_progress(request, IosGCAchievementProgressResponse.error(error.localizedDescription))
        }
    }
}

public func reset_achievements(request: Int64) {
    Task {
        do{
            try await GKAchievement.resetAchievements()
            receive_achievement_reset(request, IosGCAchievementsResetResponse.done())
        } catch {
            receive_achievement_reset(request, IosGCAchievementsResetResponse.error(error.localizedDescription))
        }
    }
}

public func leaderboards_score(request: Int64, id:RustString, score:Int64, context:Int64) {
    Task {
        do{
            try await GKLeaderboard.submitScore(Int(score), context:Int(context), player:GKLocalPlayer.local, leaderboardIDs: [id.toString()])
            receive_leaderboard_score(request, IosGCScoreSubmitResponse.done())
        } catch {
            receive_leaderboard_score(request, IosGCScoreSubmitResponse.error(error.localizedDescription))
        }
    }
}

public func trigger_view(state: Int32) {
    GKAccessPoint.shared.trigger(state: GKGameCenterViewControllerState(rawValue: Int(state)) ?? GKGameCenterViewControllerState.default){
        //TODO: no idea why this gets never called
    }
}

public func fetch_signature(request: Int64) {
    Task {
        do{
            let items = try await GKLocalPlayer.local.fetchItemsForIdentityVerificationSignature()
            
            let rust_items = IosGCFetchItemsForSignatureVerification.new(
                items.0.absoluteString,
                items.1.base64EncodedString(),
                items.2.base64EncodedString(),
                items.3)
            
            receive_items_for_signature_verification(request, IosGCFetchItemsForSignatureVerificationResponse.done(rust_items))
        } catch {
            receive_items_for_signature_verification(request, IosGCFetchItemsForSignatureVerificationResponse.error(error.localizedDescription))
        }
    }
}
