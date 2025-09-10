pkgname=kotofetch
pkgver=0.1.0
pkgrel=1
pkgdesc="A configurable terminal Japanese quote fetcher"
arch=('x86_64')
url="https://github.com/hxpe-dev/kotofetch"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')

build() {
    cargo build --release
}

package() {
    install -Dm755 "target/release/kotofetch" "$pkgdir/usr/bin/kotofetch"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 quotes/* "$pkgdir/usr/share/kotofetch/quotes/"
}
