{ rustPlatform, lib, pacman, sudo }:
rustPlatform.buildRustPackage {
  pname = "neko";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  buildInputs = [
    sudo
    pacman
  ];
}
