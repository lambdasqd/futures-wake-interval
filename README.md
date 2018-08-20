## Wake Interval for Futures 0.3
Wrap a future so it is periodically polled after a set
time interval.

#### Motivation
Ideally you would never need to use this but it may be useful
for custom futures that do not have an 'event' that will trigger
a wake. For example, say you are waiting for a server to start up
before establishing a client connection to it.

### Run
Requires nightly to run. Tested on ```1.30.0-nightly (b2028828d 2018-08-16)```.

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
