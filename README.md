# LuaU <-> JSON

Sending data from one application to another can be difficult when dealing with special LuaU types. This project aims to fix that issue by properly serialising special types like `Color3`.

You can find the Lua module [here](./module.lua) which allows you to serialise and deserialise the data while the main Rust project aims to serve as an example of how to do it with another application.

[Rust Examples](./tests/)

[Lua Examples](./lua_tests/)

## Notices

- These models are not extensive and do not contain all of the impls
- While it uses JSON as the "middleman", support for other file formats could be added (like binary formats).
