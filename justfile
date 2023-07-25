bundles := "./src-tauri/target/release/bundle"
es-version := "10.2.1"

#Build syncer binary and move it to the correct folder
@update-es $version=es-version:
    echo Grabbing earthstar version v${version##v}
    curl -o src/lib/earthstar.web.js https://cdn.earthstar-project.org/js/earthstar.web.v${version##v}.js 

#Build release packages
build:
    yarn build
    -yarn tauri build -b deb
    -rm -rf ./bundles
    -mkdir ./bundles
    cp -r {{bundles}}/* ./bundles
    
#Run tauri dev, frontend dev server should be started separately
dev:
    yarn tauri dev