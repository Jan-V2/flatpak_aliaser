cargo build --features flatpak
flatpak-builder --force-clean build-dir com.jan_v2.flatpak_aliaser.yml
