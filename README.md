# money-jar-backend
This is where the Axum server is located. 
All web services will go here.
```shell
cd money-jar-backend
cargo run
```

## Links
- [Axum](https://github.com/tokio-rs/axum)

# money-jar-core
This is where the core logic for the application is located.
Anything that has to do with interacting with the database and core business logic will go here.
```shell
cd money-jar-core
```

## Links
- [Diesel](https://diesel.rs/)

# frontend
This is where the React fronted is located that is used by the Axum server and Tauri.
```shell
cd frontend
pnpm run dev
```

## Links
- [Mockup](https://www.figma.com/design/lkQllGH9VZ4ftp7e18XcP5/Money-Jar?node-id=0-1&t=8NeJOl9vvhGBdtSs-1)
- [React](https://reactjs.org/)
- [TanStack](https://tanstack.com/router/latest/docs/framework/react/overview)

# tauri
This is where the Tauri application is located. 
Eventually this will be what builds the iOS and Android apps.
```shell
cd tauri
pnpm run tauri dev
```

## Links
- [Tauri](https://v2.tauri.app/)