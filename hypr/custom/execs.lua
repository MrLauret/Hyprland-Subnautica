-------------------
---- AUTOSTART ----
-------------------

hl.on("hyprland.start", function () 
   hl.exec_cmd("flatpak run app.zen_browser.zen")
   hl.exec_cmd("flatpak run com.discordapp.Discord")
   hl.exec_cmd("nm-applet")
   hl.exec_cmd("hypridle")
   hl.exec_cmd("hyprpaper")
   hl.exec_cmd("waybar")
   hl.exec_once("hyprctl setcursor catppuccin-mocha-pink-cursors 24")
end)