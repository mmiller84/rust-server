--
-- APIs for functions that are not built-in to the DCS Mission Scripting Environment
--

GRPC.methods.requestMissionAssignment = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.joinMission = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.abortMission = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.getMissionStatus = function()
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
    local ret = {
        red = Conquest:GetRedTickets(),
        blue = Conquest:GetBlueTickets(),
    }

    -- reset the local ticket count. The grpc service is the source of truth for the ticket counts
    Conquest.redTickets = 0
    Conquest.blueTickets = 0

    return GRPC.success(ret)
end

GRPC.methods.initializeTickets = function(params)
    Conquest:Initialize(params.maxRedTickets, params.maxBlueTickets)
    return GRPC.success(nil)
end

GRPC.methods.initializeFactoryObjectives = function(params)
    SpawnFactoryObjectives(params.redState, params.blueState)
    return GRPC.success(nil)
end

GRPC.methods.initializeSkynet = function(params)
    InitializeSkynet()
    return GRPC.success(nil)
end

GRPC.methods.initializeCapturePoint = function(params)
    InitializeCapturePoint(params.zoneName, params.zoneFriendlyName, params.coalition - 1, params.reinforced, params.redTemplates, params.blueTemplates, params.staticsOnly)
    return GRPC.success(nil)
end

GRPC.methods.initializePlayerPoints = function(params)
    Points:SetBalance(params.playerName, params.points)
    return GRPC.success(nil)
end

GRPC.methods.getPlayerPoints = function(params)
    return GRPC.success({
        playerPoints = Points:GetAndResetSpentPoints()
    })
end

GRPC.methods.creditPlayerPoints = function(params)
    Points:CreditPoints(params.playerName, params.points, true)

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