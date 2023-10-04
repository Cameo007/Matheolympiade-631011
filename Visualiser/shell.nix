{ pkgs ? import <nixpkgs> {} }:
let
  python-packages = ps: with ps; [
    matplotlib
    numpy
  ];
  python = pkgs.python3.withPackages python-packages;
in python.env
