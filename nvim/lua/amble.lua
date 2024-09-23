local amble = {}

function amble.init()
	amble.ffi = require("amble_ffi")

	amble.ffi.init("./core/target/release/libamble.so")
end

function amble.on_buf_write(category)
	local content_raw = vim.api.nvim_buf_get_lines(0, 0, -1, false)

	local content = table.concat(content_raw, '\n')
	local output = amble.ffi.write_category(category.id, category.name, content)

	vim.api.nvim_buf_set_lines(
		0, 0, -1, false,
		vim.split(output.content, "\n")
	)
end

function amble.is_file_open(path)
	local buffers = vim.api.nvim_list_bufs()

	for _, buf in ipairs(buffers) do
		if vim.api.nvim_buf_is_loaded(buf) and vim.api.nvim_buf_get_name(buf) == path then
			return true
		end
	end

	return false
end

function amble.open_category(category)
	local content = amble.ffi.get_category_content(category.id)

	local filename = category.name .. '.org'
	local path = '/tmp/' .. filename

	if not amble.is_file_open(path) then
		local file = io.open(path, 'w')

		if file == nil then
			print("File was nil")
			return
		end

		file:write(content)
		file:close()
	end

	vim.cmd('edit ' .. path)

	vim.api.nvim_create_autocmd('BufWritePost', {
		pattern = path,
		callback = function()
			amble.on_buf_write(category)
		end
	})
end

function amble.select_category()
	local categories = amble.ffi.get_top_level_categories()

	local choices = {}

	for index, value in ipairs(categories) do
		table.insert(choices, index .. ". " .. value.name)
	end

	local choice = vim.fn.inputlist(choices)

	amble.open_category(categories[choice])
end

function amble.new_category()
	local category_name = vim.fn.input("Category name: ")
	local category_id = amble.ffi.write_category(-1, category_name, "").id

	local category = {
		name = category_name,
		id = category_id
	}

	amble.open_category(category)
end

vim.api.nvim_create_user_command("AmbleList", amble.select_category, {})
vim.api.nvim_create_user_command("AmbleNew", amble.new_category, {})

return amble
