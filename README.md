# ascents-rs :climbing_man:

## Overview

Command line app for logging rock climbing ascents. Ascents are logged in a SQLite database table with the following fields:

1. `route`: Name of the route.
2. `grade`: Grade of the route in terms of the Yosemite Decimal System (YDS).
3. `crag`: Name of the crag, or general climbing area, where the route is located.
4. `date`: Date of first recorded ascent.

An ascent is defined as a redpoint ascent (i.e., successfully leading the route with no falls or takes).

## Example Usage

```
$ ascents --help
Usage: ascents [-h] {init,log,drop,analyze} database
```

Create a new ascent DB:

```
$ ascents init ascent.db
Initializing ascent database: ascent.db
Successfully initialized database
```

Log an ascent:

```
$ ascents log ascent.db
Enter the name of the route: Slither
Enter the grade of the route: 5.7
Enter the name of the crag where the route is located: Reimers Ranch
Enter the date of the ascent in YYYY-MM-DD format: 2022-06-27
Ascent to be logged: Slither 5.7 at Reimers Ranch on 2022-06-27
Log the above ascent (y/n)? y
Successfully logged the above ascent
```

Confirm its existence:

```
$ sqlite3 --markdown ascent.db 'select * from ascents'
```

|  route  | grade |     crag      |    date    |
|---------|-------|---------------|------------|
| Slither | 5.7   | Reimers Ranch | 2022-06-27 |
