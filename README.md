# TODO

- Реализовать TTL. Написать скрипт для удаления по колонке expire
- Создать бенч
- Подключить мониторинг в графане
- Подключить метрики prometheus + grafana

## Запуск

```
make picodata-build
make picodata-up
make plubings-build
PICODATA_CONNECTION_URL=postgresql://admin:T0psecret@localhost:55432 go run main.go
```

## Create table

```sql
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

select * from _pico_property

select * from _pico_plugin

select * from _pico_service

select * from _pico_plugin_config

CREATE PLUGIN ttl 0.1.0;

ALTER PLUGIN ttl MIGRATE TO 0.1.0;

ALTER PLUGIN ttl 0.1.0 ADD SERVICE example_service TO TIER default;

ALTER PLUGIN ttl 0.1.0 ENABLE;

--

ALTER PLUGIN ttl 0.1.0 DISABLE;

DROP PLUGIN ttl 0.1.0

--

select count(*) from profiles;
```

## Подключение

```
psql -h localhost -p 55432 -U admin -d T0psecre
```

## WEB UI

http://localhost:18301

User: admin
Password: T0psecret

## Plugins

```
make plugins-build
```
