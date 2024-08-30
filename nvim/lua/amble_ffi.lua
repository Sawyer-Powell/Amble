local ffi = require("ffi")

ffi.cdef [[
typedef struct {
	int id;
	const char* name;
	const char* content;
} TopLevelCategory;
char* write_category(TopLevelCategory* category);
]]

local amble_ffi = {}

function amble_ffi.init(libamble_path)
	amble_ffi.interface = ffi.load(libamble_path)
end

function amble_ffi.write_category(id, name, content)
	local category = ffi.new(
		"TopLevelCategory",
		{
			id = id,
			name = name,
			content = content,
		}
	)
	local result_ptr = amble_ffi.interface.write_category(category)
	return ffi.string(result_ptr)
end

return amble_ffi
