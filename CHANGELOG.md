# Changelog

## 0.9.0 - 2024-01-13 

- `sqlite` and `postgres` features PR: `9glenda`[#10](https://github.com/taiki-e/sqlxinsert/pull/10)
- removed `eyre` dependency and using `sqlx::Result` instead.


## 0.8.0 - 2023-12-22

- Added `udpate` method
  - method expect update is based on first attribue ( usually id )

## 0.7.0 - 2023-05-07

- Updated to `sqlx 0.7`

## 0.6.0 - 2022-06-25

- Merged 0.4 and 0.5 ( forgot that there is new `insert_raw` method )

## 0.5.0 - 2022-06-25

- Updated to `sqlx 6.x`

## 0.4.0 - 2022-02-07

- Using `sqlx 0.5` and `tokio 1.16`

## 0.3.0-alpha.0 - 2020-11-18

- `SqliteInsert` `insert` method changed into `insert_raw` ( it does not return `Result<T>` but only `Result<sqlx::sqlite::SqliteDone>`
- Renamed typo `PqInsert` into correct `PgInsert`
- Not finished: generic method for `insert<T>`

## 0.2.2 - 2020-11-14

- chagned to `runtime-actix-rustls` from `tokio` because there is problem with sqlite under actix with tokio runtime.
    - [sqlx issue - Remove sqlx_rt::blocking!() and change runtime-actix](https://github.com/launchbadge/sqlx/issues/793)

## 0.2.1 - 2020-11-14

- Updated readme.md

## 0.2.0 - 2020-11-14

- Updated to `sqlx 0.4`
