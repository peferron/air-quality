# Air Quality

Reads air quality measurements from a Dylos DC1100 PRO via Serial / RS-232 and pipes it through Redis running on a Raspberry Pi, InfluxDB and Grafana to finally display in a live dashboard.

Written in Rust. No tests because it's a toy project. :)

For more details, see:

* [Measuring air quality in my home (part 1)](https://peferron.com/2017/03/23/measuring-air-quality-part-1/).
* [Measuring air quality in my home (part 2)](https://peferron.com/2017/04/07/measuring-air-quality-part-2/).
* [Live dashboard](https://air-quality.peferron.com/grafana/dashboard/db/air-quality?orgId=2&from=now-7d&to=now&refresh=5m&theme=light).

[![Dashboard](screenshot.png?raw=true)](https://air-quality.peferron.com/grafana/dashboard/db/air-quality?orgId=2&from=now-7d&to=now&refresh=5m&theme=light)
