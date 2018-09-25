{ pkgs ? (import <nixpkgs> {}).pkgs }:

with pkgs;

callPackage ./derivation.nix {}
