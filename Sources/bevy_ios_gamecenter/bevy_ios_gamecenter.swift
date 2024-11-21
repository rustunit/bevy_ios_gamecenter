import BevyIosGamecenterRust 
public func receive_authentication(_ request: Int64, _ result: IosGCAuthResult) {
    __swift_bridge__$receive_authentication(request, {result.isOwned = false; return result.ptr;}())
}
public func receive_player(_ request: Int64, _ p: IosGCPlayer) {
    __swift_bridge__$receive_player(request, {p.isOwned = false; return p.ptr;}())
}
public func receive_load_game(_ request: Int64, _ response: IosGCLoadGamesResponse) {
    __swift_bridge__$receive_load_game(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_saved_game(_ request: Int64, _ response: IosGCSavedGameResponse) {
    __swift_bridge__$receive_saved_game(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_save_games(_ request: Int64, _ response: IosGCSaveGamesResponse) {
    __swift_bridge__$receive_save_games(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_deleted_game(_ request: Int64, _ response: IosGCDeleteSaveGameResponse) {
    __swift_bridge__$receive_deleted_game(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_achievement_progress(_ request: Int64, _ response: IosGCAchievementProgressResponse) {
    __swift_bridge__$receive_achievement_progress(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_achievement_reset(_ request: Int64, _ response: IosGCAchievementsResetResponse) {
    __swift_bridge__$receive_achievement_reset(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_leaderboard_score(_ request: Int64, _ response: IosGCScoreSubmitResponse) {
    __swift_bridge__$receive_leaderboard_score(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_items_for_signature_verification(_ request: Int64, _ response: IosGCFetchItemsForSignatureVerificationResponse) {
    __swift_bridge__$receive_items_for_signature_verification(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_resolved_conflicts(_ request: Int64, _ response: IosGCResolvedConflictsResponse) {
    __swift_bridge__$receive_resolved_conflicts(request, {response.isOwned = false; return response.ptr;}())
}
public func receive_conflicting_savegames(_ savegames: IosGCSaveGames) {
    __swift_bridge__$receive_conflicting_savegames({savegames.isOwned = false; return savegames.ptr;}())
}
@_cdecl("__swift_bridge__$init_listeners")
public func __swift_bridge__init_listeners () {
    init_listeners()
}

@_cdecl("__swift_bridge__$trigger_view")
public func __swift_bridge__trigger_view (_ state: Int32) {
    trigger_view(state: state)
}

@_cdecl("__swift_bridge__$authenticate")
public func __swift_bridge__authenticate (_ request: Int64) {
    authenticate(request: request)
}

@_cdecl("__swift_bridge__$get_player")
public func __swift_bridge__get_player (_ request: Int64) {
    get_player(request: request)
}

@_cdecl("__swift_bridge__$save_game")
public func __swift_bridge__save_game (_ request: Int64, _ data: UnsafeMutableRawPointer, _ name: UnsafeMutableRawPointer) {
    save_game(request: request, data: RustString(ptr: data), name: RustString(ptr: name))
}

@_cdecl("__swift_bridge__$load_game")
public func __swift_bridge__load_game (_ request: Int64, _ save_game: UnsafeMutableRawPointer) {
    load_game(request: request, save_game: IosGCSaveGame(ptr: save_game))
}

@_cdecl("__swift_bridge__$delete_game")
public func __swift_bridge__delete_game (_ request: Int64, _ name: UnsafeMutableRawPointer) {
    delete_game(request: request, name: RustString(ptr: name))
}

@_cdecl("__swift_bridge__$resolve_conflicting_games")
public func __swift_bridge__resolve_conflicting_games (_ request: Int64, _ save_games: UnsafeMutableRawPointer, _ data: UnsafeMutableRawPointer) {
    resolve_conflicting_games(request: request, save_games: IosGCSaveGames(ptr: save_games), data: RustString(ptr: data))
}

@_cdecl("__swift_bridge__$fetch_save_games")
public func __swift_bridge__fetch_save_games (_ request: Int64) {
    fetch_save_games(request: request)
}

@_cdecl("__swift_bridge__$achievement_progress")
public func __swift_bridge__achievement_progress (_ request: Int64, _ id: UnsafeMutableRawPointer, _ progress: Double) {
    achievement_progress(request: request, id: RustString(ptr: id), progress: progress)
}

@_cdecl("__swift_bridge__$reset_achievements")
public func __swift_bridge__reset_achievements (_ request: Int64) {
    reset_achievements(request: request)
}

@_cdecl("__swift_bridge__$leaderboards_score")
public func __swift_bridge__leaderboards_score (_ request: Int64, _ id: UnsafeMutableRawPointer, _ score: Int64, _ context: Int64) {
    leaderboards_score(request: request, id: RustString(ptr: id), score: score, context: context)
}

@_cdecl("__swift_bridge__$fetch_signature")
public func __swift_bridge__fetch_signature (_ request: Int64) {
    fetch_signature(request: request)
}


public class IosGCResolvedConflictsResponse: IosGCResolvedConflictsResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCResolvedConflictsResponse$_free(ptr)
        }
    }
}
extension IosGCResolvedConflictsResponse {
    class public func done(_ items: IosGCSaveGames) -> IosGCResolvedConflictsResponse {
        IosGCResolvedConflictsResponse(ptr: __swift_bridge__$IosGCResolvedConflictsResponse$done({items.isOwned = false; return items.ptr;}()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCResolvedConflictsResponse {
        IosGCResolvedConflictsResponse(ptr: __swift_bridge__$IosGCResolvedConflictsResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCResolvedConflictsResponseRefMut: IosGCResolvedConflictsResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCResolvedConflictsResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCResolvedConflictsResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCResolvedConflictsResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCResolvedConflictsResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCResolvedConflictsResponse) {
        __swift_bridge__$Vec_IosGCResolvedConflictsResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCResolvedConflictsResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCResolvedConflictsResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCResolvedConflictsResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCResolvedConflictsResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCResolvedConflictsResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCResolvedConflictsResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCResolvedConflictsResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCResolvedConflictsResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCResolvedConflictsResponseRef> {
        UnsafePointer<IosGCResolvedConflictsResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCResolvedConflictsResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCResolvedConflictsResponse$len(vecPtr)
    }
}


public class IosGCSaveGames: IosGCSaveGamesRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCSaveGames$_free(ptr)
        }
    }
}
extension IosGCSaveGames {
    class public func new(_ items: RustVec<IosGCSaveGame>) -> IosGCSaveGames {
        IosGCSaveGames(ptr: __swift_bridge__$IosGCSaveGames$new({ let val = items; val.isOwned = false; return val.ptr }()))
    }

    class public func contains(_ items: IosGCSaveGamesRef, _ item: IosGCSaveGameRef) -> Bool {
        __swift_bridge__$IosGCSaveGames$contains(items.ptr, item.ptr)
    }
}
public class IosGCSaveGamesRefMut: IosGCSaveGamesRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCSaveGamesRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCSaveGames: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCSaveGames$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCSaveGames$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCSaveGames) {
        __swift_bridge__$Vec_IosGCSaveGames$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGames$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCSaveGames(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSaveGamesRef> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGames$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSaveGamesRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSaveGamesRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGames$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSaveGamesRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCSaveGamesRef> {
        UnsafePointer<IosGCSaveGamesRef>(OpaquePointer(__swift_bridge__$Vec_IosGCSaveGames$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCSaveGames$len(vecPtr)
    }
}


public class IosGCFetchItemsForSignatureVerificationResponse: IosGCFetchItemsForSignatureVerificationResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCFetchItemsForSignatureVerificationResponse$_free(ptr)
        }
    }
}
extension IosGCFetchItemsForSignatureVerificationResponse {
    class public func done(_ items: IosGCFetchItemsForSignatureVerification) -> IosGCFetchItemsForSignatureVerificationResponse {
        IosGCFetchItemsForSignatureVerificationResponse(ptr: __swift_bridge__$IosGCFetchItemsForSignatureVerificationResponse$done({items.isOwned = false; return items.ptr;}()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCFetchItemsForSignatureVerificationResponse {
        IosGCFetchItemsForSignatureVerificationResponse(ptr: __swift_bridge__$IosGCFetchItemsForSignatureVerificationResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCFetchItemsForSignatureVerificationResponseRefMut: IosGCFetchItemsForSignatureVerificationResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCFetchItemsForSignatureVerificationResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCFetchItemsForSignatureVerificationResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCFetchItemsForSignatureVerificationResponse) {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCFetchItemsForSignatureVerificationResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCFetchItemsForSignatureVerificationResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCFetchItemsForSignatureVerificationResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCFetchItemsForSignatureVerificationResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCFetchItemsForSignatureVerificationResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCFetchItemsForSignatureVerificationResponseRef> {
        UnsafePointer<IosGCFetchItemsForSignatureVerificationResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerificationResponse$len(vecPtr)
    }
}


public class IosGCFetchItemsForSignatureVerification: IosGCFetchItemsForSignatureVerificationRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCFetchItemsForSignatureVerification$_free(ptr)
        }
    }
}
extension IosGCFetchItemsForSignatureVerification {
    class public func new<GenericIntoRustString: IntoRustString>(_ url: GenericIntoRustString, _ signature_as_base64: GenericIntoRustString, _ salt_as_base64: GenericIntoRustString, _ timestamp: UInt64) -> IosGCFetchItemsForSignatureVerification {
        IosGCFetchItemsForSignatureVerification(ptr: __swift_bridge__$IosGCFetchItemsForSignatureVerification$new({ let rustString = url.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = signature_as_base64.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = salt_as_base64.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), timestamp))
    }
}
public class IosGCFetchItemsForSignatureVerificationRefMut: IosGCFetchItemsForSignatureVerificationRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCFetchItemsForSignatureVerificationRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCFetchItemsForSignatureVerification: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCFetchItemsForSignatureVerification) {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCFetchItemsForSignatureVerification(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCFetchItemsForSignatureVerificationRef> {
        let pointer = __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCFetchItemsForSignatureVerificationRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCFetchItemsForSignatureVerificationRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCFetchItemsForSignatureVerificationRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCFetchItemsForSignatureVerificationRef> {
        UnsafePointer<IosGCFetchItemsForSignatureVerificationRef>(OpaquePointer(__swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCFetchItemsForSignatureVerification$len(vecPtr)
    }
}


public class IosGCDeleteSaveGameResponse: IosGCDeleteSaveGameResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCDeleteSaveGameResponse$_free(ptr)
        }
    }
}
extension IosGCDeleteSaveGameResponse {
    class public func done<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCDeleteSaveGameResponse {
        IosGCDeleteSaveGameResponse(ptr: __swift_bridge__$IosGCDeleteSaveGameResponse$done({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCDeleteSaveGameResponse {
        IosGCDeleteSaveGameResponse(ptr: __swift_bridge__$IosGCDeleteSaveGameResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCDeleteSaveGameResponseRefMut: IosGCDeleteSaveGameResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCDeleteSaveGameResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCDeleteSaveGameResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCDeleteSaveGameResponse) {
        __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCDeleteSaveGameResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCDeleteSaveGameResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCDeleteSaveGameResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCDeleteSaveGameResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCDeleteSaveGameResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCDeleteSaveGameResponseRef> {
        UnsafePointer<IosGCDeleteSaveGameResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCDeleteSaveGameResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCDeleteSaveGameResponse$len(vecPtr)
    }
}


public class IosGCScoreSubmitResponse: IosGCScoreSubmitResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCScoreSubmitResponse$_free(ptr)
        }
    }
}
extension IosGCScoreSubmitResponse {
    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCScoreSubmitResponse {
        IosGCScoreSubmitResponse(ptr: __swift_bridge__$IosGCScoreSubmitResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCScoreSubmitResponseRefMut: IosGCScoreSubmitResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCScoreSubmitResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCScoreSubmitResponseRef {
    class public func done() -> IosGCScoreSubmitResponse {
        IosGCScoreSubmitResponse(ptr: __swift_bridge__$IosGCScoreSubmitResponse$done())
    }
}
extension IosGCScoreSubmitResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCScoreSubmitResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCScoreSubmitResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCScoreSubmitResponse) {
        __swift_bridge__$Vec_IosGCScoreSubmitResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCScoreSubmitResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCScoreSubmitResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCScoreSubmitResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCScoreSubmitResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCScoreSubmitResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCScoreSubmitResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCScoreSubmitResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCScoreSubmitResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCScoreSubmitResponseRef> {
        UnsafePointer<IosGCScoreSubmitResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCScoreSubmitResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCScoreSubmitResponse$len(vecPtr)
    }
}


public class IosGCAchievementsResetResponse: IosGCAchievementsResetResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCAchievementsResetResponse$_free(ptr)
        }
    }
}
extension IosGCAchievementsResetResponse {
    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCAchievementsResetResponse {
        IosGCAchievementsResetResponse(ptr: __swift_bridge__$IosGCAchievementsResetResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCAchievementsResetResponseRefMut: IosGCAchievementsResetResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCAchievementsResetResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCAchievementsResetResponseRef {
    class public func done() -> IosGCAchievementsResetResponse {
        IosGCAchievementsResetResponse(ptr: __swift_bridge__$IosGCAchievementsResetResponse$done())
    }
}
extension IosGCAchievementsResetResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCAchievementsResetResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCAchievementsResetResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCAchievementsResetResponse) {
        __swift_bridge__$Vec_IosGCAchievementsResetResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCAchievementsResetResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCAchievementsResetResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAchievementsResetResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCAchievementsResetResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAchievementsResetResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAchievementsResetResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCAchievementsResetResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAchievementsResetResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCAchievementsResetResponseRef> {
        UnsafePointer<IosGCAchievementsResetResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCAchievementsResetResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCAchievementsResetResponse$len(vecPtr)
    }
}


public class IosGCAchievementProgressResponse: IosGCAchievementProgressResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCAchievementProgressResponse$_free(ptr)
        }
    }
}
extension IosGCAchievementProgressResponse {
    class public func done(_ a: IosGCAchievement) -> IosGCAchievementProgressResponse {
        IosGCAchievementProgressResponse(ptr: __swift_bridge__$IosGCAchievementProgressResponse$done({a.isOwned = false; return a.ptr;}()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCAchievementProgressResponse {
        IosGCAchievementProgressResponse(ptr: __swift_bridge__$IosGCAchievementProgressResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCAchievementProgressResponseRefMut: IosGCAchievementProgressResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCAchievementProgressResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCAchievementProgressResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCAchievementProgressResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCAchievementProgressResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCAchievementProgressResponse) {
        __swift_bridge__$Vec_IosGCAchievementProgressResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCAchievementProgressResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCAchievementProgressResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAchievementProgressResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCAchievementProgressResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAchievementProgressResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAchievementProgressResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCAchievementProgressResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAchievementProgressResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCAchievementProgressResponseRef> {
        UnsafePointer<IosGCAchievementProgressResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCAchievementProgressResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCAchievementProgressResponse$len(vecPtr)
    }
}


public class IosGCAchievement: IosGCAchievementRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCAchievement$_free(ptr)
        }
    }
}
extension IosGCAchievement {
    class public func new<GenericIntoRustString: IntoRustString>(_ identifier: GenericIntoRustString, _ progress: Double, _ is_completed: Bool, _ last_reported_date: UInt64) -> IosGCAchievement {
        IosGCAchievement(ptr: __swift_bridge__$IosGCAchievement$new({ let rustString = identifier.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), progress, is_completed, last_reported_date))
    }
}
public class IosGCAchievementRefMut: IosGCAchievementRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCAchievementRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCAchievement: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCAchievement$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCAchievement$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCAchievement) {
        __swift_bridge__$Vec_IosGCAchievement$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCAchievement$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCAchievement(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAchievementRef> {
        let pointer = __swift_bridge__$Vec_IosGCAchievement$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAchievementRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAchievementRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCAchievement$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAchievementRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCAchievementRef> {
        UnsafePointer<IosGCAchievementRef>(OpaquePointer(__swift_bridge__$Vec_IosGCAchievement$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCAchievement$len(vecPtr)
    }
}


public class IosGCLoadGamesResponse: IosGCLoadGamesResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCLoadGamesResponse$_free(ptr)
        }
    }
}
extension IosGCLoadGamesResponse {
    class public func done<GenericIntoRustString: IntoRustString>(_ save_game: IosGCSaveGame, _ data: GenericIntoRustString) -> IosGCLoadGamesResponse {
        IosGCLoadGamesResponse(ptr: __swift_bridge__$IosGCLoadGamesResponse$done({save_game.isOwned = false; return save_game.ptr;}(), { let rustString = data.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }

    class public func unknown(_ save_game: IosGCSaveGame) -> IosGCLoadGamesResponse {
        IosGCLoadGamesResponse(ptr: __swift_bridge__$IosGCLoadGamesResponse$unknown({save_game.isOwned = false; return save_game.ptr;}()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCLoadGamesResponse {
        IosGCLoadGamesResponse(ptr: __swift_bridge__$IosGCLoadGamesResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCLoadGamesResponseRefMut: IosGCLoadGamesResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCLoadGamesResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCLoadGamesResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCLoadGamesResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCLoadGamesResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCLoadGamesResponse) {
        __swift_bridge__$Vec_IosGCLoadGamesResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCLoadGamesResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCLoadGamesResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCLoadGamesResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCLoadGamesResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCLoadGamesResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCLoadGamesResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCLoadGamesResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCLoadGamesResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCLoadGamesResponseRef> {
        UnsafePointer<IosGCLoadGamesResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCLoadGamesResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCLoadGamesResponse$len(vecPtr)
    }
}


public class IosGCSaveGamesResponse: IosGCSaveGamesResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCSaveGamesResponse$_free(ptr)
        }
    }
}
extension IosGCSaveGamesResponse {
    class public func done(_ items: IosGCSaveGames) -> IosGCSaveGamesResponse {
        IosGCSaveGamesResponse(ptr: __swift_bridge__$IosGCSaveGamesResponse$done({items.isOwned = false; return items.ptr;}()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCSaveGamesResponse {
        IosGCSaveGamesResponse(ptr: __swift_bridge__$IosGCSaveGamesResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCSaveGamesResponseRefMut: IosGCSaveGamesResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCSaveGamesResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCSaveGamesResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCSaveGamesResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCSaveGamesResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCSaveGamesResponse) {
        __swift_bridge__$Vec_IosGCSaveGamesResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGamesResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCSaveGamesResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSaveGamesResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGamesResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSaveGamesResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSaveGamesResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGamesResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSaveGamesResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCSaveGamesResponseRef> {
        UnsafePointer<IosGCSaveGamesResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCSaveGamesResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCSaveGamesResponse$len(vecPtr)
    }
}


public class IosGCSavedGameResponse: IosGCSavedGameResponseRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCSavedGameResponse$_free(ptr)
        }
    }
}
extension IosGCSavedGameResponse {
    class public func done(_ save: IosGCSaveGame) -> IosGCSavedGameResponse {
        IosGCSavedGameResponse(ptr: __swift_bridge__$IosGCSavedGameResponse$done({save.isOwned = false; return save.ptr;}()))
    }

    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCSavedGameResponse {
        IosGCSavedGameResponse(ptr: __swift_bridge__$IosGCSavedGameResponse$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCSavedGameResponseRefMut: IosGCSavedGameResponseRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCSavedGameResponseRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCSavedGameResponse: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCSavedGameResponse$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCSavedGameResponse$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCSavedGameResponse) {
        __swift_bridge__$Vec_IosGCSavedGameResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCSavedGameResponse$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCSavedGameResponse(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSavedGameResponseRef> {
        let pointer = __swift_bridge__$Vec_IosGCSavedGameResponse$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSavedGameResponseRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSavedGameResponseRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCSavedGameResponse$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSavedGameResponseRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCSavedGameResponseRef> {
        UnsafePointer<IosGCSavedGameResponseRef>(OpaquePointer(__swift_bridge__$Vec_IosGCSavedGameResponse$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCSavedGameResponse$len(vecPtr)
    }
}


public class IosGCSaveGame: IosGCSaveGameRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCSaveGame$_free(ptr)
        }
    }
}
extension IosGCSaveGame {
    class public func new<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ device_name: GenericIntoRustString, _ modification_date: UInt64) -> IosGCSaveGame {
        IosGCSaveGame(ptr: __swift_bridge__$IosGCSaveGame$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = device_name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), modification_date))
    }

    class public func equals(_ a: IosGCSaveGameRef, _ b: IosGCSaveGameRef) -> Bool {
        __swift_bridge__$IosGCSaveGame$equals(a.ptr, b.ptr)
    }
}
public class IosGCSaveGameRefMut: IosGCSaveGameRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCSaveGameRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCSaveGame: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCSaveGame$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCSaveGame$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCSaveGame) {
        __swift_bridge__$Vec_IosGCSaveGame$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGame$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCSaveGame(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSaveGameRef> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGame$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSaveGameRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCSaveGameRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCSaveGame$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCSaveGameRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCSaveGameRef> {
        UnsafePointer<IosGCSaveGameRef>(OpaquePointer(__swift_bridge__$Vec_IosGCSaveGame$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCSaveGame$len(vecPtr)
    }
}


public class IosGCAuthResult: IosGCAuthResultRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCAuthResult$_free(ptr)
        }
    }
}
extension IosGCAuthResult {
    class public func error<GenericIntoRustString: IntoRustString>(_ e: GenericIntoRustString) -> IosGCAuthResult {
        IosGCAuthResult(ptr: __swift_bridge__$IosGCAuthResult$error({ let rustString = e.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCAuthResultRefMut: IosGCAuthResultRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCAuthResultRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCAuthResultRef {
    class public func authenticated() -> IosGCAuthResult {
        IosGCAuthResult(ptr: __swift_bridge__$IosGCAuthResult$authenticated())
    }

    class public func login_presented() -> IosGCAuthResult {
        IosGCAuthResult(ptr: __swift_bridge__$IosGCAuthResult$login_presented())
    }
}
extension IosGCAuthResult: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCAuthResult$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCAuthResult$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCAuthResult) {
        __swift_bridge__$Vec_IosGCAuthResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCAuthResult$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCAuthResult(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAuthResultRef> {
        let pointer = __swift_bridge__$Vec_IosGCAuthResult$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAuthResultRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCAuthResultRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCAuthResult$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCAuthResultRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCAuthResultRef> {
        UnsafePointer<IosGCAuthResultRef>(OpaquePointer(__swift_bridge__$Vec_IosGCAuthResult$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCAuthResult$len(vecPtr)
    }
}


public class IosGCPlayer: IosGCPlayerRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$IosGCPlayer$_free(ptr)
        }
    }
}
extension IosGCPlayer {
    class public func new<GenericIntoRustString: IntoRustString>(_ game_id: GenericIntoRustString, _ team_id: GenericIntoRustString, _ is_authenticated: Bool, _ alias: GenericIntoRustString, _ display_name: GenericIntoRustString) -> IosGCPlayer {
        IosGCPlayer(ptr: __swift_bridge__$IosGCPlayer$new({ let rustString = game_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = team_id.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), is_authenticated, { let rustString = alias.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = display_name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
public class IosGCPlayerRefMut: IosGCPlayerRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class IosGCPlayerRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension IosGCPlayer: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_IosGCPlayer$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_IosGCPlayer$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: IosGCPlayer) {
        __swift_bridge__$Vec_IosGCPlayer$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_IosGCPlayer$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (IosGCPlayer(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCPlayerRef> {
        let pointer = __swift_bridge__$Vec_IosGCPlayer$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCPlayerRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<IosGCPlayerRefMut> {
        let pointer = __swift_bridge__$Vec_IosGCPlayer$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return IosGCPlayerRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<IosGCPlayerRef> {
        UnsafePointer<IosGCPlayerRef>(OpaquePointer(__swift_bridge__$Vec_IosGCPlayer$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_IosGCPlayer$len(vecPtr)
    }
}



