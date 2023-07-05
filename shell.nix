# Development shell for this project.

{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = [ pkgs.rustc 
                          pkgs.cargo
                          pkgs.rustfmt 
                          pkgs.clippy 
                        ];
}
