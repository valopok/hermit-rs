#!/usr/bin/env bash

# Usage:   run.sh TARGET BIN
# Example: run.sh linux server-bw
#          run.sh hermit client-bw

set -o errexit

netbench_dir="${0%/*}"
root_dir="$netbench_dir"/../..

bin=$2
args="--bytes 1048576 --rounds 1000"

hermit() {
    echo "Building $bin image"

    cargo build --manifest-path "$netbench_dir"/Cargo.toml --bin $bin \
        -Zbuild-std=core,alloc,std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
        --target x86_64-unknown-hermit \
        --release

    echo "Launching $bin image on QEMU"

    qemu-system-x86_64 -cpu host \
            -enable-kvm -display none -smp 1 -m 1G -serial stdio \
            -kernel "$root_dir"/kernel/hermit-loader-x86_64 \
            -initrd "$root_dir"/target/x86_64-unknown-hermit/release/$bin \
            -netdev tap,id=net0,ifname=tap10,script=no,downscript=no,vhost=on \
            -device virtio-net-pci,netdev=net0,disable-legacy=on \
            -append "-- --address 10.0.5.1 $args"
}

linux() {
    echo "Launching $bin on linux"

    cargo run --manifest-path "$netbench_dir"/Cargo.toml --bin $bin \
        --release \
        --target x86_64-unknown-linux-gnu \
        -- \
        --address 10.0.5.3 $args
}

$1
