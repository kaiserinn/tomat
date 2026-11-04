{
  inputs.nixpkgs.url = "nixpkgs/nixos-26.05";

  # The fonts are gitignored, so the flake source never contains them
  inputs.fonts = {
    url = "path:/home/azhar/dev/tomat/assets/fonts";
    flake = false;
  };

  outputs = {
    self,
    nixpkgs,
    fonts,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    src = pkgs.runCommand "tomat-src" {} ''
      cp -r ${./.} $out
      chmod -R u+w $out
      cp -r ${fonts} $out/assets/fonts
    '';
    libPath = with pkgs;
      lib.makeLibraryPath [
        libxkbcommon

        # Shit won't run without this
        wayland

        # For using wgpu
        vulkan-loader

        # For audio (see https://github.com/RustAudio/rodio#dependencies-linux-only)
        alsa-lib
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

        # For audio (see https://github.com/RustAudio/rodio#dependencies-linux-only)
        alsa-lib
      ];

      LD_LIBRARY_PATH = libPath;
    };

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "tomat";
      version = "0.1.0";

      src = src;

      cargoHash = "sha256-6+9BY4Ftxiuy5g+Eh7+ZuUXhvGaxcx+8M0joFnFmGaQ=";

      nativeBuildInputs = with pkgs; [
        pkg-config
        clang
        mold
        makeWrapper
      ];

      buildInputs = with pkgs; [
        alsa-lib
        libxkbcommon
        wayland
        vulkan-loader
      ];

      postFixup = ''
        wrapProgram $out/bin/tomat --suffix LD_LIBRARY_PATH : ${libPath}
      '';

      meta.mainProgram = "tomat";
    };
  };
}
