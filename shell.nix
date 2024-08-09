let
    # standard nix packages
    pkgs = import <nixpkgs> {};
    # apple-specific nix pakcages, e.g. what you'd get from xcode
    apple_frameworks = pkgs.darwin.apple_sdk.frameworks;
    # unstable nix packages (for latest versions if needed)
    # requires adding the unstable channel as `unstable`:
    #   - https://gist.github.com/voidIess/59ba97e4f759c2498f81289205582e61
    unstable = import <unstable> {};
in pkgs.mkShell {

    buildInputs = [
        pkgs.rustup
        pkgs.rust-analyzer
    ];

    # stabilizes the rust path
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
