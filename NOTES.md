# Blind spot log

- Byte-consumption ownership: skipped `*` in both main.rs and findArraySize (double-skip → array size parsed as 0); earlier, findArraySize left its trailing `\r\n` unconsumed. Underlying gap: no fixed convention for which function consumes which bytes of the input.
- Bounds check ANDed into the terminator-match condition in both findArraySize and parseString, so running out of input means "never break" instead of "stop parsing." Underlying gap: treats the bounds guard as part of the content check rather than as its own loop-exit condition.
- Debugged the same boundary condition by cycling operators (`<`, `>`, `==`) across three edits without tracing an input through the loop. Underlying gap: mutating code before simulating it by hand; conditions treated as slot machines.
- Wrote Rust constructs by shape without semantics: `match` on a bool with `True`/`False` arms, bare `Result` as an if-body statement, `Err(False)`. Underlying gap: type vs. value distinction; `match` earns its keep destructuring enums, not dispatching on bools.
