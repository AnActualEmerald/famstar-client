let extension = ''
if (Deno.build.os === 'windows') {
  extension = '.exe'
}

async function main() {
  Deno.renameSync(
    `src-tauri/bin/app${extension}`,
    `src-tauri/bin/syncer-${Deno.build.target}${extension}`
  )
}

main().catch((e) => {
  throw e
})