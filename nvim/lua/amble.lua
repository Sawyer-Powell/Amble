local amble = {}

function amble.init()
	amble.ffi = require("amble_ffi")

	amble.ffi.init("./core/target/release/libamble.so")
end

function amble.on_buf_write()
	local content_raw = vim.api.nvim_buf_get_lines(0, 0, -1, false)

	local content = table.concat(content_raw, '\n')
	local output = amble.ffi.write_category(1, "Test Category", content)

	vim.api.nvim_buf_set_lines(
		0, 0, -1, false,
		vim.split(output, "\n")
	)
end

function amble.get_top_level_categories()
	local categories = amble.ffi.get_top_level_categories()
end

vim.api.nvim_create_user_command("AmbleWrite", amble.on_buf_write, {})
vim.api.nvim_create_user_command("AmbleList", amble.get_top_level_categories, {})

return amble
