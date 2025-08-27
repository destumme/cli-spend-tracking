# DB

## Scripts
DB Setup scripts, run from sqlite directly


```
$ touch spending.db
$ sqlite3
sqlite> ATTACH DATABASE 'spending.db' AS spending;
sqlite> .read scripts/setup.sql
```