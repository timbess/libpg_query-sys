let

moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
stableRust = nixpkgs.latest.rustChannels.stable.rust;
stableCargo = nixpkgs.latest.rustChannels.stable.cargo;
buildRustPackage = nixpkgs.callPackage (import <nixpkgs/pkgs/build-support/rust>) {
  rust = {
    rustc = stableRust;
    cargo = stableCargo;
  };
};

derivation = { stdenv, rustPlatform, buildRustPackage, llvmPackages_39, rust-bindgen, binutils, gnumake, ... }:
  buildRustPackage rec {
  name = "libpg_query-sys";
  version = "0.1.0";

  src = ./.;

  libclang = llvmPackages_39.libclang.lib;
  clang = llvmPackages_39.clang;

  buildInputs = [ libclang rust-bindgen gnumake binutils ];
  propagatedBuildInputs = [ clang ];

  configurePhase = ''
    export LIBCLANG_PATH=${libclang}/lib
  '';

  cargoSha256 = "0g87vcvaih4g3i2n6ac0xaszdf774k8n1yw5hn64mcpi3dwxsz0n";

  meta = with stdenv.lib; {
    description = "C library for accessing the PostgreSQL parser outside of the server.";
    homepage = https://github.com/lfittl/libpg_query;
    license = licenses.bsd3;
    maintainers = with maintainers; [ lfittl ];
  };

  shellHook = ''
    export LIBCLANG_PATH=${libclang}/lib
  '';
};

in

  nixpkgs.callPackage derivation {
    inherit buildRustPackage;
    rust = stableRust;
    cargo = stableCargo;
  }
