//
//  File.swift
//  
//
//  Created by Stephan on 26.04.24.
//

import GameKit
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
