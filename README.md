# Cron Expression Parser

Goal of this project was to write cli application, which parses a
cron string and expands each field to show the times it will run

Fields, that are taken into consideration are:

```shell
minute, hour, day_of_month, month, day_of_week, command
```

## Run

To run this example, you have to have installed Rust (Ideally with Cargo package manager)

Assuming you have cargo, to run application just run:

```shell
cd parser_rust

cargo run "*/15 0 1,15 * 1-5 /usr/bin/find"
```

## Input

The cron string will be passed to application as a single argument.

```shell
~$ cron-expr-parser "*/15 0 1,15 * 1-5 /usr/bin/find"
```

## Output

Application should yield following output:

```shell
minute:         0 15 30 35
hour            0
day of month    1 15
month:          1 2 3 4 5 6 7 8 9 10 11 12
command:        /usr/bin/find
```

### Note

Program should handle all possible cron strings
