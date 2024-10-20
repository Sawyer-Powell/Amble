package.path = package.path .. ";amble/?.lua"
local amble = {};
amble.ffi = require("amble_ffi")
amble.ffi.init("../core/target/release/libamble.so")

local category = amble.ffi.write_category(-1, "Greetings", [[
* TODO Need to talk to Sarah
* FOLD
** FROM
*** "TODO" title
** INTO
*** title
]])

print(category.content)
