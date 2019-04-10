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

  LIBCLANG_PATH="${libclang}/lib";

  cargoSha256 = "0rrp5vgjyv02lljr06fr8zxzn1syqhfjcch7v38y4xq2p39j6znp";

  meta = with stdenv.lib; {
    description = "C library for accessing the PostgreSQL parser outside of the server.";
    homepage = https://github.com/lfittl/libpg_query;
    license = licenses.bsd3;
    maintainers = with maintainers; [ lfittl ];
  };
};

in

  nixpkgs.callPackage derivation {
    inherit buildRustPackage;
    rust = stableRust;
    cargo = stableCargo;
  }
