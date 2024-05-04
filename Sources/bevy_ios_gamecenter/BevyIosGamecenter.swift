//
//  File.swift
//  
//
//  Created by Stephan on 26.04.24.
//

import GameKit
import Foundation
import BevyIosGamecenterRust

public func ios_gc_init() {
    GKLocalPlayer.local.authenticateHandler = nil
    GKLocalPlayer.local.authenticateHandler = { gcAuthVC, error in
       if GKLocalPlayer.local.isAuthenticated {
           authentication(IosGCAuthResult.authenticated())
       } else if let vc = gcAuthVC {
           UIApplication.shared.keyWindow?.rootViewController?.present(vc, animated: true)
           authentication(IosGCAuthResult.login_presented())
       }
       else {
           //TODO: more error types
           authentication(IosGCAuthResult.error(error!.localizedDescription))
       }
     }
}

public func get_player() {
    let p = GKLocalPlayer.local;
    
    receive_player(IosGCPlayer.new(p.gamePlayerID, p.teamPlayerID, p.isAuthenticated, p.alias, p.displayName))
}

fileprivate func convert_save_game(_ save: GKSavedGame) throws -> IosGCSaveGame {
    return IosGCSaveGame.new(save.name!, save.deviceName!, UInt64(try save.modificationDate!.timeIntervalSince1970))
}

public func save_game(data: RustString, name: RustString) {
    Task{
        do {
            let gameData = try Data.init(base64Encoded:data.toString())!
            let save : GKSavedGame = try await GKLocalPlayer.local.saveGameData(gameData, withName: name.toString())
            let rust_save = try convert_save_game(save)
            
            receive_saved_game(IosGCSavedGameResponse.done(rust_save))
        } catch {
            receive_saved_game(IosGCSavedGameResponse.error(error.localizedDescription))
        }
    }
}

public func load_game(save_game: IosGCSaveGame) {
    Task{
        do {
            let games = try await GKLocalPlayer.local.fetchSavedGames()
            
            for game in games {
                let rust_game = try convert_save_game(game)
                if IosGCSaveGame.equals(rust_game,save_game) {
                    var data = try await game.loadData()
                    let result = data.base64EncodedString()
                    receive_load_game(IosGCLoadGamesResponse.done(rust_game,result))
                    return
                }
            }
            
            receive_load_game(IosGCLoadGamesResponse.unknown(save_game))
        } catch {
            receive_load_game(IosGCLoadGamesResponse.error(error.localizedDescription))
        }
    }
}

public func fetch_save_games() {
    Task{
        do {
            let games = try await GKLocalPlayer.local.fetchSavedGames()
            
            var result = RustVec<IosGCSaveGame>()
            for game in games {
                result.push(value: try convert_save_game(game))
            }
            
            receive_save_games(IosGCSaveGamesResponse.done(result))
        } catch {
            receive_save_games(IosGCSaveGamesResponse.error(error.localizedDescription))
        }
    }
}

fileprivate func convert_achievement(_ a: GKAchievement) throws -> IosGCAchievement {
    return IosGCAchievement.new(a.identifier, a.percentComplete, a.isCompleted, UInt64(try a.lastReportedDate.timeIntervalSince1970))
}

public func achievement_progress(id: RustString, progress: Double){
    Task {
        do{
            var achievement = GKAchievement.init(identifier: id.toString())
            
            achievement.percentComplete = progress
            try await GKAchievement.report([achievement])
            let response = try convert_achievement(achievement)
            receive_achievement_progress(IosGCAchievementProgressResponse.done(response))
        } catch {
            receive_achievement_progress(IosGCAchievementProgressResponse.error(error.localizedDescription))
        }
    }
}

public func reset_achievements() {
    Task {
        do{
            try await GKAchievement.resetAchievements()
            receive_achievement_reset(IosGCAchievementsResetResponse.done())
        } catch {
            receive_achievement_reset(IosGCAchievementsResetResponse.error(error.localizedDescription))
        }
    }
}

