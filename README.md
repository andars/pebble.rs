## Pebble.rs

Pebble.rs is a crate that allows [rust](http://www.rust-lang.org) to be used to
develop Pebble applications. It is compatible with SDK 3.0 and is known to work
on the aplite platform. It *should* also work on basalt, but I don't currently
have one to test. 

To see an example application using Pebble.rs, look
[here](https://github.com/andars/pebble.rs-template).

*This is very much a work in progress and currently has very limited
capabilities.* **But it does work ;)**

## NOTE:

As of now, this library requires a patch to the pebble sdk. When bundling the app,
the pebble sdk parses the output of `readelf` with the assumption that there will
never be more than 100 sections. 

Some example lines from `readelf`:
```
  [ 1] .header           PROGBITS        00000000 008000 000082 00   A  0   0  1
  [11] .data             PROGBITS        00000e60 008e60 0000b4 00  WA  0   0  4
```

Note that the close bracket is at index 5 in both cases. As a result, in the process of parsing, 
the pebble sdk uses `line[6:]`. When the second numbers get to 3 digits, however, the close
bracket is no longer in position 6, which breaks the bundling process.

Paths are relative to root of pebble sdk.

In file
`Pebble/.waf-1.7.11-cf7e1a867a97a34ac27942862f927bc2/waflib/extras/inject_metadata.py`
(not sure if that hash changes, if it does update it to yours):

Change
```
line = line[6:]
```
on line 71 to 
```
if not ']' in line:
    continue
line=line[line.index(']')+1:]
```
