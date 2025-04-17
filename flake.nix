{
  description = "mörderspiel dev flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-compat.follows = "";
      };
    };
  };

  outputs =
    {
      self,
      fenix,
      git-hooks,
      nixpkgs,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlays.default ];
      };
      # using the stable toolchain
      rust-toolchain = fenix.packages.${system}.stable;
    in
    {
      checks.${system}.pre-commit-hooks = git-hooks.lib.${system}.run {
        hooks = {
          end-of-file-fixer.enable = true;
          nixfmt-rfc-style.enable = true;
          rustfmt = {
            enable = true;
            packageOverrides = {
              inherit (rust-toolchain) cargo rustfmt;
            };
          };
          trim-trailing-whitespace.enable = true;
        };
        src = ./.;
      };
      devShells.${system}.default = pkgs.mkShell {
        # git hooks
        inherit (self.checks.${system}.pre-commit-hooks) shellHook;
        inputsFrom = self.checks.${system}.pre-commit-hooks.enabledPackages;

        nativeBuildInputs = with pkgs; [
          # improve nested shells
          bashInteractive
          pkg-config
        ];

        buildInputs = with pkgs; [
          (rust-toolchain.withComponents [
            "cargo"
            "clippy"
            "rust-analyzer"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
          openssl
        ];
        # env vars
        RUST_SRC_PATH = "${rust-toolchain.rust-src}/lib/rustlib/src/rust/library";
      };
      formatter.${system} = pkgs.nixfmt-rfc-style;
    };
}
