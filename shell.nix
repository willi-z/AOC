with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup

    # Example Build-time Additional Dependencies
    rust-analyzer
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    lld
    pkg-config
    openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}