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

public func save_game(data: RustString, name: RustString) {
    Task{
        do {
            let gameData = try Data.init(base64Encoded:data.toString())!
            try await GKLocalPlayer.local.saveGameData(gameData, withName: name.toString())
            let games = try await GKLocalPlayer.local.fetchSavedGames()
            print("Fetched: \(games).")
        } catch {
            print("Error: \(error.localizedDescription).")
        }
    }
}

public func load_game(name: RustString) {
    
    Task{
        do {
            let games = try await GKLocalPlayer.local.fetchSavedGames()
            
            for game in games {
                if game.name == name.toString() {
                    var data = try await game.loadData()
                                        
                    let result = data.base64EncodedString()

                    receive_load_game(result)
                }
            }
        } catch {
            print("Error: \(error.localizedDescription).")
        }
    }
}
