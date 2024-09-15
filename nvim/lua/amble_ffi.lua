local ffi = require("ffi")

ffi.cdef [[
typedef struct {
	int id;
	const char* name;
	const char* content;
} TopLevelCategory;

char* write_category(TopLevelCategory* category);

typedef struct {
	int id;
	const char* name;
} TopLevelCategoriesResult;

typedef struct {
	const TopLevelCategoriesResult* categories;
	int length;
} TopLevelCategoryResults;

TopLevelCategoryResults get_top_level_categories();
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

function amble_ffi.get_top_level_categories()
	local results = amble_ffi.interface.get_top_level_categories();
	local categories = {}

	for i = 0, results.length - 1 do
		print(results.categories[i].name)
		categories[i+1] = results.categories[i].value
	end

	return categories
end

return amble_ffi
