{
  inputs.nixpkgs.url = "nixpkgs/nixos-26.05";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    libPath = with pkgs;
      lib.makeLibraryPath [
        libxkbcommon

        # Shit won't run without this
        wayland

        # For using wgpu
        vulkan-loader
      ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        cargo
        rustc
        rust-analyzer
        clippy
        rustfmt

        # For cargo-hot (see https://github.com/iced-rs/iced/pull/3000)
        openssl
        pkg-config

        # Faster linker (NixOS by default uses ld)
        mold
        clang
      ];

      LD_LIBRARY_PATH = libPath;
    };
  };
}
