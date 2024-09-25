vim.o.runtimepath = vim.o.runtimepath .. ",./nvim/third-party/telescope.nvim"
vim.o.runtimepath = vim.o.runtimepath .. ",./nvim/third-party/plenary.nvim"
vim.o.runtimepath = vim.o.runtimepath .. ",./"
vim.g.mapleader = " "

vim.api.nvim_set_keymap('n', '<leader>q', ':q!<CR>', {})

require('amble').setup()
vim.api.nvim_set_keymap('n', '<leader>an', ':AmbleNew<CR>', {})

local builtin = require('telescope.builtin')
require('telescope').load_extension('amble')

vim.keymap.set('n', '<leader>ff', builtin.find_files, { desc = 'Telescope find files' })
vim.keymap.set('n', '<leader>fg', builtin.live_grep, { desc = 'Telescope live grep' })
vim.keymap.set('n', '<leader>fb', builtin.buffers, { desc = 'Telescope buffers' })
vim.keymap.set('n', '<leader>fh', builtin.help_tags, { desc = 'Telescope help tags' })
vim.keymap.set('n', '<leader>af', require('telescope').extensions.amble.picker, { desc = 'Telescope help tags' })
