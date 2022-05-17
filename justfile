bundles := "./src-tauri/target/release/bundle"

#Build release packages
build:
    npm run build
    -cargo tauri build
    cp -r {{bundles}} ./bundles
    