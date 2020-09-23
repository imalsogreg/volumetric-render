{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
   buildInputs = [
    freetype expat protobuf rustfmt zlib cargo rustc pkgconfig rust-analyzer
  ] ++ stdenv.lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreText
  ];
  nativeBuildInputs = [
    cmake pkgconfig rustc cargo zlib clippy rust-analyzer
  ];

}

# stdenv.mkDerivation {
  # name = "rust-env";
  # buildInputs = [
    # freetype expat protobuf rustfmt
  # ];
  # nativeBuildInputs = [
    # cmake pkgconfig rustc cargo zlib
  # ];
#
# }
