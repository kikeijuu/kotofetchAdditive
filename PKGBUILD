pkgname=kotofetch
pkgver=0.1.0
pkgrel=1
pkgdesc="A configurable terminal Japanese quote fetcher"
arch=('x86_64')
url="https://github.com/hxpe-dev/kotofetch"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')
source=("https://github.com/hxpe-dev/kotofetch/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP') # replace with real checksum or 'SKIP'

build() {
    cd "$srcdir/kotofetch-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$srcdir/kotofetch-$pkgver"
    install -Dm755 "target/release/kotofetch" "$pkgdir/usr/bin/kotofetch"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 quotes/* "$pkgdir/usr/share/kotofetch/quotes/"
}
