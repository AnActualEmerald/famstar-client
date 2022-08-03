bundles := "./src-tauri/target/release/bundle"

#Build syncer binary and move it to the correct folder
build-sync:
    -mkdir ./src-tauri/bin
    deno compile -A --allow-read --allow-write --allow-env --allow-net ./famstar-client-sync/app.ts
    mv app ./src-tauri/bin/
    deno run --allow-read --allow-write rename_bin.js

#Build release packages
build: build-sync
    npm run build
    -cargo tauri build -b deb
    -rm -rf ./bundles
    -mkdir ./bundles
    cp -r {{bundles}}/* ./bundles
    
#Run tauri dev, frontend dev server should be started separately
dev: build-sync
    cargo tauri dev