{
	description = "spillcash";

	inputs = {
		nixpkgs.url = "nixpkgs/nixos-24.05";
		flake-utils = {
			url = "github:numtide/flake-utils";
		};
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, flake-utils, rust-overlay }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				rustVersion = "1.88.0";

				isDarwin = pkgs.stdenv.hostPlatform.isDarwin;
				overlays = [ rust-overlay.overlays.default ];
				pkgs = import nixpkgs {
					inherit system overlays;
				};

				rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
					extensions = [ "rust-src" ]; #"rust-analyzer" ];
				};

			in
			{
				devShells.default = pkgs.mkShell {
					nativeBuildInput = [ ];
					buildInputs = [
						# For CI image
						pkgs.coreutils
						pkgs.which
						pkgs.git
						pkgs.gnugrep
						# For building
						pkgs.llvmPackages.clang
						pkgs.rustPlatform.bindgenHook
						rust
						pkgs.pkg-config
						# For development & testing
						pkgs.just
						pkgs.jq
					] ++ (if isDarwin then [
						pkgs.darwin.apple_sdk.frameworks.Security
						pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
					] else []);

					LIBCLANG_PATH = "${pkgs.llvmPackages.clang-unwrapped.lib}/lib/";
				};
			}
		);
}
