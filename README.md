## Tricks

### Yarn + `node_modules/`

- This repository uses Yarn so that it can use a [Yarn workspace](https://yarnpkg.com/features/workspaces) to allow the webapp from importing the WASM library (which is a separate module).
- This repository also [uses `node_modules` instead of Yarn Plug and Play](https://yarnpkg.com/configuration/yarnrc#nodeLinker), so that we get Typescript static analysis to work in VS code without additional configuration.
