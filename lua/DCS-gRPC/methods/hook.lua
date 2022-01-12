--
-- Hook actions
-- Docs: /DCS World/API/DCS_ControlAPI.html
--

local GRPC = GRPC

GRPC.methods.getMissionName = function()
  return GRPC.success({name = DCS.getMissionName()})
end

GRPC.methods.getPlayerInfo = function(params)
  return GRPC.success(net.get_player_info(params.playerID))
end

GRPC.methods.getPlayerInfoByPlayerName = function(params)
  net.log("getPlayerInfoByPlayerName " .. params.name)

  local players = net.get_player_list()
  for _,playerID in pairs(players) do
    local _playerDetails = net.get_player_info(playerID)

    if _playerDetails.name == params.name then
      return GRPC.success({
        id = _playerDetails.id,
        name = _playerDetails.name,
        side = _playerDetails.side,
        ping = _playerDetails.ping,
        ipaddr = _playerDetails.ipaddr,
        ucid = _playerDetails.ucid,
        slot = _playerDetails.slot
      })
    end
  end

  return GRPC.error("Failed to find player " .. params.name)
end

GRPC.methods.hookEval = function(params)
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
