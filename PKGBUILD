# Maintainer: Lucien Cartier-Tilet <lucien@phundrak.com>
pkgname=pumopm-git
pkgver=.r0.4e2acb9
pkgrel=1
pkgdesc="A tiny power manager written in Rust"
arch=('i686' 'x86_64' 'arm' 'armv6h' 'armv7h' 'aarch64')
url="https://labs.phundrak.com/phundrak/pumopm"
license=('GPL3')
depends=()
makedepends=('rustup' 'git')
options=()
source=("$pkgname::git+https://github.com/phundrak/pumopm")
md5sums=('SKIP')

pkgver() {
  cd "$pkgname"
  local tag=$(git tag --sort=-v:refname | grep '^[0-9]' | head -1)
  local commits_since=$(git rev-list $tag..HEAD --count)
  echo "$tag.r$commits_since.$(git log --pretty=format:'%h' -n 1)"
}

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
