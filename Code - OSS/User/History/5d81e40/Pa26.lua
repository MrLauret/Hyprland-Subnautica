--Internal--
require("hyprland.lib")
require("hyprland.services")

--Custom--
require("hyprland.env")

if is_file_exists(HOME .. "/.config/hypr/custom/keybinds.lua") then
    require("custom.keybinds")
end
if is_file_exists(HOME .. "/.config/hypr/custom/misc.lua") then
    require("custom.misc")
end
if is_file_exists(HOME .. "/.config/hypr/custom/execs.lua") then
    require("custom.execs")
end
if is_file_exists(HOME .. "/.config/hypr/custom/input.lua") then
    require("custom.input")
end
if is_file_exists(HOME .. "/.config/hypr/custom/monitors.lua") then
    require("custom.monitors")
end
if is_file_exists(HOME .. "/.config/hypr/custom/theme.lua") then
    require("custom.theme")
end
if is_file_exists(HOME .. "/.config/hypr/custom/windows.lua") then
    require("custom.windows")
end