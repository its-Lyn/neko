{ rustPlatform
, lib
, pacman
, makeWrapper }:
rustPlatform.buildRustPackage {
  pname = "neko";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  nativeBuildInputs = [ makeWrapper ];

  postInstall = ''
    wrapProgram $out/bin/neko \
      --prefix PATH : ${lib.makeBinPath [ pacman ]}
  '';
}
