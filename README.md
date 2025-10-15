# TODO

- Реализовать TTL. Написать скрипт для удаления по колонке expire
- Создать бенч
- Подключить мониторинг в графане
- Подключить метрики prometheus + grafana

## Запуск

```
PICODATA_CONNECTION_URL=postgresql://admin:T0psecret@localhost:55432 go run main.go
```

## Подключение

```
psql -h localhost -p 55432 -U admin -d T0psecre
```

## Create table
```
CREATE TABLE profiles (
    id TEXT NOT NULL,
    region TEXT NOT NULL,
    ids string not null,
    expire datetime,
    PRIMARY KEY (id))
USING memtx DISTRIBUTED BY (region)
OPTION (TIMEOUT = 3.0);

create index if not exists profiles_expire on profiles
using TREE (expire) with (HINT = true)
OPTION (TIMEOUT = 3.0);

INSERT INTO profiles (id, region, ids, expire) VALUES('id1', 'region', '1,2,3', '2025-10-15T14:19:50.047Z');
```
