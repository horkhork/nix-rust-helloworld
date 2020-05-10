# nix/rust.nix
{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = "2020-05-05";
  targets = [ ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in chan
