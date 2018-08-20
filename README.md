## Wake Interval for Futures 0.3
Wrap a future in a task that is set to wake the future after a set
time interval.

### Run
The ```wake_interval``` example demonstrates this with a toy future
that is set to be ```Poll:Ready(())``` after 3 calls to ```poll()```.

```bash
cargo run --example wake_interval
```

### Output
```
future will return after 3 polls
poll 1
poll 2
poll 3
```
