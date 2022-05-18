bundles := "./src-tauri/target/release/bundle"

build-sync:
    -mkdir ./src-tauri/bin
    deno compile --allow-read --allow-write --allow-env --allow-net ./famstar-client-sync/app.ts
    mv app ./src-tauri/bin/
    deno run --allow-read --allow-write rename_bin.js

#Build release packages
build: build-sync
    npm run build
    -cargo tauri build
    cp -r {{bundles}} ./bundles
    
dev: build-sync
    cargo tauri dev