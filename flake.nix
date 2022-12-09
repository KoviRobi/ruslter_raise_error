{
  inputs.nixpkgs.url = "nixpkgs";
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          beam.packages.erlangR24.erlang
          beam.packages.erlangR24.elixir
          beam.packages.erlangR24.elixir_ls
          rustup
          rust-analyzer
        ];
      };
    };
}
