# UoS3 GSI Client

Note we use tauri to create a standalone binary app, which can be setup
[following these instructions](https://tauri.studio/en/docs/getting-started/intro).
Tauri is configured in the Hermit pattern, meaning it's simply used to host the
web app in it's own window. The primary web API is provided by the
`gsi-server`. 

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run tauri:serve
```

### Compiles and minifies for production
```
npm run tauri:build
```

### Lints and fixes files
```
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
