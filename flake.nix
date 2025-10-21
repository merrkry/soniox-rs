{
  description = "An unofficial Rust binding for Soniox API";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs =
    { self, nixpkgs, ... }:
    let
      cargoLock = builtins.readFile ./Cargo.toml;
      semVer = (builtins.fromTOML cargoLock).package.version;
      lastModifiedDate = self.lastModifiedDate or self.lastModified or "19700101";
      subDate = x: y: builtins.substring x y lastModifiedDate;
      version = "${semVer}-unstable-${subDate 0 4}-${subDate 4 2}-${subDate 6 2}";
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      inherit version;
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              openssl
              pkg-config
            ];
          };
        }
      );
    };
}
