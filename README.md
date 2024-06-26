## Following Along of SQLX Tutorial With SQLite

This repo contains basic code that copies [sqlx's tutorial on a basic todo with
sqlite](https://github.com/launchbadge/sqlx/tree/main/examples/sqlite/todos). It
makes some small adjustments, adding features, etc.

To ensure that there is indeed a db though, you need to run the following from
the shell after cloning this repo:

```sh
cp env.sample .env && sqlx db create && sqlx migrate run
```

**TODO:**

- [ ] Add more fields to the Todos (i.e. timestamps, ordering features, etc.)
- [ ] Implement an HTTP server that serves back the Todos as JSON
- [ ] Implement an HTTP server that serves back the Todos as HTML
- [ ] Add unit tests
