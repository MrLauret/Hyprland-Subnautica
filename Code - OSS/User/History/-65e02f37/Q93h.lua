hl.monitor({
    output   = "",
    mode     = "1280x1024@165", --1280x1024 y 1920x1080
    position = "auto",
    scale    = "1",
    mirror   = "",
})

hl.xwayland({
    force_zero_scaling = true,
})