local amble = {}

function amble.init()
	amble.ffi = require("amble_ffi")

	amble.ffi.init("./core/target/release/libamble.so")
end

local function get_lines(input_string)
	local lines = {} -- Initialize an empty table to hold the lines

	for line in string.gmatch(input_string, '[^\n]+') do
		-- Insert each line into the lines table
		table.insert(lines, line)
	end
	return lines
end

function amble.on_buf_write()
	local content_raw = vim.api.nvim_buf_get_lines(0, 0, -1, false)

	local content = table.concat(content_raw, '\n')
	local processed = get_lines(amble.ffi.write_category(1, "Test Category", content))

	vim.api.nvim_buf_set_lines(
		0, 0, -1, false,
		processed
	)
end

vim.api.nvim_create_user_command("AmbleWrite", amble.on_buf_write, {})

return amble
