local ffi = require("ffi")

ffi.cdef [[
typedef struct {
	int64_t id;
	const char* name;
	const char* content;
} TopLevelCategory;

TopLevelCategory write_category(TopLevelCategory* category);

typedef struct {
	int64_t id;
	const char* name;
} TopLevelCategoryResult;

typedef struct {
	const TopLevelCategoryResult* categories;
	int64_t length;
} TopLevelCategoryResults;

TopLevelCategoryResults get_top_level_categories();
char* get_category_content(int64_t id);
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

	local top_level_category = amble_ffi.interface.write_category(category)

	return {
		id = tonumber(top_level_category.id),
		content = ffi.string(top_level_category.content),
		name = ffi.string(top_level_category.name)
	}
end

function amble_ffi.get_top_level_categories()
	local results = amble_ffi.interface.get_top_level_categories();
	local categories = {}

	for i = 0, tonumber(results.length) - 1 do
		categories[i+1] = {
			name = ffi.string(results.categories[i].name),
			id = tonumber(results.categories[i].id)
		}
	end

	return categories
end

function amble_ffi.get_category_content(id)
	local content_ptr = amble_ffi.interface.get_category_content(id)
	return ffi.string(content_ptr)
end

return amble_ffi
