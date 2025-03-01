{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        naersk' = pkgs.callPackage naersk { };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            bacon
            cargo
            cargo-msrv
            clippy
            nixfmt-rfc-style
            rustc
            sqlx-cli
            openssl
            openssl.dev # Ensure development headers are available
            openssl.out # Ensure the runtime libraries are available
            pkg-config
          ];

          shellHook = ''
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
          '';
        };
        packages = {
          default = naersk'.buildPackage {
            pname = "rustoodoo";
            version = "git";

            src = ./.;
            doCheck = true; # run `cargo test` on build

            meta = with pkgs.lib; {
              description = "Simple CLI todo application";
              homepage = "https://github.com/lucasmartinsvieira/rustoooo";
              license = licenses.mit;
              # maintainers = with maintainers; [ ];
              mainProgram = "rt";
            };
          };
        };
      }
    );

}
