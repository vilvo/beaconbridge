# Beaconbridge

## Summary

Configurable daemon to bridge Bluetooth Low Energy Beacons, like RuuviTag, in rust.

## Status

[ ] Bridge decoded data to some observability platform to view on dashboard.  
[ ] Decode RuuviTag data format.  
[ ] Refactor configuration to its' own module. Validate config values.  
[X] Event based Bluetooth beacon scanning and raw data printing to daemon standard out.  
[X] Daemon redirects stdout and stderr to beaconbridge.[out|err] under configured working directory.  
[X] Configuration file for daemon name, working directory, user and group - tested on OSX.

## License

MIT
