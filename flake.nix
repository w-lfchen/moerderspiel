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
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      allPkgs = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        }
      );
      # using the stable toolchain
      mkRustToolchain = system: fenix.packages.${system}.stable;
    in
    {
      checks = forAllSystems (system: {
        pre-commit-hooks = git-hooks.lib.${system}.run {
          hooks = {
            end-of-file-fixer.enable = true;
            nixfmt.enable = true;
            rustfmt = {
              enable = true;
              packageOverrides = {
                inherit (mkRustToolchain system) cargo rustfmt;
              };
            };
            trim-trailing-whitespace.enable = true;
          };
          src = ./.;
        };
      });
      devShells = forAllSystems (
        system:
        let
          inherit (allPkgs.${system}) pkgs;
          rust-toolchain = mkRustToolchain system;
        in
        {
          default = pkgs.mkShell {
            # git hooks
            inherit (self.checks.${system}.pre-commit-hooks) shellHook;
            inputsFrom = self.checks.${system}.pre-commit-hooks.enabledPackages;

            nativeBuildInputs = with pkgs; [
              # improve nested shells
              bashInteractive
            ];

            buildInputs = [
              (rust-toolchain.withComponents [
                "cargo"
                "clippy"
                "rust-analyzer"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
            ];
            # env vars
            RUST_SRC_PATH = "${rust-toolchain.rust-src}/lib/rustlib/src/rust/library";
          };
        }
      );
      formatter = forAllSystems (system: allPkgs.${system}.nixfmt);
    };
}
