{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ] ++
  /* add some tim apple shit here because nix on darwin is an unholy abomination */
  (with darwin.apple_sdk.frameworks; [ AppKit QuartzCore ]);
}
