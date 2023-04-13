# CTF Rankings API

This provides a simple RESTful API to get player rankings of a [Minetest CTF](https://github.com/MT-CTF/capturetheflag).

### Usage

```
./ctf-rankings-api /path/to/config.toml`
```

### The config file syntax

The config file syntax is [TOML](https://toml.io). Entries:

 - `redis_addr`(`string`): [Address to a Redis server](https://docs.rs/redis/0.23.0/redis/struct.Client.html)
 - `listen_addr`(`string`): [An address](https://docs.rs/tide/latest/tide/listener/trait.ToListener.html) for the HTTP server to listen on

### License

This software is a free and open source software under AGPL-2.0 as published by the free software foundation. You should have received a copy in LICENSE.
