{ pkgs, ... }:

pkgs.devShell.mkShell {
  name = "codeforces";
  packages = with pkgs; [
    # Toolchain required for C + Rust binaries building
    binutils
    gcc

    # Rust 1.66.0 toolchain as Codeforces supports only this version
    bacon
    cargo-flamegraph
    (rust-bin.stable."1.66.0".default.override {
      # Extensions which ease your development process
      extensions = [ "rust-analyzer" "rust-src" ];
    })
  ];
}
