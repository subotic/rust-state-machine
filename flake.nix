{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.default = pkgs.mkShell {

        shellHook = ''
            # echo "C++ includes: $(clang++ -E -x c++ - -v < /dev/null 2>&1 | grep '/include')"
            # echo | clang++ -v -E -x c++ -
            # echo | g++ -Wp,-v -x c++ - -fsyntax-only
            export PS1="\\u@\\h | nix-develop> "
        '';
        packages = with pkgs; [
          rustup
          clang
          protobuf
        ] ++ (pkgs.lib.optional stdenv.isDarwin [
          pkgs.iconv
          darwin.apple_sdk.frameworks.Foundation
        ]);

        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
      };
    });
}
