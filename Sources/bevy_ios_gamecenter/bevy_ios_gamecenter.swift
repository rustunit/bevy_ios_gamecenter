import BevyIosGamecenterRust 
public func authentication(_ result: IosGCAuthResult) {
    __swift_bridge__$authentication({result.isOwned = false; return result.ptr;}())
}
public func receive_player(_ p: IosGCPlayer) {
    __swift_bridge__$receive_player({p.isOwned = false; return p.ptr;}())
}
@_cdecl("__swift_bridge__$ios_gc_init")
public func __swift_bridge__ios_gc_init () {
    ios_gc_init()
}

@_cdecl("__swift_bridge__$get_player")
public func __swift_bridge__get_player () {
    get_player()
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



