local amble = require('amble')
local pickers = require('telescope.pickers')
local finders = require('telescope.finders')
local previewers = require('telescope.previewers')
local actions = require('telescope.actions')
local action_state = require('telescope.actions.state')
local conf = require("telescope.config").values
local entry_display = require("telescope.pickers.entry_display")

local amble_picker = function(opts)
	opts = opts or {}
	pickers.new(opts, {
		prompt_title = "Category: ",
		attach_mappings = function(prompt_bufnr, map)
			actions.select_default:replace(function()
				actions.close(prompt_bufnr)
				local selection = action_state.get_selected_entry()
				amble.open_category(selection.value)
			end)
			return true
		end,
		sorter = conf.generic_sorter(opts),
		finder = finders.new_table {
			results = amble.get_categories(),
			entry_maker = function(entry)
				local displayer = entry_display.create({
					separator = ": ",
					items = {
						{ width = 50 },
						{ width = 8 },
						{ remaining = true },
					},
				})

				local make_display = function()
					return displayer({
						tostring(entry.name),
						tostring(entry.id),
					})
				end

				return {
					value = entry,
					display = make_display,
					ordinal = entry.name .. " : " .. tostring(entry.id)
				}
			end
		},
		previewer = previewers.new_buffer_previewer({
			define_preview = function(self, entry, status)
				vim.api.nvim_buf_set_lines(
					self.state.bufnr, 0, -1, false,
					vim.split(amble.get_category_content(entry.value.id), "\n")
				)
			end,
			title = 'Amble Preview'
		})
	}):find()
end

return require('telescope').register_extension {
		setup = function() end,
		exports = {
				picker = amble_picker
		}
}
