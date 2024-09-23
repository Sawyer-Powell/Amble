local amble = {}

amble.ffi = require("amble_ffi")
amble.ffi.init("../../core/target/release/libamble.so")

print('writing greetings')
amble.ffi.write_category(-1, "Greetings", [[
* Hello!
This is some content

** Here is some more content]])

print('writing farewell')
local id = amble.ffi.write_category(1, "Farewell", [[
* Goodbye!
This is some content
** Here is some more content
Farewell farewell!]]).id

print(id)

local category = amble.ffi.get_top_level_categories()[1]

print(category.name)

print(amble.ffi.get_category_content(category.id))
