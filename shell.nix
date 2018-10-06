with import ((import <nixpkgs> {}).pkgs.fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs-channels";
  rev = "0a7e258012b60cbe530a756f09a4f2516786d370";
  sha256 = "1qcnxkqkw7bffyc17mqifcwjfqwbvn0vs0xgxnjvh9w0ssl2s036";
}) { };
stdenv.mkDerivation {
  name = "quicr";
  buildInputs = with pkgs; [ rustChannels.stable.rust pkgconfig openssl_1_1 ];
  shellHook = ''
    export CARGO_INCREMENTAL=1
    export RUST_BACKTRACE=1
  '';
}
