--
-- APIs for functions that are not built-in to the DCS Mission Scripting Environment
--

GRPC.methods.requestMissionAssignment = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.joinMission = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.missionEval = function(params)
    local fn, err = loadstring(params.lua)
    if not fn then
        return GRPC.error("Failed to load Lua code: "..err)
    end

    local ok, result = pcall(fn)
    if not ok then
        return GRPC.error("Failed to execute Lua code: "..result)
    end

    return GRPC.success(result)
end

GRPC.methods.getTickets = function()
    return GRPC.success({
        red = Conquest:GetRedTickets(),
        blue = Conquest:GetBlueTickets(),
    })
end

GRPC.methods.initializeTickets = function(params)
    Conquest:Initialize(params.maxRedTickets, params.maxBlueTickets)
    return GRPC.success(nil)
end

GRPC.methods.initializeFactoryObjectives = function(params)
    SpawnFactoryObjectives(params.redState, params.blueState)
    return GRPC.success(nil)
end

GRPC.methods.initializeCapturePoint = function(params)
    InitializeCapturePoint(params.zoneName, params.zoneFriendlyName, params.coalition - 1, params.reinforced, params.redTemplates, params.blueTemplates)
    return GRPC.success(nil)
end

GRPC.methods.onZoneCaptured = function(params)
    OnZoneCaptured(params.coalition - 1, params.zoneName, params.zoneFriendlyName)
    return GRPC.success(nil)
end

GRPC.methods.isZoneEmpty = function(params)
    return GRPC.success({
        empty = Conquest:GetCapturePoint(params.zoneName):IsZoneEmpty()
    })
end

GRPC.methods.getZoneStatuses = function(params)
    local statuses = {}

    for _, c in pairs(Conquest.capturePoints) do
        table.insert(statuses, {zoneName = c.ZoneName, reinforced = c.Reinforced})
    end

    return GRPC.success({
        statuses = statuses
    })
end

GRPC.methods.getFactoryState = function()
    return GRPC.success({
        red = RedFactory:GetState(),
        blue = BlueFactory:GetState()
    })
end

GRPC.methods.sendMessageFromHQ = function(params)
    GetCCForCoalition(params.coalition - 1):MessageToCoalition(params.text)
    PlayNotificationSoundForCoalition(params.coalition - 1)
    return GRPC.success(nil)
end