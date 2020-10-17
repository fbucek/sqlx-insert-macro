// extern crate we're testing, same as any other code will do.
//extern crate gmacro;

use sqlx::prelude::SqliteQueryAs;

// #[derive(Default, Debug, sqlx::FromRow)]
#[derive(Default, Debug, sqlx::FromRow, sqlxinsert::SqliteInsert)]
struct Car {
    pub car_id: i32,
    pub car_name: String,
}

#[tokio::test]
async fn test_sqlite_gmacro() {
    let car = Car {
        car_id: 33,
        car_name: "Skoda".to_string(),
    };

    // bug: https://github.com/launchbadge/sqlx/issues/530
    let url = "sqlite:%3Amemory:";

    let pool = sqlx::SqlitePool::builder().build(&url).await.unwrap();

    let create_table = "create table cars (
        car_id INTEGER PRIMARY KEY,
        car_name TEXT NOT NULL
    )";
    sqlx::query(create_table)
        .execute(&pool)
        .await
        .expect("Not possible to execute");

    let res = car.insert(&pool, "cars").await.unwrap();

    assert_eq!(res, 1);

    let rows = sqlx::query_as::<_, Car>("SELECT * FROM cars")
        .fetch_all(&pool)
        .await
        .expect("Not possible to fetch");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].car_name, "Skoda");
}
