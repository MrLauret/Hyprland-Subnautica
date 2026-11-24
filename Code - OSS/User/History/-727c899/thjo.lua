--------------------------------
---- WINDOWS AND WORKSPACES ----
--------------------------------

-- See https://wiki.hypr.land/Configuring/Basics/Window-Rules/
-- and https://wiki.hypr.land/Configuring/Basics/Workspace-Rules/

-- Example window rules that are useful

local suppressMaximizeRule = hl.window_rule({
    -- Ignore maximize requests from all apps. You'll probably like this.
    name  = "suppress-maximize-events",
    match = { class = ".*" },

    suppress_event = "maximize",
})
-- suppressMaximizeRule:set_enabled(false)

hl.window_rule({
    -- Fix some dragging issues with XWayland
    name  = "fix-xwayland-drags",
    match = {
        class      = "^$",
        title      = "^$",
        xwayland   = true,
        float      = true,
        fullscreen = false,
        pin        = false,
    },

    no_focus = true,
})

hl.window_rule({
    name  = "move-hyprland-run",
    match = { class = "hyprland-run" },
    move  = "20 monitor_h-120",
    float = true,
})

hl.window_rule({
    name = "discord",
    match = { class = "discord" },
    workspace = "3 silent"
})

hl.window_rule({
    name = "volume",
    match = { class = "org.pulseaudio.pavucontrol" },
    float = true,
    move = {"1400", "100"}
})

hl.window_rule({
    name = "zen",
    match = { class = "app.zen_browser.zen" },
    workspace = "1 silent"
})

hl.window_rule({
    name = "dolphin",
    match = { class = "org.kde.dolphin" },
    float = true
})

hl.window_rule({
    name = "steam",
    match = { class = "steam" },
    workspace = "2 silent"
})