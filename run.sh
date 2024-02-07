#!/usr/bin/env bash

set -eou pipefail

flatpak-builder --user --install --force-clean build-dir com.github.gtkrs.BackgroundDemo.yml
flatpak run com.github.gtkrs.BackgroundDemo


