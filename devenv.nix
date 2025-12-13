{ pkgs, lib, config, inputs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-unknown-unknown" ];
  };
  
  packages = with pkgs; [ sea-orm-cli dioxus-cli ];

  env.DATABASE_URL = "sqlite://db.sqlite?mode=rwc";
}
