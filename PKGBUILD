# Maintainer: Lucien Cartier-Tilet <lucien@phundrak.com>
pkgname=pumopm
pkgver=0.1.1
pkgrel=2
pkgdesc="A tiny power manager written in Rust"
arch=('i686' 'x86_64' 'arm' 'armv6h' 'armv7h' 'aarch64')
url="https://labs.phundrak.com/phundrak/pumopm"
license=('GPL3')
depends=()
makedepends=('rustup' 'git')
options=()
source=("$pkgname::https://labs.phundrak.com/phundrak/pumopm/archive/$pkgver.tar.gz")
md5sums=('347a95efacdbf9f8ab3b2da6a7eff6cc')

build() {
  cd "$pkgname"
  if command -v rustup >/dev/null 2>&1; then
    RUSTFLAGS="-C target-cpu=native" rustup run stable cargo build --release
  elif rustc --version | grep -q stable; then
    RUSTFLAGS="-C target-cpu=native" cargo build --release
  else
    cargo build --release
  fi
}

package() {
  cd "$pkgname"

  install -Dm755 "target/release/pumopm" "$pkgdir/usr/bin/pumopm"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE"
}

# vim:set ts=2 sw=2 et:
