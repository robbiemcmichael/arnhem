{ rustPlatform, dbus, meson, pkgconfig, wayland, wayland-protocols, wlc,
  libGL, libdrm, libinput, libxkbcommon, pixman,
  mesa_noglu, xorg, libcap, ninja, llvmPackages, perlPackages,
  clang
}:

rustPlatform.buildRustPackage rec {
  name = "arnhem";
  version = "0.0.0";

  src = ./.;

  cargoSha256 = "1v7hh32wd3h2qmshf1xbcrrnc35yvvy5sskn2mjipryi2cw4jb95";

  propagatedBuildInputs = [ clang ];

  nativeBuildInputs = [ pkgconfig ];

  libclang = llvmPackages.libclang.lib;

  NIX_CFLAGS_COMPILE = "-Wno-error";

  LIBCLANG_PATH="${libclang}/lib";

  LC_ALL="en_US.UTF-8";

  buildInputs = [
    dbus
    libGL
    libcap
    libdrm
    libinput
    libxkbcommon
    llvmPackages.libclang
    mesa_noglu
    meson
    ninja
    perlPackages.X11XCB
    pixman
    wayland
    wayland-protocols
    xorg.libX11
    xorg.libxcb
    xorg.xcbutil
    xorg.xcbutilerrors
    xorg.xcbutilimage
    xorg.xcbutilwm
  ];
}
